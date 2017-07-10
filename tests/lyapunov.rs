
extern crate ndarray;
extern crate ndarray_rand;
extern crate ndarray_linalg;
extern crate aics_da;

use aics_da::lyapunov::*;
use ndarray::*;
use ndarray_linalg::*;
use ndarray_rand::*;

#[test]
fn hstack_success() {
    let x = arr1(&[1.0, 0.0, 0.0]);
    let y = arr1(&[1.0, 0.0, 0.0]);
    let v = vec![x, y];
    let s = hstack(&v).unwrap();
    println!("s = {:?}", s);
    s.assert_allclose_l2(&arr2(&[[1.0, 1.0], [0.0, 0.0], [0.0, 0.0]]), 1e-9);
}

#[should_panic]
#[test]
fn hstack_fails() {
    let x = arr1(&[1.0, 0.0, 0.0]);
    let y = arr1(&[1.0, 0.0]);
    let v = vec![x, y];
    let s = hstack(&v).unwrap();
    println!("s = {:?}", s);
}

#[test]
fn jacobi_cached_linear() {
    let n = 3;
    let a = generate::random((n, n));
    let x = generate::random(n);
    let f = |y| a.dot(&y);
    let b = jacobi_cached(&f, &x, 1e-7);
    b.assert_allclose_l2(&a, 1e-5);
}

#[test]
fn jacobi_dot_v() {
    let n = 3;
    let a = generate::random((n, n));
    let x = generate::random(n);
    let f = |y| a.dot(&y);
    let fx = f.jacobian(&x, 1e-7).dot(&x);
    fx.assert_allclose_l2(&a.dot(&x), 1e-5);
}

#[test]
fn jacobi_dot_m() {
    let a = generate::random((3, 3));
    let x = generate::random(3);
    let xs = generate::random((3, 2));
    let f = |y| a.dot(&y);
    let fx = f.jacobian(&x, 1e-7).dot(&xs);
    fx.assert_allclose_l2(&a.dot(&xs), 1e-5);
}
