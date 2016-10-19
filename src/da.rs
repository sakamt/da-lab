
extern crate ndarray;
extern crate ndarray_linalg;

use self::ndarray::prelude::*;
use self::ndarray_linalg::*;
use einsum;

pub type V = Array<f64, Ix>;
pub type M = Array<f64, (Ix, Ix)>;
pub type Ensemble = Vec<V>;

pub fn forcast(teo: &Fn(V) -> V, xs: Ensemble) -> Ensemble {
    xs.into_iter().map(teo).collect()
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

pub fn enkf(xs: Ensemble, y: &V, h: &M, r: &M) -> Ensemble {
    let (_, p) = stat2(&xs);
    let v = h.dot(&p).dot(&h.t()) + r;
    let vinv = v.inv().unwrap();
    let k = p.dot(&h.t()).dot(&vinv);
    xs.into_iter()
        .map(|x| {
            let err = y + &h.dot(&x);
            x + k.dot(&err)
        })
        .collect()
}
