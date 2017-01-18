
use ndarray::prelude::*;
use ndarray_odeint::*;
use itertools::iterate;

use super::types::V;
use super::da;

pub fn teo(dt: f64, step: usize, mut x: V) -> V {
    let p = lorenz63::Parameter::default();
    let l = |y| lorenz63::f(p, y);
    let u = |y| explicit::rk4(&l, dt, y);
    for _ in 0..step {
        x = u(x);
    }
    x
}

pub fn generate_truth(setting: &da::Setting) -> Vec<V> {
    let t = setting.tau * setting.count;
    let x0: V = arr1(&[1.0, 0.0, 0.0]);
    iterate(x0, |x| teo(setting.dt, 1, x.clone()))
        .skip(t / 10)
        .take(t)
        .collect()
}
