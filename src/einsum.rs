#![allow(non_snake_case)]

extern crate ndarray;
use self::ndarray::prelude::*;

pub fn a_b__ab(va: &Array<f64, Ix>, vb: &Array<f64, Ix>) -> Array<f64, (Ix, Ix)> {
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
