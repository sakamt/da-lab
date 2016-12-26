
use ndarray_linalg::prelude::*;
use std::mem;
use std::marker::PhantomData;

use observation::*;
use ensemble::*;
use stat::*;

pub fn rmse(mean: &V, truth: &V) -> f64 {
    let n = mean.len() as f64;
    (mean - truth).norm() / n.sqrt()
}

/// execute Ensemble Kalman filter (EnKF)
pub struct EnKF<'a, TEO, Iter>
    where TEO: Fn(V) -> V,
          Iter: Iterator<Item = &'a V>
{
    h: M,
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
            rs: rs,
            states: states,
            teo: teo,
            obs_iter: obs_iter,
            phantom: PhantomData,
        }
    }

    /// execute analysis step
    fn analysis(&self, xs: Ensemble, y: &V) -> Ensemble {
        let ys = xs.iter().map(|x| self.h.dot(x) + noise(&self.rs)).collect();
        let v = covar(&ys, &ys);
        let u = covar(&xs, &ys);
        let k = u.dot(&v.inv().unwrap());
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
        let xs_b = self.teo.forecast(xs_a.clone());
        let xs_b_pre = mem::replace(&mut self.states, xs_b);
        Some((xs_b_pre, xs_a))
    }
}
