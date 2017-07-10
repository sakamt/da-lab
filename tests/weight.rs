
extern crate rand;
extern crate ndarray;
#[macro_use]
extern crate ndarray_linalg;
extern crate aics_da;

use ndarray::Array;
use ndarray_linalg::*;
use rand::distributions::IndependentSample;
use std::f64::consts::E;

use aics_da::weight::*;

fn close_max(a: &Vec<f64>, b: &Vec<f64>, atol: f64) {
    let a = Array::from_vec(a.clone());
    let b = Array::from_vec(b.clone());
    ndarray_linalg::assert::close_max(&a, &b, atol).unwrap();
}

#[test]
fn lw2w2lw() {
    let n = 10;
    let lw: LogWeight = vec![0.0; n].into();
    let w: Weight = lw.clone().into();
    let lw2: LogWeight = w.into();
    close_max(lw2.get_raw_logweight(), lw.get_raw_logweight(), 1e-7);
}

#[test]
fn w2lw2w() {
    let n = 10;
    let w = Weight::random(n);
    let lw: LogWeight = w.clone().into();
    let w2: Weight = lw.into();
    close_max(w2.get_raw_weight(), w.get_raw_weight(), 1e-7);
}

#[test]
fn logweight_to_weight() {
    let n = 10;
    let lw: LogWeight = vec![0.0; n].into();
    let w: Weight = lw.into();
    let truth = vec![1.0 / n as f64; n];
    close_max(w.get_raw_weight(), &truth, 1e-7);
}

#[test]
fn logweight() {
    let lw: LogWeight = vec![0.0, 1.0].into();
    let w: Weight = lw.into();
    let raw = w.get_raw_weight();
    assert_rclose!(raw[1] / raw[0], E, 1e-7);
}

#[test]
fn dist() {
    let n = 3;
    let w = Weight::random(n);
    let mut count: Vec<u64> = vec![0; n];
    let mut rng = rand::thread_rng();
    let dist = w.to_dist();
    let k: usize = 10000;
    for _ in 0..k {
        let idx = dist.ind_sample(&mut rng);
        count[idx] += 1;
    }
    let w_eff: Vec<f64> = count.into_iter().map(|x| x as f64 / k as f64).collect();
    println!("weight = {:?}", w);
    println!("observed = {:?}", w_eff);
    close_max(&w_eff, &w.get_raw_weight(), 0.5);
}
