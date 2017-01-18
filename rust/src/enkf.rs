
use ndarray_linalg::prelude::*;

use types::*;
use stat::*;
use observation::*;
use da::EnsembleAnalyzer;

/// Ensemble Kalman Filter with perturbed observation implementation
#[derive(Clone, Debug)]
pub struct EnKF {
    obs: LinearNormal,
}

impl EnKF {
    pub fn new(obs: LinearNormal) -> Self {
        EnKF { obs: obs }
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
