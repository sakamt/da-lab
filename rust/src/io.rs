
use rustc_serialize::{Encodable, Decodable, json};
use rmp_serialize::{Encoder, Decoder};
use std::fs::File;
use std::io::Read;
use std::string::String;

use super::types::{Truth, Observation};
use super::settings;


pub fn save_msg<T: Encodable>(val: &T, filename: &str) {
    let mut buf = File::create(filename).ok().unwrap();
    let mut enc = Encoder::new(&mut buf);
    val.encode(&mut enc).unwrap();
}

pub fn load_msg<T: Decodable>(filename: &str) -> T {
    let mut buf = File::open(filename).unwrap();
    let mut dec = Decoder::new(&mut buf);
    Decodable::decode(&mut dec).unwrap()
}

pub fn read_json<Contents: Decodable>(filename: &str) -> Contents {
    let mut f = File::open(filename).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    json::decode(s.as_str()).unwrap()
}

pub trait SeriesStorage {
    type Key;
    fn save_truth(&self, &settings::Truth, &Truth) -> Self::Key;
    fn load_truth(&self, Self::Key) -> (settings::Truth, Truth);
    fn save_observation(&self, &settings::Observation, truth_key: Self::Key, &Observation) -> Self::Key;
    fn load_observation(&self, Self::Key) -> (settings::Observation, Self::Key, Observation);
}
