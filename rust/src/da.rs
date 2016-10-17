
extern crate ndarray;
extern crate ndarray_odeint;

use self::ndarray::prelude::*;
use self::ndarray_odeint::*;

pub type V = Array<f64, Ix>;
pub type Ensemble = Vec<V>;

fn teo(dt: f64, step: usize, mut x: V) -> V {
    let l = |y| lorenz63(10., 28., 8.0 / 3.0, y);
    let u = |y| rk4(&l, dt, y);
    for _ in 0..step {
        x = u(x);
    }
    x
}

pub fn forcast(xs: Ensemble, dt: f64, step: usize) -> Ensemble {
    xs.into_iter().map(|y| teo(dt, step, y)).collect()
}
