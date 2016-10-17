
extern crate ndarray;
extern crate rustc_serialize;
extern crate aics_da;

use std::fs;
use ndarray::prelude::*;
use aics_da::*;

#[derive(RustcDecodable)]
struct Setting {
    count: usize,
    k: usize,
    tau: usize,
    dt: f64,
}

fn main() {
    let setting: Setting = io::read_json("setting.json");
    let mut xs = vec![Array::range(1., 4., 1.); setting.k ];
    fs::create_dir_all("data").unwrap();
    for t in 0..setting.count {
        xs = da::forcast(xs, setting.dt, 1);
        let fname = format!("data/{:05}.msg", t);
        if t % setting.tau == 0 {
            // TODO analysis
            io::save_as_msg(&xs, fname);
        }
    }
}
