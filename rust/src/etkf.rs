
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
        let r_inv = obs.corr().inv().unwrap();
        ETKF {
            obs: obs,
            r_inv: r_inv,
            rho: rho,
        }
    }
}

impl EnsembleAnalyzer for ETKF {
    fn analysis(&self, xs: Ensemble, y: &V) -> Ensemble {
        let n = y.len();
        let m = xs.len();
        let xm = mean(&xs);
        let dxs = hstack(&xs.into_iter().map(|x| x - &xm).collect::<Vec<_>>()).unwrap();
        let dys = hstack(&dxs.axis_iter(Axis(0)).map(|x| self.obs.eval(&x.to_owned())).collect::<Vec<_>>()).unwrap();
        let dy = y - &self.obs.eval(&xm);
        let p_inv = (m as f64 - 1.0) * Array::eye(n) + dys.t().dot(&self.r_inv).dot(&dys);
        let p = p_inv.inv().unwrap();
        let w = p.dot(&dys.t().dot(&self.r_inv.dot(&dy)));
        let t = p.ssqrt().unwrap();
        let xm_a = dxs.t().dot(&w);
        let dxs_a = t.dot(&dxs);
        dxs_a.axis_iter(Axis(0)).map(|dx| &dx + &xm_a).collect()
    }
}
