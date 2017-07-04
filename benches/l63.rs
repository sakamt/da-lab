#![feature(test)]

extern crate test;
extern crate ndarray;
extern crate aics_da;

use ndarray::prelude::*;
use aics_da::*;
use aics_da::da::{EnsembleForecaster, EnsembleAnalyzer};

macro_rules! impl_bench {
    ($replica:ident, $clone:ident, $K:expr) => {
#[bench]
fn $replica(b: &mut test::Bencher) {
    let r = 0.1;
    let k = $K;
    let x = arr1(&[1.0, 0.0, 0.0]);
    b.iter(|| da::replica(&x, r, k))
}

#[bench]
fn $clone(b: &mut test::Bencher) {
    let r = 0.1;
    let k = $K;
    let x = arr1(&[1.0, 0.0, 0.0]);
    let xs = da::replica(&x, r, k);
    b.iter(|| xs.clone())
}
}} // impl_bench

impl_bench!(bench_replica_k1, bench_clone_k1, 10);
impl_bench!(bench_replica_k2, bench_clone_k2, 100);
impl_bench!(bench_replica_k3, bench_clone_k3, 1000);

macro_rules! impl_bench_forecast {
    ($func:ident, $tau:expr, $K:expr) => {
#[bench]
fn $func(b: &mut test::Bencher) {
    let dt = 0.01;
    let r = 0.1;
    let tau = $tau;
    let k = $K;
    let x = arr1(&[1.0, 0.0, 0.0]);
    let teo = |x| l63::teo(dt, tau, x);
    let xs = da::replica(&x, r, k);
    b.iter(|| teo.forecast(xs.clone()))
}
}} // impl_bench_forecast

impl_bench_forecast!(bench_forecast_t08_k1, 8, 10);
impl_bench_forecast!(bench_forecast_t25_k1, 25, 10);
impl_bench_forecast!(bench_forecast_t50_k1, 50, 10);
impl_bench_forecast!(bench_forecast_t08_k2, 8, 100);
impl_bench_forecast!(bench_forecast_t25_k2, 25, 100);
impl_bench_forecast!(bench_forecast_t50_k2, 50, 100);
impl_bench_forecast!(bench_forecast_t08_k3, 8, 1000);
impl_bench_forecast!(bench_forecast_t25_k3, 25, 1000);
impl_bench_forecast!(bench_forecast_t50_k3, 50, 1000);

macro_rules! impl_bench_analysis {
    ($enkf:ident, $mpf:ident, $K:expr) => {
#[bench]
fn $enkf(b: &mut test::Bencher) {
    let r = 0.1;
    let k = $K;
    let x = arr1(&[1.0, 0.0, 0.0]);
    let xs = da::replica(&x, r, k);
    let obs_op = observation::LinearNormal::isotropic(3, r);
    let analyzer = enkf::EnKF::new(obs_op.clone());
    b.iter(|| analyzer.analysis(xs.clone(), &x))
}

#[bench]
fn $mpf(b: &mut test::Bencher) {
    let r = 0.1;
    let k = $K;
    let x = arr1(&[1.0, 0.0, 0.0]);
    let xs = da::replica(&x, r, k);
    let obs_op = observation::LinearNormal::isotropic(3, r);
    let analyzer = mpf::MPF::new(obs_op, 3);
    b.iter(|| analyzer.analysis(xs.clone(), &x))
}
}} // impl_bench_analysis

impl_bench_analysis!(bench_enkf_k1, bench_mpf_k1, 10);
impl_bench_analysis!(bench_enkf_k2, bench_mpf_k2, 100);
impl_bench_analysis!(bench_enkf_k3, bench_mpf_k3, 1000);
