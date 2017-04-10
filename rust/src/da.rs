
use std::mem;
use rand::distributions::*;
use ndarray::prelude::*;
use ndarray_rand::RandomExt;

use super::types::*;
use super::stat;

#[derive(RustcDecodable)]
pub struct Setting {
    pub k: usize,
    pub tau: usize,
    pub count: usize,
    pub everyn: Option<usize>,
    pub dt: f64,
    pub r: f64,
    pub rho: Option<f64>,
}

pub fn replica(x: &V, r: f64, k: usize) -> Ensemble {
    let n = x.len();
    let dist = Normal::new(0.0, 1.0);
    (0..k).map(|_| r * Array::random(n, dist) + x).collect()
}

pub trait EnsembleForecaster {
    fn forecast(&self, xs: Ensemble) -> Ensemble;
}

impl<TEO> EnsembleForecaster for TEO
    where TEO: Fn(V) -> V
{
    fn forecast(&self, xs: Ensemble) -> Ensemble {
        xs.into_iter().map(self).collect()
    }
}

pub trait EnsembleAnalyzer {
    fn analysis(&self, xs: Ensemble, obs: &V) -> Ensemble;
}

pub fn iterate<F, A>(forecaster: &F, analyzer: &A, mut state: &mut Ensemble, obs: &V) -> (Ensemble, Ensemble)
    where F: EnsembleForecaster + ?Sized,
          A: EnsembleAnalyzer + ?Sized
{
    let xs_a = analyzer.analysis(state.clone(), obs);
    let xs_f = forecaster.forecast(xs_a.clone());
    let xs_f_pre: Ensemble = mem::replace(&mut state, xs_f);
    (xs_f_pre, xs_a)
}

pub fn iterate_bias_collect<F, A>(forecaster: &F,
                                  analyzer: &A,
                                  mut xs: &mut Ensemble,
                                  y: &V,
                                  t: &V)
                                  -> (Ensemble, Ensemble)
    where F: EnsembleForecaster + ?Sized,
          A: EnsembleAnalyzer + ?Sized
{
    let xs_a = analyzer.analysis(xs.clone(), y);
    let xs_f = forecaster.forecast(remove_bias(xs_a.clone(), t));
    let xs_f_pre: Ensemble = mem::replace(xs, xs_f);
    (xs_f_pre, xs_a)
}

fn remove_bias(mut xs: Ensemble, truth: &V) -> Ensemble {
    let dev = stat::mean(&xs) - truth;
    for x in xs.iter_mut() {
        *x = &*x - &dev;
    }
    xs
}
