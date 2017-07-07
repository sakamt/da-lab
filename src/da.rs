use ndarray::*;
use ndarray_rand::RandomExt;
use rand::distributions::*;
use std::mem;

use super::types::*;

#[derive(RustcDecodable, Clone, Copy)]
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
where
    TEO: Fn(V) -> V,
{
    fn forecast(&self, xs: Ensemble) -> Ensemble {
        xs.into_iter().map(self).collect()
    }
}

pub trait EnsembleAnalyzer {
    fn analysis(&self, xs: Ensemble, obs: &V) -> Ensemble;
}

pub fn iterate<F, A>(forecaster: &F, analyzer: &A, mut state: &mut Ensemble, obs: &V) -> (Ensemble, Ensemble)
where
    F: EnsembleForecaster + ?Sized,
    A: EnsembleAnalyzer + ?Sized,
{
    let xs_a = analyzer.analysis(state.clone(), obs);
    let xs_f = forecaster.forecast(xs_a.clone());
    let xs_f_pre: Ensemble = mem::replace(&mut state, xs_f);
    (xs_f_pre, xs_a)
}

pub fn series<'a, F, A>(
    forecaster: &'a F,
    analyzer: &'a A,
    xs0: Ensemble,
    obs: &'a Vec<V>,
) -> Box<Iterator<Item = (Ensemble, Ensemble)> + 'a>
where
    F: EnsembleForecaster + ?Sized,
    A: EnsembleAnalyzer + ?Sized,
{
    Box::new(obs.iter().scan(xs0, move |xs, y| {
        Some(iterate(forecaster, analyzer, xs, y))
    }))
}
