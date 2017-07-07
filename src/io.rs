
use rmp_serialize::{Decoder, Encoder};
use rustc_serialize::{Decodable, Encodable, json};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read};
use std::string::String;

use super::{settings, stat};
use super::types::{Ensemble, Observation, Truth};

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

pub trait SeriesStorage {
    type Key;
    fn save_truth(&self, &settings::Truth, &Truth) -> Self::Key;
    fn load_truth(&self, Self::Key) -> (settings::Truth, Truth);
    fn save_observation(&self, &settings::Observation, truth_key: Self::Key, &Observation) -> Self::Key;
    fn load_observation(&self, Self::Key) -> (settings::Observation, Self::Key, Observation);
}

pub trait EnsembleStorage {
    type SeriesKey;
    type Key;
    fn load_ensemble(&self, Self::Key) -> Ensemble;
    fn save_ensemble(&self, &Ensemble) -> Self::Key;
    fn commit_ensemble_series(&self, &[(f64, Self::Key, Self::Key)]) -> Self::SeriesKey;
    fn query_ensemble_series(&self, Self::SeriesKey) -> Vec<(f64, Self::Key, Self::Key)>;
}

pub trait StatStorage {
    type Key;
    fn save_stat(&self, stat: &[(f64, stat::Stat)]) -> Self::Key;
}