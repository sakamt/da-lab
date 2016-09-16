
extern crate ndarray;
extern crate ndarray_odeint;
extern crate rmp_serialize;
extern crate rustc_serialize;

use ndarray::prelude::*;
use rustc_serialize::Encodable;
use rmp_serialize::Encoder;
use std::fs::File;

fn save_as_msg<T: Encodable>(val: &T, filename: &str) -> Result<(), &'static str> {
    let mut buf = File::create(filename).ok().unwrap();
    let mut enc = Encoder::new(&mut buf);
    val.encode(&mut enc).map_err(|_| "Faild to encode")
}

fn main() {
    let l = |y| ndarray_odeint::lorenz63(10., 28., 8.0 / 3.0, y);
    let teo = |y| ndarray_odeint::rk4(&l, 0.01, y);

    let mut ts = vec![];
    let time = 100000; // 100k
    let mut x = arr1(&[1.0, 0.0, 0.0]);
    for _ in 0..time {
        x = teo(x);
        ts.push(x.clone());
    }
    match save_as_msg(&ts, "ts.msg") {
        Ok(()) => println!("Saved."),
        Err(s) => println!("Error: {}", s),
    }
}
