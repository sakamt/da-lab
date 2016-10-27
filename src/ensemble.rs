
extern crate rand;
extern crate ndarray;
extern crate ndarray_linalg;
extern crate ndarray_rand;

use self::ndarray::prelude::*;
use self::rand::distributions::*;
use self::ndarray_rand::RandomExt;
use einsum;

pub type V = Array<f64, Ix>;
pub type M = Array<f64, (Ix, Ix)>;
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

pub fn skewness(xs: &Ensemble) -> V {
    let k = xs.len() as f64;
    let n = xs[0].len();
    let mut mu = Array::zeros(n);
    for x in xs.iter() {
        mu += x;
    }
    mu /= k;

    let mut m3 = Array::zeros(n);
    for x in xs.iter() {
        let dx = x - &mu;
        m3 = m3 + dx.mapv(|a| a * a * a);
    }
    m3 * (k / ((k - 1.0) * (k - 2.0)))
}
