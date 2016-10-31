
extern crate rand;
extern crate ndarray;
extern crate ndarray_linalg;
extern crate ndarray_rand;

use self::ndarray_linalg::*;
use self::ndarray::prelude::*;
use self::rand::distributions::*;
use self::ndarray_rand::RandomExt;
use ensemble::*;
use std::mem;
use std::marker::PhantomData;

pub fn forcast(teo: &Fn(V) -> V, xs: Ensemble) -> Ensemble {
    xs.into_iter().map(teo).collect()
}

pub fn rmse(mean: &V, truth: &V) -> f64 {
    let n = mean.len() as f64;
    ((mean - truth).norm() / n).sqrt()
}

pub fn noise(rs: &M) -> V {
    let (n, _) = rs.size();
    let dist = Normal::new(0., 1.0);
    let d = Array::random(n, dist);
    rs.dot(&d)
}

/// execute Ensemble Kalman filter (EnKF)
pub struct EnKF<'a, TEO, Iter>
    where TEO: Fn(V) -> V,
          Iter: Iterator<Item = &'a V>
{
    h: M,
    r: M,
    rs: M,
    states: Ensemble,
    teo: TEO,
    obs_iter: Iter,
    phantom: PhantomData<&'a V>,
}

impl<'a, TEO, Iter> EnKF<'a, TEO, Iter>
    where TEO: Fn(V) -> V,
          Iter: Iterator<Item = &'a V>
{
    pub fn new(h: M, rs: M, states: Ensemble, teo: TEO, obs_iter: Iter) -> Self {
        EnKF {
            h: h,
            r: rs.dot(&rs),
            rs: rs,
            states: states,
            teo: teo,
            obs_iter: obs_iter,
            phantom: PhantomData,
        }
    }

    /// execute analysis step
    fn analysis(&self, xs: Ensemble, y: &V) -> Ensemble {
        let (_, p) = stat2(&xs);
        let v = &self.h.dot(&p).dot(&self.h.t()) + &self.r;
        let vinv = v.inv().unwrap();
        let k = p.dot(&self.h.t()).dot(&vinv);
        xs.into_iter()
            .map(|x| {
                let err = y - &self.h.dot(&x) + noise(&self.rs);
                x + k.dot(&err)
            })
            .collect()
    }
}

impl<'a, TEO, Iter> Iterator for EnKF<'a, TEO, Iter>
    where TEO: Fn(V) -> V,
          Iter: Iterator<Item = &'a V>
{
    type Item = (Ensemble, Ensemble);
    fn next(&mut self) -> Option<Self::Item> {
        let y = match self.obs_iter.next() {
            Some(y) => y,
            None => return None,
        };
        let xs_a = self.analysis(self.states.clone(), y);
        let xs_b = forcast(&self.teo, xs_a.clone());
        let xs_b_pre = mem::replace(&mut self.states, xs_b);
        Some((xs_b_pre, xs_a))
    }
}
