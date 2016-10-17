
extern crate ndarray;
extern crate aics_da;

use ndarray::prelude::*;
use std::fs;
use aics_da::*;

fn main() {
    let mut xs = vec![Array::range(1., 4., 1.); 5];
    fs::create_dir_all("data").unwrap();
    for t in 0..1000 {
        xs = da::forcast(xs, 0.01, 10);
        let fname = format!("data/{:04}.msg", t);
        io::save_as_msg(&xs, fname);
    }
}
