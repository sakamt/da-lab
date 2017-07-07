use itertools::iterate;
use ndarray::*;
use ndarray_odeint::*;

use super::da;
use super::types::V;

pub fn teo(dt: f64, step: usize, mut x: V) -> V {
    let l = model::lorenz63::Lorenz63::default();
    let u = explicit::rk4(l, dt);
    for _ in 0..step {
        u.iterate(&mut x);
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
