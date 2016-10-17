
extern crate ndarray;
extern crate ndarray_odeint;
extern crate aics_da;

use ndarray::prelude::*;
use std::fs;

type V = Array<f64, Ix>;
type Ensemble = Vec<V>;

fn teo(dt: f64, step: u32, mut x: V) -> V {
    let l = |y| ndarray_odeint::lorenz63(10., 28., 8.0 / 3.0, y);
    let u = |y| ndarray_odeint::rk4(&l, dt, y);
    for _ in 0..step {
        x = u(x);
    }
    x
}

fn forcast(xs: Ensemble, dt: f64, step: u32) -> Ensemble {
    xs.into_iter().map(|y| teo(dt, step, y)).collect()
}

fn main() {
    let mut xs = vec![Array::range(1., 4., 1.); 5];
    fs::create_dir_all("data").unwrap();
    for t in 0..1000 {
        xs = forcast(xs, 0.01, 10);
        let fname = format!("data/{:04}.msg", t);
        aics_da::save_as_msg(&xs, fname);
    }
}
