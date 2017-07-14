use rmp_serialize::{Decoder, Encoder};
use rustc_serialize::{Decodable, Encodable, json};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::string::String;

pub fn save_msg<T: Encodable>(val: &T, filename: &str) {
    let f = File::create(filename).ok().unwrap();
    let mut buf = BufWriter::new(f);
    let mut enc = Encoder::new(&mut buf);
    val.encode(&mut enc).unwrap();
}

pub fn load_msg<T: Decodable>(filename: &str) -> T {
    let f = File::open(filename).unwrap();
    let mut buf = BufReader::new(f);
    let mut dec = Decoder::new(&mut buf);
    Decodable::decode(&mut dec).unwrap()
}

pub fn read_json<Contents: Decodable>(filename: &str) -> Contents {
    let f = File::open(filename).unwrap();
    let mut buf = BufReader::new(f);
    let mut s = String::new();
    buf.read_to_string(&mut s).unwrap();
    json::decode(s.as_str()).unwrap()
}
