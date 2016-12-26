
extern crate num_traits;
extern crate ndarray_linalg;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;
extern crate pbr;

use num_traits::float::Float;
use docopt::Docopt;
use ndarray_linalg::prelude::*;
use aics_da::*;
use aics_da::ensemble::V;
use pbr::ProgressBar;

const USAGE: &'static str = "
Calculate RMSE of Lorenz63 model

Usage:
  l63_rmse <truth> <datadir>
  l63_rmse (-h | --help)

Options:
  -h --help  Show this screen
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_datadir: String,
    arg_truth: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    println!("[Arguments]");
    println!("- executable: l63_rmse");
    println!("- data dir  : {}", args.arg_datadir);
    println!("- truth     : {}", args.arg_truth);
    let truth: Vec<V> = io::load_msg(&args.arg_truth);
    let count = truth.len();
    println!("[Settings]");
    println!("- count: {}", count);
    let output = "rmse.msg";
    println!("[Outputs]");
    println!("- RMSE: {}", output);

    let mut pb = ProgressBar::new(count as u64);
    let rmse: Vec<f64> = truth.iter()
        .enumerate()
        .map(|(t, x)| {
            pb.inc();
            let xs_a: Vec<V> = io::load_msg(&format!("{}/a{:05}.msg", args.arg_datadir, t));
            let xm = stat::mean(&xs_a);
            (x - &xm).norm() / 3.0.sqrt()
        })
        .collect();
    println!("Mean RMSE = {}",
             rmse.iter().sum::<f64>() / rmse.len() as f64);
    io::save_msg(&rmse, output);

}
