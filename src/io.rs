use serde::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn save_msg<T: Serialize>(val: &T, filename: &str) {
    let f = File::create(filename).ok().unwrap();
    let mut buf = BufWriter::new(f);
    let mut enc = ::rmp_serde::Serializer::new(&mut buf);
    val.serialize(&mut enc).unwrap();
}

pub fn load_msg<T: Deserialize>(filename: &str) -> T {
    let f = File::open(filename).unwrap();
    let mut buf = BufReader::new(f);
    let mut dec = ::rmp_serde::Deserializer::new(&mut buf);
    Deserialize::deserialize(&mut dec).unwrap()
}

pub fn read_json<Contents: Deserialize>(filename: &str) -> Contents {
    let f = File::open(filename).unwrap();
    let mut buf = BufReader::new(f);
    ::serde_json::from_reader(&mut buf).unwrap()
}
