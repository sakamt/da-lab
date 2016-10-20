
extern crate ndarray;
extern crate ndarray_odeint;
extern crate ndarray_linalg;
extern crate rustc_serialize;
extern crate aics_da;

use std::fs;
use ndarray::prelude::*;
use ndarray_linalg::*;
use ndarray_odeint::*;
use aics_da::*;
use aics_da::ensemble::V;

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
    let mut xs = ensemble::replica(&x, 0.01, setting.k);

    // observation settings
    let h = Array::<f64, _>::eye(3);
    let r = setting.r * Array::<f64, _>::eye(3);

    // time-evolutions operators
    let u = |x| teo(setting.dt, setting.tau, x);
    let f = |xs| da::forcast(&u, xs);

    // data assimilation
    println!("time,dev,std");
    for t in 0..setting.count {
        x = u(x);
        xs = f(xs);
        xs = da::enkf(xs, &x, &h, &r);
        let (xm, p) = ensemble::stat2(&xs);
        println!("{:.05},{:.05},{:.05}",
                 t as f64 * setting.dt,
                 (xm - &x).norm(),
                 p.trace().unwrap().sqrt());
        if t % setting.save_count == 0 {
            let tt = t / setting.save_count;
            let xs_fname = format!("data/xs{:05}.msg", tt);
            io::save_as_msg(&xs, xs_fname);
            let x_fname = format!("data/x{:05}.msg", tt);
            io::save_as_msg(&x, x_fname);
        }
    }
}
