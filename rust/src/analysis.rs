
use ndarray::prelude::*;
use ndarray_linalg::prelude::*;
use ensemble::*;

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
    (m2 * (k / (k - 1.0)), m3 * ((k * k) / ((k - 1.0) * (k - 2.0))), m4)
}

pub fn pca_kstat4(xs: &Ensemble) -> (V, V, V) {
    kstat4(&pca(xs))
}
