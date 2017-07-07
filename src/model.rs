use ndarray::prelude::*;
use ndarray_odeint::*;

use super::da;
use super::types::V;

pub struct L63 {
    //
}

pub fn l63(x: V, setting: &da::Setting) -> V {
    x
}

pub fn teo(dt: f64, step: usize, mut x: V) -> V {
    let p = lorenz63::Parameter::default();
    let l = |y| lorenz63::f(p, y);
    let u = |y| explicit::rk4(&l, dt, y);
    for _ in 0..step {
        x = u(x);
    }
    x
}
