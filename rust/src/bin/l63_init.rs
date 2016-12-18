
extern crate ndarray;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;

use docopt::Docopt;
use ndarray::prelude::*;
use aics_da::*;

const USAGE: &'static str = "
Generate inital state of Lorenz63 model

Usage:
  l63_init <setting>
";

#[derive(RustcDecodable)]
struct Args {
    arg_setting: String,
}

#[derive(RustcDecodable)]
struct Setting {
    dt: f64,
    init_count: usize,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    println!("[Arguments]");
    println!("- executable  : l63_init");
    println!("- setting JSON: {}", args.arg_setting);
    let setting: Setting = io::read_json(&args.arg_setting);
    let output = "init.msg";
    println!("[Settings]");
    println!("- dt    : {}", setting.dt);
    println!("- count : {}", setting.init_count);
    println!("- output: {}", output);

    let u = |y| l63::teo(setting.dt, 1, y);

    // generate initial state
    let mut x = arr1(&[1.0, 0.0, 0.0]);
    for _ in 0..setting.init_count {
        x = u(x);
    }
    io::save_msg(&x, output);
}
