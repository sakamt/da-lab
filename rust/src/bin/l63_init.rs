#![allow(non_snake_case)]

extern crate ndarray;
extern crate ndarray_odeint;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;

use docopt::Docopt;
use ndarray::prelude::*;
use ndarray_odeint::*;
use aics_da::*;

#[derive(RustcDecodable)]
struct Setting {
    dt: f64,
    init_count: usize,
}

const USAGE: &'static str = "
Generate inital state of Lorenz63 model

Usage:
  l63_init <setting>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_setting: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    println!("[Arguments]");
    println!("- setting JSON: {}", args.arg_setting);
    let setting: Setting = io::read_json(&args.arg_setting);
    let output = "init.msg";
    println!("[Settings]");
    println!("- dt    : {}", setting.dt);
    println!("- count : {}", setting.init_count);
    println!("- output: {}", output);

    // ODE solver
    let l = |y| lorenz63(10., 28., 8.0 / 3.0, y);
    let u = |y| rk4(&l, setting.dt, y);

    // generate initial state
    let mut x = arr1(&[1.0, 0.0, 0.0]);
    for _ in 0..setting.init_count {
        x = u(x);
    }
    io::save_msg(&x, output);
}
