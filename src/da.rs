use ndarray::*;
use ndarray_linalg::*;
use ndarray_odeint::*;

use super::*;
use super::types::*;

/// Master setting struct of all DA process
#[derive(Serialize, Deserialize, Clone)]
pub struct Setting {
    /// DA method
    pub da: String,
    /// dynamics model
    pub model: String,
    /// dt of dynamics
    pub dt: f64,
    /// size of ensemble
    pub k: usize,
    /// time interval between observations
    pub tau: usize,
    /// count of assimilation to be executed
    pub count: usize,
    /// intensity of noise
    pub r: f64,
    /// Inflation factor (default = 1.0)
    pub rho: Option<f64>,
    /// Parameter for merge-resampling
    pub merge: Option<usize>,
    /// Number of replica
    pub replica: Option<usize>,
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
