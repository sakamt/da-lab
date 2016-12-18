
extern crate ndarray;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;
extern crate itertools;

use docopt::Docopt;
use ndarray::prelude::*;
use aics_da::*;
use aics_da::ensemble::V;
use itertools::iterate;

const USAGE: &'static str = "
Generate observations of Lorenz63 model

Usage:
  l63_genobs <setting> <init>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_setting: String,
    arg_init: String,
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
    println!("- executable   : l63_genobs");
    println!("- setting JSON : {}", args.arg_setting);
    println!("- initial state: {}", args.arg_init);
    let setting: Setting = io::read_json(&args.arg_setting);
    let truth_output = "truth.msg";
    let obs_output = "obs.msg";
    println!("[Settings]");
    println!("- dt         : {}", setting.dt);
    println!("- tau        : {}", setting.tau);
    println!("- r          : {}", setting.r);
    println!("- count      : {}", setting.count);
    println!("[Outputs]");
    println!("- truth      : {}", truth_output);
    println!("- observation: {}", obs_output);

    let rs = setting.r.sqrt() * Array::<f64, _>::eye(3);
    let x0: V = io::load_msg(&args.arg_init);
    let truth: Vec<V> = iterate(x0, |x| l63::teo(setting.dt, setting.tau, x.clone()))
        .take(setting.count)
        .collect();
    let obs: Vec<V> = truth.iter()
        .map(|x| x + &da::noise(&rs))
        .collect();
    io::save_msg(&truth, truth_output);
    io::save_msg(&obs, obs_output);
}
