use ndarray_linalg::*;
use std::mem;

use super::*;
use super::types::*;

#[derive(RustcDecodable, Clone)]
pub struct Setting {
    pub da: String,
    pub model: String,
    pub k: usize,
    pub tau: usize,
    pub count: usize,
    pub everyn: Option<usize>,
    pub merge: Option<usize>,
    pub dt: f64,
    pub r: f64,
    pub rho: Option<f64>,
}

pub fn replica(x: &V, r: f64, k: usize) -> Ensemble {
    let n = x.len();
    (0..k).map(|_| r * generate::random(n) + x).collect()
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

pub fn select_analyzer(setting: &Setting) -> Box<EnsembleAnalyzer> {
    match setting.da.as_str() {
        "etkf" => Box::new(etkf::ETKF::new(setting)),
        "enkf" => Box::new(enkf::EnKF::new(setting)),
        "mpf" => Box::new(mpf::MPF::new(setting)),
        _ => panic!("unsupported method"),
    }
}
