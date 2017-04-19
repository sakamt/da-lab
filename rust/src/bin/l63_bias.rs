
extern crate ndarray;
extern crate ndarray_linalg;
extern crate rustc_serialize;
extern crate aics_da;
extern crate docopt;
extern crate pbr;
extern crate env_logger;
extern crate dotenv;

use std::io::stderr;
use docopt::Docopt;
use aics_da::*;
use aics_da::types::*;
use ndarray_linalg::prelude::*;
use pbr::ProgressBar;

const USAGE: &'static str = "
Bias of methods for Lorenz63 model

Usage:
  l63_bias <da> <setting> <truth> <obs> [--correct] [--shake] [--progress]
  l63_bias (-h | --help)

Options:
  -h --help   Show this
  --progress  Show progress bar
  --correct   Collect bias by truth
  --shake     Shake ensemble by merge-resmpling
";

#[derive(RustcDecodable)]
struct Args {
    arg_da: String,
    arg_setting: String,
    arg_truth: String,
    arg_obs: String,
    flag_progress: bool,
    flag_correct: bool,
    flag_shake: bool,
}

fn bias(args: Args, setting: da::Setting) {
    let step = setting.dt * setting.tau as f64;

    let truth: Vec<V> = io::load_msg(&args.arg_truth);
    let obs: Vec<V> = io::load_msg(&args.arg_obs);

    let analyzer = select_analyzer(args.arg_da.trim(), setting);
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    let xs0 = da::replica(&truth[0], setting.r.sqrt(), setting.k);
    let series = bias_correct::series(&teo,
                                      &*analyzer,
                                      xs0,
                                      &obs,
                                      &truth,
                                      args.flag_correct,
                                      args.flag_shake);

    let mut pb = if args.flag_progress {
        Some(ProgressBar::on(stderr(), setting.count as u64))
    } else {
        None
    };
    println!("time,X,Y,Z,Bx,By,Bz,sb,sa");
    for (t, (tr, (xs_b, xs_a))) in truth.iter().zip(series).enumerate() {
        pb = pb.map(|mut p| {
            p.inc();
            p
        });
        let time = step * (t as f64);
        let (_, pb) = stat::stat2(&xs_b);
        let (xm_a, pa) = stat::stat2(&xs_a);
        let sb = pb.trace().unwrap().sqrt();
        let sa = pa.trace().unwrap().sqrt();
        let bias = xm_a - tr;
        println!("{},{},{},{},{},{},{},{},{}",
                 time,
                 tr[0],
                 tr[1],
                 tr[2],
                 bias[0],
                 bias[1],
                 bias[2],
                 sb,
                 sa);
    }
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().unwrap();
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);
    bias(args, setting);
}
