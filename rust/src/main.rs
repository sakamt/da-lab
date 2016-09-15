
extern crate rmp_serialize;
extern crate rustc_serialize;

use rustc_serialize::Encodable;
use rmp_serialize::Encoder;
use std::fs::File;

fn main() {
    let val = (42u8, "the Answer");
    let mut buf = File::create("foo.msg").ok().unwrap();
    let mut enc = Encoder::new(&mut buf);
    let res = val.encode(&mut enc);
    println!("{:?}", res);
}
