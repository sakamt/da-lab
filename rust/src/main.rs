
extern crate ndarray;
extern crate ndarray_odeint;
extern crate rmp_serialize;
extern crate rustc_serialize;

use ndarray::prelude::*;
use rustc_serialize::Encodable;
use rmp_serialize::Encoder;
use std::fs::File;
use std::iter::FromIterator;

fn save_as_msg<T: Encodable>(val: &T, filename: &str) -> Result<(), &'static str> {
    let mut buf = File::create(filename).ok().unwrap();
    let mut enc = Encoder::new(&mut buf);
    val.encode(&mut enc).map_err(|_| "Faild to encode")
}

type V = Array<f64, Ix>;

struct TimeSeries<T: Fn(V) -> V> {
    teo: T,
    state: V,
}

impl<T: Fn(V) -> V> Iterator for TimeSeries<T> {
    type Item = V;
    fn next(&mut self) -> Option<V> {
        let v = self.state.clone();
        self.state = (self.teo)(self.state.clone());
        Some(v)
    }
}

fn main() {
    let l = |y| ndarray_odeint::lorenz63(10., 28., 8.0 / 3.0, y);
    let teo = |y| ndarray_odeint::rk4(&l, 0.01, y);
    let ts = TimeSeries {
        teo: teo,
        state: arr1(&[1.0, 0.0, 0.0]),
    };

    let ts_vec = Vec::from_iter(ts.take(10000));
    match save_as_msg(&ts_vec, "ts.msg") {
        Ok(()) => println!("Saved."),
        Err(s) => println!("Error: {}", s),
    }
}
