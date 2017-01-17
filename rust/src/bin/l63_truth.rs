#![allow(non_snake_case)]

extern crate ndarray;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;
extern crate itertools;
extern crate rusqlite;

use docopt::Docopt;
use ndarray::prelude::*;
use aics_da::*;
use aics_da::types::V;
use aics_da::sqlite as sql;
use itertools::iterate;

const USAGE: &'static str = "
Generate truth of Lorenz63 model

Usage:
  l63_truth <setting> <db>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_setting: String,
    arg_db: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);
    let mut conn = rusqlite::Connection::open(args.arg_db).unwrap();
    let tx = conn.transaction().unwrap();
    gen_truth(setting, &tx);
    tx.commit().unwrap();
}

fn gen_truth(setting: da::Setting, conn: &rusqlite::Connection) {
    let T = setting.tau * setting.count;
    let x0: V = arr1(&[1.0, 0.0, 0.0]);
    let truth: Vec<V> = iterate(x0, |x| l63::teo(setting.dt, 1, x.clone()))
        .skip(T / 10)
        .take(T)
        .collect();
    let tid = sql::save_truth(setting.dt,
                              T as f64 * setting.dt,
                              &truth,
                              &conn,
                              &sql::util::now_str());
    println!("{}", tid);
}
