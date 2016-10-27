
extern crate ndarray;
extern crate ndarray_odeint;
extern crate ndarray_linalg;
extern crate num_traits;
extern crate rustc_serialize;
extern crate aics_da;

use std::fs;
use ndarray::prelude::*;
use ndarray_linalg::*;
use ndarray_odeint::*;
use num_traits::float::Float;
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


    // observation settings
    let h = Array::<f64, _>::eye(3);
    let r = setting.r * Array::<f64, _>::eye(3);
    let rs = r.clone().ssqrt().unwrap();

    // time-evolutions operators
    let u = |x| teo(setting.dt, setting.tau, x);
    let f = |xs| da::forcast(&u, xs);

    // init data
    let mut x = arr1(&[1., 0., 0.]);
    for _ in 0..setting.count / 2 {
        x = u(x);
    }
    let mut xs = ensemble::replica(&x, 0.01, setting.k);

    // data assimilation
    println!("time,x,y,z,dev,std,sk0");
    for t in 0..setting.count {
        x = u(x);
        xs = f(xs);
        if t % setting.save_count == 0 {
            let tt = t / setting.save_count;
            let xs_fname = format!("data/pre{:05}.msg", tt);
            io::save_as_msg(&xs, xs_fname);
        }
        let y = h.dot(&x) + rs.dot(&da::random(3));
        let sk0 = ensemble::skewness(&xs)[0];
        xs = da::enkf(xs, &y, &h, &r);
        if t % setting.save_count == 0 {
            let tt = t / setting.save_count;
            let xs_fname = format!("data/post{:05}.msg", tt);
            io::save_as_msg(&xs, xs_fname);
        }
        let (xm, p) = ensemble::stat2(&xs);
        println!("{:.05},{:.05},{:.05},{:.05},{:.05},{:.05},{:.05e}",
                 (t * setting.tau) as f64 * setting.dt,
                 x[0],
                 x[1],
                 x[2],
                 (xm - &x).norm().sqrt() / 3.0.sqrt(),
                 p.trace().unwrap().sqrt(),
                 sk0);
    }
}
