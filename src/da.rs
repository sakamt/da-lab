use ndarray::*;
use ndarray_linalg::*;
use ndarray_odeint::*;

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
    for<'a> &'a TEO: TimeEvolution<OwnedRepr<f64>, Ix1>,
{
    fn forecast(&self, mut xs: Ensemble) -> Ensemble {
        for x in xs.iter_mut() {
            self.iterate(x);
        }
        xs
    }
}

pub trait EnsembleAnalyzer {
    fn analysis(&self, xs: Ensemble, obs: &V) -> Ensemble;
}

pub fn select_analyzer(setting: &Setting) -> Box<EnsembleAnalyzer> {
    match setting.da.as_str() {
        "etkf" => Box::new(etkf::ETKF::new(setting)),
        "enkf" => Box::new(enkf::EnKF::new(setting)),
        "mpf" => Box::new(mpf::MPF::new(setting)),
        _ => panic!("unsupported method"),
    }
}
