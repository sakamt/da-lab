
use ndarray::*;
use ndarray_linalg::prelude::*;
use ndarray_linalg::util::hstack;

use super::types::*;
use super::stat::*;
use super::observation::*;
use super::da::EnsembleAnalyzer;

/// Ensemble Kalman Filter with perturbed observation implementation
#[derive(Clone, Debug)]
pub struct ETKF {
    obs: LinearNormal,
    r_inv: M,
    rho: f64,
}

impl ETKF {
    pub fn new(obs: LinearNormal, rho: f64) -> Self {
        ETKF {
            obs: obs,
            r_inv: obs.corr().inv().unwrap(),
            rho: rho,
        }
    }
}

impl EnsembleAnalyzer for ETKF {
    fn analysis(&self, xs: Ensemble, y: &V) -> Ensemble {
        let n = y.len();
        let m = xs.len();
        let xm = mean(&xs);
        let dxs = hstack(xs.into_iter().map(|x| x - &xm).collect());
        let dys = Array::from_iter(dxs.axis_iter(Axis(0)).map(|x| self.obs.eval(&x.to_owned())));
        // let dy = y - &self.obs.eval(&xm);
        // let p_inv = (m as f64 - 1.0) * Array::eye(n);
        // for ((i, j), val) in p_inv.indexed_iter_mut() {
        //     *val += dys[i].dot(&(self.r_inv.dot(&dys[j])));
        // }
        // let p = p_inv.inv().unwrap();
        // let r_inv_dy = self.r_inv.dot(&dy);
        // let w = p.dot(&Array::from_iter(dys.iter().map(|dys_a| dys_a.dot(&r_inv_dy))));
        // let t = p.ssqrt().unwrap();
        // let xm_a = w.iter().zip(dxs.iter()).map(|(weight, dx)| dx * *weight).fold(Array::zeros(n), |a, b| a + b);
    }
}
