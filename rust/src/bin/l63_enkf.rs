#![allow(non_snake_case)]

extern crate ndarray;
extern crate ndarray_odeint;
extern crate rustc_serialize;
extern crate aics_da;
extern crate docopt;
extern crate pbr;

use docopt::Docopt;
use ndarray::prelude::*;
use ndarray_odeint::*;
use aics_da::*;
use aics_da::ensemble::V;
use pbr::ProgressBar;

#[derive(RustcDecodable)]
struct Setting {
    k: usize,
    tau: usize,
    save_count: usize,
    dt: f64,
    r: f64,
}

const USAGE: &'static str = "
EnKF for Lorenz63 model

Usage:
  l63_enkf <setting> <observation> <init> <output>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_setting: String,
    arg_observation: String,
    arg_init: String,
    arg_output: String,
}

fn teo(setting: &Setting, mut x: V) -> V {
    let dt = setting.dt;
    let step = setting.tau;
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
    println!("- executable   : l63_enkf");
    println!("- setting JSON : {}", args.arg_setting);
    println!("- initial state: {}", args.arg_init);
    println!("- observations : {}", args.arg_observation);
    println!("- output       : {}", args.arg_output);
    std::fs::create_dir_all(&args.arg_output).unwrap();
    let setting: Setting = io::read_json(&args.arg_setting);
    let x0: V = io::load_msg(&args.arg_init);
    let obs: Array<f64, Ix2> = io::load_msg(&args.arg_observation);
    let T = obs.shape()[0];
    let N = obs.shape()[1];
    let duration = (T * setting.tau) as f64 * setting.dt;
    assert_eq!(N, 3);
    println!("[Settings]");
    println!("- dt            : {}", setting.dt);
    println!("- tau           : {}", setting.tau);
    println!("- ensemble size : {}", setting.k);
    println!("- initial spread: {}", setting.r);
    println!("- steps         : {}", T);
    println!("- duration      : {}", duration);
    println!("- save count    : {}", setting.save_count);

    let h = Array::<f64, _>::eye(3);
    let rs = setting.r.sqrt() * Array::<f64, _>::eye(3);

    let y_tl: Vec<V> = obs.axis_iter(Axis(0)).map(|x| x.to_owned()).collect();
    let xs = ensemble::replica(&x0, setting.r.sqrt(), setting.k);
    let enkf = da::EnKF::new(h, rs, xs, |x| teo(&setting, x), y_tl.iter());

    let mut pb = ProgressBar::new(T as u64);
    for (t, (xs_b, xs_a)) in enkf.enumerate() {
        pb.inc();
        if t % setting.save_count == 0 {
            let tt = t / setting.save_count;
            io::save_msg(&xs_b, &format!("{}/b{:05}.msg", args.arg_output, tt));
            io::save_msg(&xs_a, &format!("{}/a{:05}.msg", args.arg_output, tt));
        }
    }
    pb.finish_print("done!\n");
}
