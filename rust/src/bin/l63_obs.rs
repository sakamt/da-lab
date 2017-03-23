
extern crate ndarray;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;
extern crate itertools;

use docopt::Docopt;
use ndarray::prelude::*;
use aics_da::*;
use aics_da::types::V;

const USAGE: &'static str = "
Generate observations of Lorenz63 model

Usage:
  l63_obs <setting> <truth> --output=<output>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_setting: String,
    arg_truth: String,
    flag_output: Option<String>,
}

#[derive(RustcDecodable)]
struct Setting {
    dt: f64,
    r: f64,
    tau: usize,
    count: usize,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    println!("[Arguments]");
    println!("- executable   : l63_obs");
    println!("- setting JSON : {}", args.arg_setting);
    println!("- truth series : {}", args.arg_truth);
    let setting: Setting = io::read_json(&args.arg_setting);
    let truth: Vec<V> = io::load_msg(&args.arg_truth);
    let output = args.flag_output.unwrap_or("obs.msg".to_string());
    println!("[Settings]");
    println!("- dt         : {}", setting.dt);
    println!("- tau        : {}", setting.tau);
    println!("- r          : {}", setting.r);
    println!("- count      : {}", setting.count);
    println!("[Outputs]");
    println!("- observation: {}", output);

    let rs = setting.r.sqrt() * Array::<f64, _>::eye(3);
    let obs: Vec<V> = truth.iter()
        .map(|x| x + &observation::noise(&rs))
        .collect();
    io::save_msg(&obs, &output);
}
