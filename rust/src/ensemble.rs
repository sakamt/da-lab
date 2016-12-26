
use ndarray::prelude::*;
use rand::distributions::*;
use ndarray_rand::RandomExt;

pub type V = Array<f64, Ix1>;
pub type M = Array<f64, Ix2>;
pub type Ensemble = Vec<V>;

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
