
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

fn teo(setting: &Setting, mut x: V) -> V {
    let dt = setting.dt;
    let step = setting.tau;
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

    let ts = TimeSeries {
        teo: |x| teo(&setting, x),
        state: arr1(&[1.0, 0.0, 0.0]),
    };
    let x_tl: Vec<V> = ts.skip(setting.count / 2).take(setting.count).collect();
    let y_tl: Vec<V> = x_tl.iter().map(|x| da::noise(&rs) + x).collect();
    let xs = ensemble::replica(&x_tl[0], 0.01, setting.k);

    let enkf = da::EnKF::new(h, rs, xs, |x| teo(&setting, x), y_tl.iter());

    for (xs_a, xs_b) in enkf {
        //
    }
}
