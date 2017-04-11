
extern crate num_traits;
extern crate ndarray_linalg;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;

use num_traits::float::Float;
use ndarray_linalg::prelude::*;
use docopt::Docopt;
use aics_da::*;
use aics_da::types::V;

const USAGE: &'static str = "
Calculate RMSE of Lorenz63 model

Usage:
  l63_rmse <setting> <truth> <datadir>
  l63_rmse (-h | --help)

Options:
  -h --help   Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_datadir: String,
    arg_truth: String,
    arg_setting: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    println!("[Arguments]");
    println!("- executable  : l63_rmse");
    println!("- setting JSON: {}", args.arg_setting);
    println!("- truth       : {}", args.arg_truth);
    println!("- data dir    : {}", args.arg_datadir);
    let setting: da::Setting = io::read_json(&args.arg_setting);
    let truth: Vec<V> = io::load_msg(&args.arg_truth);
    println!("[Settings]");
    let everyn = setting.everyn.unwrap_or(1);
    println!("- everyn: {}", everyn);
    let output = "rmse.msg";
    println!("[Outputs]");
    println!("- RMSE: {}", output);

    let rmse: Vec<f64> = truth.iter()
        .enumerate()
        .filter_map(|(t, x)| {
            if t % everyn != 0 {
                return None;
            }
            let xs_a: Vec<V> = io::load_msg(&format!("{}/a{:05}.msg", args.arg_datadir, t / everyn));
            let xm = stat::mean(&xs_a);
            Some((x - &xm).norm() / 3.0.sqrt())
        })
        .collect();
    println!("Mean RMSE = {}",
             rmse.iter().sum::<f64>() / rmse.len() as f64);
    io::save_msg(&rmse, output);
}
