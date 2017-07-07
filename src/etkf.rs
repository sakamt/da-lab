use ndarray::*;
use ndarray_linalg::*;

use super::da::{EnsembleAnalyzer, Setting};
use super::observation::*;
use super::stat::*;
use super::types::*;

/// Ensemble Kalman Filter with perturbed observation implementation
#[derive(Clone, Debug)]
pub struct ETKF {
    obs: LinearNormal,
    r_inv: M,
    rho_inv: f64,
}

impl ETKF {
    pub fn new(obs: LinearNormal, rho: f64) -> Self {
        let r_inv = obs.corr().inv().unwrap();
        ETKF {
            obs: obs,
            r_inv: r_inv,
            rho_inv: 1.0 / rho,
        }
    }
}

impl From<Setting> for ETKF {
    fn from(setting: Setting) -> Self {
        let rho = setting.rho.unwrap_or(1.0);
        ETKF::new(LinearNormal::isotropic(3, setting.r), rho)
    }
}

impl EnsembleAnalyzer for ETKF {
    fn analysis(&self, xs: Ensemble, y: &V) -> Ensemble {
        let m = xs.len();
        let mm1 = m as f64 - 1.0;
        let xm = mean(&xs);
        let dxs = hstack(&xs.into_iter().map(|x| x - &xm).collect::<Vec<_>>()).unwrap();
        let dys = hstack(&dxs.axis_iter(Axis(1))
            .map(|x| self.obs.eval(&x.to_owned()))
            .collect::<Vec<_>>()).unwrap();
        let dy = y - &self.obs.eval(&xm);
        let p_inv = mm1 * self.rho_inv * Array::eye(m) + dys.t().dot(&self.r_inv).dot(&dys);
        let p = p_inv.inv().unwrap();
        let w = p.dot(&dys.t().dot(&self.r_inv.dot(&dy)));
        let t = (mm1 * p).ssqrt(UPLO::Upper).unwrap();
        let xm_a = xm + dxs.dot(&w);
        let dxs_a = dxs.dot(&t);
        dxs_a.axis_iter(Axis(1)).map(|dx| &dx + &xm_a).collect()
    }
}
