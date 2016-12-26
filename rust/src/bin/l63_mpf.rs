#![allow(non_snake_case)]

extern crate ndarray;
extern crate rustc_serialize;
extern crate aics_da;
extern crate docopt;
extern crate pbr;

use docopt::Docopt;
use ndarray::prelude::*;
use aics_da::*;
use aics_da::types::V;
use pbr::ProgressBar;

const USAGE: &'static str = "
MPF for Lorenz63 model

Usage:
  l63_mpf <setting> <observation> <init> <output>
";

#[derive(RustcDecodable)]
struct Args {
    arg_setting: String,
    arg_observation: String,
    arg_init: String,
    arg_output: String,
}

#[derive(RustcDecodable)]
struct Setting {
    k: usize,
    tau: usize,
    save_count: usize,
    dt: f64,
    r: f64,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    println!("[Arguments]");
    println!("- executable   : l63_mpf");
    println!("- setting JSON : {}", args.arg_setting);
    println!("- initial state: {}", args.arg_init);
    println!("- observations : {}", args.arg_observation);
    println!("- output       : {}", args.arg_output);
    std::fs::create_dir_all(&args.arg_output).unwrap();
    let setting: Setting = io::read_json(&args.arg_setting);
    let x0: V = io::load_msg(&args.arg_init);
    let obs: Vec<V> = io::load_msg(&args.arg_observation);
    let T = obs.len();
    let N = obs[0].len();
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
    let obs_op = observation::ObsOperator::new(h, rs);

    let xs0 = da::replica(&x0, setting.r.sqrt(), setting.k);
    let analyzer = mpf::MPF::new(obs_op, 3);
    let teo = |x| l63::teo(setting.dt, setting.tau, x);
    let mpf = obs.iter().scan(xs0, |xs, y| Some(da::iterate(&teo, &analyzer, xs, y)));

    let mut pb = ProgressBar::new(T as u64);
    for (t, (xs_b, xs_a)) in mpf.enumerate() {
        pb.inc();
        if t % setting.save_count == 0 {
            let tt = t / setting.save_count;
            io::save_msg(&xs_b, &format!("{}/b{:05}.msg", args.arg_output, tt));
            io::save_msg(&xs_a, &format!("{}/a{:05}.msg", args.arg_output, tt));
        }
    }
    pb.finish_print("done!\n");
}