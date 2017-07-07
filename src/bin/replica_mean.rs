
extern crate env_logger;
extern crate dotenv;
extern crate pbr;

extern crate ndarray;
extern crate ndarray_linalg;

extern crate aics_da;

use aics_da::*;
use aics_da::types::*;
use ndarray_linalg::prelude::*;
use pbr::ProgressBar;
use std::io::stderr;

fn replica(truth: &Vec<V>, s: da::Setting) {
    let analyzer = select_analyzer(args.arg_da.trim(), setting);
    let teo = |x| l63::teo(setting.dt, setting.tau, x);
    let xs0 = da::replica(&truth[0], setting.r.sqrt(), setting.k);
    let series = da::series(&teo, &*analyzer, xs0, &obs, &truth);
}

fn bias(setting: da::Setting) {
    let step = setting.dt * setting.tau as f64;

    let truth: Vec<V> = io::load_msg(&args.arg_truth);
    let obs: Vec<V> = io::load_msg(&args.arg_obs);

    let analyzer = select_analyzer(args.arg_da.trim(), setting);
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    let xs0 = da::replica(&truth[0], setting.r.sqrt(), setting.k);
    let series = da::series(&teo, &*analyzer, xs0, &obs, &truth);

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
    }
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().unwrap();
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(
        |e| e.exit(),
    );
    let setting: da::Setting = io::read_json(&args.arg_setting);
    bias(args, setting);
}
