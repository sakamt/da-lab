
#[macro_use]
extern crate serde_derive;
extern crate ndarray;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;
extern crate itertools;
extern crate env_logger;
extern crate dotenv;

use aics_da::*;
use aics_da::types::V;
use docopt::Docopt;
use itertools::iterate;

const USAGE: &'static str = "
Generate truth series of Lorenz63 model

Usage:
  l63_truth <setting> <init>
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_setting: String,
    arg_init: String,
}

#[derive(RustcDecodable)]
struct Setting {
    dt: f64,
    tau: usize,
    count: usize,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().unwrap();
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());
    println!("[Arguments]");
    println!("- executable   : l63_genobs");
    println!("- setting JSON : {}", args.arg_setting);
    println!("- initial state: {}", args.arg_init);
    let setting: Setting = io::read_json(&args.arg_setting);
    let output = "truth.msg";
    println!("[Settings]");
    println!("- dt         : {}", setting.dt);
    println!("- tau        : {}", setting.tau);
    println!("- count      : {}", setting.count);
    println!("[Outputs]");
    println!("- truth      : {}", output);

    let x0: V = io::load_msg(&args.arg_init);
    let truth: Vec<V> = iterate(x0, |x| l63::teo(setting.dt, setting.tau, x.clone()))
        .take(setting.count)
        .collect();
    io::save_msg(&truth, output);
}
