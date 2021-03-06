use ndarray_linalg::*;

use super::da::{EnsembleAnalyzer, Setting};
use super::observation::*;
use super::stat::*;
use super::types::*;

/// Ensemble Kalman Filter with perturbed observation implementation
#[derive(Clone, Debug)]
pub struct EnKF {
    obs: LinearNormal,
}

impl EnKF {
    pub fn new(setting: &Setting) -> Self {
        EnKF { obs: LinearNormal::isotropic(3, setting.r) }
    }
}

impl EnsembleAnalyzer for EnKF {
    fn analysis(&self, xs: Ensemble, y: &V) -> Ensemble {
        let ys = xs.iter().map(|x| self.obs.noisy_eval(x)).collect();
        let v = covar(&ys, &ys);
        let u = covar(&xs, &ys);
        let k = u.dot(&v.inv().unwrap());
        xs.into_iter()
            .map(|x| {
                let err = y - &self.obs.noisy_eval(&x);
                x + k.dot(&err)
            })
            .collect()
    }
}
