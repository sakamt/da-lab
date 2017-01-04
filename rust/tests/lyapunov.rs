
extern crate ndarray;
extern crate ndarray_rand;
extern crate ndarray_numtest;
extern crate aics_da;

use ndarray::prelude::*;
use ndarray_rand::*;
use ndarray_numtest::prelude::*;
use aics_da::lyapunov::*;

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
    let dist = RealNormal::<f64>::new(0.0, 1.0);
    let a = Array::random((n, n), dist);
    let x = Array::random(n, dist);
    let f = |y| a.dot(&y);
    let b = jacobi_cached(&f, &x, 1e-7);
    b.assert_allclose_l2(&a, 1e-5);
}
