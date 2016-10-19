
extern crate rand;
extern crate ndarray;
extern crate ndarray_odeint;
extern crate ndarray_rand;
extern crate rustc_serialize;
extern crate aics_da;

use std::fs;
use rand::distributions::*;
use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use ndarray_odeint::*;
use aics_da::*;
use aics_da::da::V;

#[derive(RustcDecodable)]
struct Setting {
    count: usize,
    k: usize,
    tau: usize,
    save_count: usize,
    dt: f64,
    r: f64,
}

pub fn teo(dt: f64, step: usize, mut x: V) -> V {
    let l = |y| lorenz63(10., 28., 8.0 / 3.0, y);
    let u = |y| rk4(&l, dt, y);
    for _ in 0..step {
        x = u(x);
    }
    x
}

fn main() {
    let setting: Setting = io::read_json("setting.json");
    fs::create_dir_all("data").unwrap();

    // init data
    let mut x = arr1(&[1., 0., 0.]);
    let mut xs: Vec<da::V> = Vec::new();
    let dist = Normal::new(0.0, 1.0);
    for _ in 0..setting.k {
        xs.push(0.01 * Array::random(3, dist) + &x);
    }

    // observation settings
    let h = Array::<f64, _>::eye(3);
    let r = setting.r * Array::<f64, _>::eye(3);

    // time-evolutions operators
    let u = |x| teo(setting.dt, setting.tau, x);
    let f = |xs| da::forcast(&u, xs);

    // data assimilation
    for t in 0..setting.count {
        x = u(x);
        xs = f(xs);
        xs = da::enkf(xs, &x, &h, &r);
        if t % setting.save_count == 0 {
            let tt = t / setting.save_count;
            let xs_fname = format!("data/xs{:05}.msg", tt);
            io::save_as_msg(&xs, xs_fname);
            let x_fname = format!("data/x{:05}.msg", tt);
            io::save_as_msg(&x, x_fname);
        }
    }
}
