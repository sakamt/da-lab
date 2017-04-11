
extern crate ndarray;
extern crate rustc_serialize;
extern crate aics_da;
extern crate docopt;
extern crate pbr;

use std::io::stderr;
use docopt::Docopt;
use aics_da::*;
use aics_da::types::*;
use pbr::ProgressBar;

const USAGE: &'static str = "
Bias of methods for Lorenz63 model

Usage:
  l63_bias_collect <method> <setting> <truth> <obs>
";

#[derive(RustcDecodable)]
struct Args {
    arg_setting: String,
    arg_truth: String,
    arg_obs: String,
    arg_method: String,
}


fn bias(args: Args, setting: da::Setting) {
    let step = setting.dt * setting.tau as f64;

    let truth: Vec<V> = io::load_msg(&args.arg_truth);
    let obs: Vec<V> = io::load_msg(&args.arg_obs);

    let obs_op = observation::LinearNormal::isotropic(3, setting.r);

    let rho = setting.rho.unwrap_or(1.0);
    let analyzer: Box<da::EnsembleAnalyzer> = match args.arg_method.trim().as_ref() {
        "etkf" => Box::new(etkf::ETKF::new(obs_op, rho)),
        "enkf" => Box::new(enkf::EnKF::new(obs_op)),
        "mpf" => Box::new(mpf::MPF::new(obs_op, 3)),
        _ => panic!("unsupported method"),
    };
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    let xs0 = da::replica(&truth[0], setting.r.sqrt(), setting.k);
    let series = bias_collect::series(&teo, &*analyzer, xs0, &obs, &truth);

    let mut pb = ProgressBar::on(stderr(), setting.count as u64);
    println!("time,X,Y,Z,Ox,Oy,Oz,Bx,By,Bz");
    for (t, ((tr, ob), (_, xs_a))) in truth.iter().zip(obs.iter()).zip(series).enumerate() {
        pb.inc();
        let time = step * (t as f64);
        let xm_a = stat::mean(&xs_a);
        let bias = xm_a - tr;
        println!("{},{},{},{},{},{},{},{},{},{}",
                 time,
                 tr[0],
                 tr[1],
                 tr[2],
                 ob[0],
                 ob[1],
                 ob[2],
                 bias[0],
                 bias[1],
                 bias[2]);
    }
    pb.finish_print("Done!\n");
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);
    bias(args, setting);
}
