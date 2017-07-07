use ndarray::prelude::*;
use ndarray_linalg::prelude::*;

use super::linalg::outer;
use super::observation::*;
use super::types::*;

pub struct Stat {
    pub rmse_f: f64,
    pub rmse_a: f64,
    pub std_f: f64,
    pub std_a: f64,
    pub bias: f64,
}

impl Stat {
    pub fn eval<Obs>(obs: &Obs, xs_f: &Ensemble, xs_a: &Ensemble, x: &V, y: &V) -> Stat
    where
        Obs: ObservationOperator + WeightEvaluator,
    {
        let (xm_f, pf) = stat2(xs_f);
        let (xm_a, pa) = stat2(xs_a);
        let w = obs.weight(xs_f, y);
        let xm_mpf = w.mean(&xs_f);
        Stat {
            rmse_f: rmse(&xm_f, x),
            rmse_a: rmse(&xm_a, x),
            std_f: pf.trace().unwrap().sqrt(),
            std_a: pa.trace().unwrap().sqrt(),
            bias: (xm_a - xm_mpf).norm(),
        }
    }
}

pub fn rmse(mean: &V, truth: &V) -> f64 {
    let n = mean.len() as f64;
    (mean - truth).norm() / n.sqrt()
}

pub fn mean(xs: &Ensemble) -> V {
    let k = xs.len();
    let n = xs[0].len();
    let mut v = Array::zeros(n);
    for x in xs.iter() {
        v = v + x;
    }
    v / k as f64
}

pub fn covar(xs: &Ensemble, ys: &Ensemble) -> M {
    let xs_m = mean(xs);
    let ys_m = mean(ys);
    let n = xs_m.len();
    let m = ys_m.len();
    let mut c = Array::zeros((n, m));
    for (x, y) in xs.iter().zip(ys.iter()) {
        let dx = x - &xs_m;
        let dy = y - &ys_m;
        c = c + outer(&dx, &dy);
    }
    c / (xs.len() - 1) as f64
}

/// mean and covariance
pub fn stat2(xs: &Ensemble) -> (V, M) {
    let k = xs.len();
    let n = xs[0].len();
    let mut v = Array::zeros(n);
    for x in xs.iter() {
        v = v + x;
    }
    v /= k as f64;
    let mut m = Array::zeros((n, n));
    for x in xs.iter() {
        let dx = x - &v;
        m = m + outer(&dx, &dx);
    }
    m /= k as f64 - 1.0;
    (v, m)
}

/// non-Gaussian bias
pub fn ng_bias(obs: &LinearNormal, xs_f: &Ensemble, y: &V) -> f64 {
    // enkf
    let (xm_f, pf) = stat2(xs_f);
    let k = obs.kalman_gain(&pf);
    let dev = y - &obs.eval(&xm_f);
    let xm_a = xm_f + k.dot(&dev);
    // mpf
    let w = obs.weight(xs_f, y);
    let xm_mpf = w.mean(&xs_f);
    (xm_a - xm_mpf).norm()
}

pub fn pca(xs: &Ensemble) -> Ensemble {
    let (xm, p) = stat2(xs);
    let (_, u) = p.eigh().unwrap();
    xs.iter().map(|x| u.t().dot(&(x - &xm))).collect()
}

/// calc unbiased estimator for cumulant of each components.
/// (i.e. ignore geometrical information)
pub fn kstat4(xs: &Ensemble) -> (V, V, V) {
    let xm = mean(xs);
    let n = xm.len();
    let mut m2 = Array::zeros(n);
    let mut m3 = Array::zeros(n);
    let mut m4 = Array::zeros(n);
    for x in xs.iter() {
        let dx = x - &xm;
        m2 = m2 + dx.mapv(|a| a * a);
        m3 = m3 + dx.mapv(|a| a * a * a);
        m4 = m4 + dx.mapv(|a| a * a * a * a);
    }
    let k = xs.len() as f64;
    m2 /= k;
    m3 /= k;
    m4 /= k;
    m4 = (k * k * ((k + 1.0) * m4 - 3.0 * (k - 1.0) * &m2 * &m2)) / ((k - 1.0) * (k - 2.0) * (k - 3.0));
    (
        m2 * (k / (k - 1.0)),
        m3 * ((k * k) / ((k - 1.0) * (k - 2.0))),
        m4,
    )
}

pub fn pca_kstat4(xs: &Ensemble) -> (V, V, V) {
    kstat4(&pca(xs))
}
