
extern crate ndarray;
extern crate rustc_serialize;
extern crate aics_da;
extern crate docopt;
extern crate pbr;
extern crate env_logger;
extern crate dotenv;

use aics_da::*;
use aics_da::types::V;
use docopt::Docopt;
use pbr::ProgressBar;
use std::io::stderr;

const USAGE: &'static str = "
Run DA for Lorenz63 model

Usage:
  l63_run <da> <setting> <obs> <init> <output> [--progress]
  l63_run (-h | --help)

Options:
  -h --help   Show this
  --progress  Show progress bar
";

#[derive(RustcDecodable)]
struct Args {
    arg_da: String,
    arg_setting: String,
    arg_obs: String,
    arg_init: String,
    arg_output: String,
    flag_progress: bool,
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().unwrap();
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(
        |e| e.exit(),
    );
    std::fs::create_dir_all(&args.arg_output).unwrap();
    let setting: da::Setting = io::read_json(&args.arg_setting);
    let x0: V = io::load_msg(&args.arg_init);
    let obs: Vec<V> = io::load_msg(&args.arg_obs);
    let duration = obs.len();
    let everyn = setting.everyn.unwrap_or(1);

    // DA settings
    let analyzer = select_analyzer(args.arg_da.trim(), setting);
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    // generate DA sequence
    let xs0 = da::replica(&x0, setting.r.sqrt(), setting.k);
    let etkf = obs.iter().scan(xs0, |xs, y| {
        Some(da::iterate(&teo, &*analyzer, xs, y))
    });

    let mut pb = if args.flag_progress {
        Some(ProgressBar::on(stderr(), duration as u64))
    } else {
        None
    };
    for (t, (xs_b, xs_a)) in etkf.enumerate() {
        pb = pb.map(|mut p| {
            p.inc();
            p
        });
        if t % everyn == 0 {
            let tt = t / everyn;
            io::save_msg(&xs_b, &format!("{}/b{:05}.msg", args.arg_output, tt));
            io::save_msg(&xs_a, &format!("{}/a{:05}.msg", args.arg_output, tt));
        }
    }
    pb.map(|mut p| p.finish_print("done!\n"));
}
