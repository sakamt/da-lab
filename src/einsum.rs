#![allow(non_snake_case)]

use ndarray::prelude::*;

pub fn a_b__ab(va: &Array<f64, Ix1>, vb: &Array<f64, Ix1>) -> Array<f64, Ix2> {
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
