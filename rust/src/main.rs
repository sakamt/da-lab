
extern crate ndarray;
extern crate ndarray_odeint;
extern crate rmp_serialize;
extern crate rustc_serialize;

use ndarray::prelude::*;
use rustc_serialize::Encodable;
use rmp_serialize::Encoder;
use std::fs::File;

fn main() {
    let val = (42u8, "the Answer");
    let mut buf = File::create("foo.msg").ok().unwrap();
    let mut enc = Encoder::new(&mut buf);
    let res = val.encode(&mut enc);
    println!("{:?}", res);

    let mut x = arr1(&[1.0, 0.0, 0.0]);
    let l = |y| ndarray_odeint::lorenz63(10., 28., 8.0 / 3.0, y);
    let teo = |y| ndarray_odeint::rk4(&l, 0.01, y);
    for _ in 0..1000000 {
        x = teo(x);
    }
    println!("{:?}", x);
}
