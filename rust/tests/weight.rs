
extern crate aics_da;
extern crate ndarray_numtest ;

use std::f64::consts::E;
use ndarray_numtest::prelude::*;
use aics_da::weight::*;

#[test]
fn lw2w2lw() {
    let n = 10;
    let lw: LogWeight = vec![0.0; n].into();
    let w: Weight = lw.clone().into();
    let lw2: LogWeight = w.into();
    lw2.get_raw_logweight().assert_allclose(&lw.get_raw_logweight(), 1e-7);
}

#[test]
fn w2lw2w() {
    let n = 10;
    let w = Weight::random(n);
    let lw: LogWeight = w.clone().into();
    let w2: Weight = lw.into();
    w2.get_raw_weight().assert_allclose(&w.get_raw_weight(), 1e-7);
}

#[test]
fn logweight_to_weight() {
    let n = 10;
    let lw: LogWeight = vec![0.0; n].into();
    let w: Weight = lw.into();
    let truth = vec![1.0/n as f64; n];
    w.get_raw_weight().assert_allclose(&truth, 1e-7);
}

#[test]
fn logweight() {
    let lw: LogWeight = vec![0.0, 1.0].into();
    let w: Weight = lw.into();
    let raw = w.get_raw_weight();
    (raw[1] / raw[0]).assert_close(E, 1e-7);
}
