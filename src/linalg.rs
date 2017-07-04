
use ndarray::prelude::*;
use super::types::M;

pub fn outer(va: &Array<f64, Ix1>, vb: &Array<f64, Ix1>) -> Array<f64, Ix2> {
    let na = va.len();
    let nb = vb.len();
    let mut res = Array::zeros((na, nb));
    for a in 0..na {
        for b in 0..nb {
            res[(a, b)] += va[a] * vb[b];
        }
    }
    res
}

/// matrix bracket $[A]_B = B^T A B$
pub fn bracket(a: &M, b: &M) -> M {
    a.t().dot(b).t().dot(b)
}
