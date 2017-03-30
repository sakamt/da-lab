
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
Run DA for Lorenz63 model

Usage:
  l63_run <method> <setting> <observation> <init> <output>
";

#[derive(RustcDecodable)]
struct Args {
    arg_setting: String,
    arg_observation: String,
    arg_init: String,
    arg_output: String,
    arg_method: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    std::fs::create_dir_all(&args.arg_output).unwrap();
    let setting: da::Setting = io::read_json(&args.arg_setting);
    let x0: V = io::load_msg(&args.arg_init);
    let obs: Vec<V> = io::load_msg(&args.arg_observation);
    let duration = obs.len();
    let everyn = setting.everyn.unwrap_or(1);
    let rho = setting.rho.unwrap_or(1.0);

    // DA settings
    let h = Array::<f64, _>::eye(3);
    let rs = setting.r.sqrt() * Array::<f64, _>::eye(3);
    let obs_op = observation::LinearNormal::new(h, rs);
    let analyzer: Box<da::EnsembleAnalyzer> = match args.arg_method.trim().as_ref() {
        "etkf" => Box::new(etkf::ETKF::new(obs_op, rho)),
        "enkf" => Box::new(enkf::EnKF::new(obs_op)),
        "mpf" => Box::new(mpf::MPF::new(obs_op, 3)),
        _ => panic!("unsupported method"),
    };
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    // generate DA sequence
    let xs0 = da::replica(&x0, setting.r.sqrt(), setting.k);
    let etkf = obs.iter().scan(xs0, |xs, y| Some(da::iterate(&teo, &*analyzer, xs, y)));

    let mut pb = ProgressBar::new(duration as u64);
    for (t, (xs_b, xs_a)) in etkf.enumerate() {
        pb.inc();
        if t % everyn == 0 {
            let tt = t / everyn;
            io::save_msg(&xs_b, &format!("{}/b{:05}.msg", args.arg_output, tt));
            io::save_msg(&xs_a, &format!("{}/a{:05}.msg", args.arg_output, tt));
        }
    }
    pb.finish_print("done!\n");
}
