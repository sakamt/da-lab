
extern crate ndarray;
extern crate ndarray_odeint;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;
extern crate itertools;

use docopt::Docopt;
use ndarray::prelude::*;
use ndarray_odeint::*;
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

fn teo(dt: f64, step: usize, mut x: V) -> V {
    let l = |y| lorenz63(10., 28., 8.0 / 3.0, y);
    let u = |y| rk4(&l, dt, y);
    for _ in 0..step {
        x = u(x);
    }
    x
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    println!("[Arguments]");
    println!("- executable   : l63_genobs");
    println!("- setting JSON : {}", args.arg_setting);
    println!("- initial state: {}", args.arg_init);
    let setting: Setting = io::read_json(&args.arg_setting);
    let output = "obs.msg";
    println!("[Settings]");
    println!("- dt    : {}", setting.dt);
    println!("- tau   : {}", setting.tau);
    println!("- r     : {}", setting.r);
    println!("- count : {}", setting.count);
    println!("- output: {}", output);

    let rs = setting.r.sqrt() * Array::<f64, _>::eye(3);
    let x0: V = io::load_msg(&args.arg_init);
    let obs: Vec<V> = iterate(x0, |x| teo(setting.dt, setting.tau, x.clone()))
        .map(|x| x + da::noise(&rs))
        .take(setting.count)
        .collect();
    io::save_msg(&obs, output);
}
