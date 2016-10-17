
extern crate rand;
extern crate ndarray;
extern crate ndarray_rand;
extern crate rustc_serialize;
extern crate aics_da;

use std::fs;
use rand::distributions::*;
use ndarray::prelude::*;
use ndarray_rand::RandomExt;
use aics_da::*;

#[derive(RustcDecodable)]
struct Setting {
    count: usize,
    k: usize,
    tau: usize,
    save_count: usize,
    dt: f64,
}

fn main() {
    let setting: Setting = io::read_json("setting.json");
    fs::create_dir_all("data").unwrap();

    let mut x = arr1(&[1., 0., 0.]);
    let mut xs: Vec<da::V> = Vec::new();
    let dist = Normal::new(0.0, 1.0);
    for _ in 0..setting.k {
        xs.push(0.01 * Array::random(3, dist) + &x);
    }
    for t in 0..setting.count {
        x = da::teo(setting.dt, setting.tau, x);
        xs = da::forcast(xs, setting.dt, setting.tau);
        // TODO analysis
        if t % setting.save_count == 0 {
            let tt = t / setting.save_count;
            let xs_fname = format!("data/xs{:05}.msg", tt);
            io::save_as_msg(&xs, xs_fname);
            let x_fname = format!("data/x{:05}.msg", tt);
            io::save_as_msg(&x, x_fname);
        }
    }
}
