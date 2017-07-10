
extern crate ndarray;
#[macro_use]
extern crate ndarray_linalg;
extern crate aics_da;

use aics_da::lyapunov::*;
use ndarray::*;
use ndarray_linalg::*;

#[test]
fn jacobi_cached_linear() {
    let n = 3;
    let a: Array<f64, _> = generate::random((n, n));
    let x = generate::random(n);
    let f = |y| a.dot(&y);
    let b = jacobi_cached(&f, &x, 1e-7);
    assert_close_l2!(&b, &a, 1e-5);
}

#[test]
fn jacobi_dot_v() {
    let n = 3;
    let a: Array<f64, _> = generate::random((n, n));
    let x = generate::random(n);
    let f = |y| a.dot(&y);
    let fx = f.jacobian(&x, 1e-7).dot(&x);
    assert_close_l2!(&fx, &a.dot(&x), 1e-5);
}

#[test]
fn jacobi_dot_m() {
    let a: Array<f64, _> = generate::random((3, 3));
    let x = generate::random(3);
    let xs: Array<f64, _> = generate::random((3, 2));
    let f = |y| a.dot(&y);
    let fx = f.jacobian(&x, 1e-7).dot(&xs);
    assert_close_l2!(&fx, &a.dot(&xs), 1e-5);
}
