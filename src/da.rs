use ndarray::*;
use ndarray_linalg::*;
use ndarray_odeint::*;

use super::*;
use super::types::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Setting {
    pub da: String,
    pub model: String,
    pub k: usize,
    pub tau: usize,
    pub count: usize,
    pub everyn: Option<usize>,
    pub merge: Option<usize>,
    pub replica: Option<usize>,
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

    /// Deduce `&mut` style function using `std::mem::replace` trick
    fn forecast_mut(&self, mut xs: &mut Ensemble) {
        let dummy = Vec::new();
        let xs_ = ::std::mem::replace(xs, dummy);
        let xs_ = self.forecast(xs_);
        ::std::mem::replace(xs, xs_);
    }
}

impl<TEO> EnsembleForecaster for TEO
where
    TEO: TimeEvolution<f64, Ix1>,
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

    /// Deduce `&mut` style function using `std::mem::replace` trick
    fn analysis_mut(&self, mut xs: &mut Ensemble, obs: &V) {
        let dummy = Vec::new();
        let xs_ = ::std::mem::replace(xs, dummy);
        let xs_ = self.analysis(xs_, obs);
        ::std::mem::replace(xs, xs_);
    }
}

pub fn select_analyzer(setting: &Setting) -> Box<EnsembleAnalyzer> {
    match setting.da.as_str() {
        "etkf" => Box::new(method::ETKF::new(setting)),
        "enkf" => Box::new(method::EnKF::new(setting)),
        "mpf" => Box::new(method::MPF::new(setting)),
        _ => panic!("unsupported method"),
    }
}
