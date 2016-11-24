
use ndarray::prelude::*;
use ndarray_linalg::prelude::*;
use rand::distributions::*;
use ndarray_rand::RandomExt;
use einsum;

pub type V = Array<f64, Ix1>;
pub type M = Array<f64, Ix2>;
pub type Ensemble = Vec<V>;

pub fn replica(x: &V, r: f64, k: usize) -> Ensemble {
    let n = x.len();
    let dist = Normal::new(0.0, 1.0);
    (0..k).map(|_| r * Array::random(n, dist) + x).collect()
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
        c = c + einsum::a_b__ab(&dx, &dy);
    }
    c / (xs.len() - 1) as f64
}

/// calc mean and covariance matrix
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
        m = m + einsum::a_b__ab(&dx, &dx);
    }
    m /= k as f64 - 1.0;
    (v, m)
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
    m4 = (k * k * ((k + 1.0) * m4 - 3.0 * (k - 1.0) * &m2 * &m2)) /
         ((k - 1.0) * (k - 2.0) * (k - 3.0));
    (m2 * (k / (k - 1.0)), m3 * ((k * k) / ((k - 1.0) * (k - 2.0))), m4)
}

pub fn pca_kstat4(xs: &Ensemble) -> (V, V, V) {
    kstat4(&pca(xs))
}
