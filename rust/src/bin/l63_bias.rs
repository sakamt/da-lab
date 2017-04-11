
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
  l63_bias <da> <special> <setting> <truth> <obs> [--progress]
  l63_bias (-h | --help)

Options:
  -h --help   Show this
  --progress  Show progress bar
  --special=<special>   Special technique
";

#[derive(RustcDecodable)]
struct Args {
    arg_da: String,
    arg_special: String,
    arg_setting: String,
    arg_truth: String,
    arg_obs: String,
    flag_progress: bool,
}

fn bias(args: Args, setting: da::Setting) {
    let step = setting.dt * setting.tau as f64;

    let truth: Vec<V> = io::load_msg(&args.arg_truth);
    let obs: Vec<V> = io::load_msg(&args.arg_obs);

    let analyzer = select_analyzer(args.arg_da.trim(), setting);
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    let xs0 = da::replica(&truth[0], setting.r.sqrt(), setting.k);
    let series = match args.arg_special.trim().as_ref() {
        "bias" => da::series(&teo, &*analyzer, xs0, &obs),
        "bias_collect" => bias_collect::series(&teo, &*analyzer, xs0, &obs, &truth),
        _ => panic!("Invalid special: {}", args.arg_special),
    };

    let mut pb = if args.flag_progress {
        Some(ProgressBar::on(stderr(), setting.count as u64))
    } else {
        None
    };
    println!("time,X,Y,Z,Ox,Oy,Oz,Bx,By,Bz");
    for (t, ((tr, ob), (_, xs_a))) in truth.iter().zip(obs.iter()).zip(series).enumerate() {
        pb = pb.map(|mut p| {
            p.inc();
            p
        });
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
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);
    bias(args, setting);
}
