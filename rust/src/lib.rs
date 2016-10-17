
extern crate ndarray;
extern crate rmp_serialize;
extern crate rustc_serialize;

use ndarray::prelude::*;
use rustc_serialize::Encodable;
use rmp_serialize::Encoder;
use std::fs::File;

pub fn save_as_msg<T: Encodable>(val: &T, filename: &str) {
    let mut buf = File::create(filename).ok().unwrap();
    let mut enc = Encoder::new(&mut buf);
    val.encode(&mut enc).unwrap();
}
