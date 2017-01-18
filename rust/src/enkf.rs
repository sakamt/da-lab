
use ndarray_linalg::prelude::*;

use types::*;
use stat::*;
use observation::*;
use da::EnsembleAnalyzer;

/// Ensemble Kalman Filter with perturbed observation implementation
#[derive(Clone, Debug)]
pub struct EnKF {
    obs: ObsOperator,
}

impl EnKF {
    pub fn new(obs: ObsOperator) -> Self {
        EnKF { obs: obs }
    }
}

impl EnsembleAnalyzer for EnKF {
    fn analysis(&self, xs: Ensemble, y: &V) -> Ensemble {
        let ys = xs.iter().map(|x| self.obs.evaluate(x)).collect();
        let v = covar(&ys, &ys);
        let u = covar(&xs, &ys);
        let k = u.dot(&v.inv().unwrap());
        xs.into_iter()
            .map(|x| {
                let err = y - &self.obs.evaluate(&x);
                x + k.dot(&err)
            })
            .collect()
    }
}
