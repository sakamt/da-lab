
extern crate ndarray;
extern crate rustc_serialize;
extern crate rusqlite;
extern crate aics_da;
extern crate docopt;
extern crate pbr;

use std::io::stderr;
use docopt::Docopt;
use aics_da::*;
use pbr::ProgressBar;

const USAGE: &'static str = "
Bias of EnKF for Lorenz63 model

Usage:
  l63_enkf_bias <setting>
";

#[derive(RustcDecodable)]
struct Args {
    arg_setting: String,
}

fn thin_out<T: Clone>(tl: &Vec<T>, n: usize) -> Vec<T> {
    tl.iter().enumerate().filter(|&(i, _)| i % n == 0).map(|(_, v)| v.clone()).collect()
}

fn enkf_bias(setting: da::Setting) {
    let step = setting.dt * setting.tau as f64;

    let truth = l63::generate_truth(&setting);
    let obs_op = observation::LinearNormal::isotropic(3, setting.r);
    let obs = observation::eval_series(&obs_op, &setting, &truth, setting.dt);

    let truth = thin_out(&truth, setting.tau);

    let analyzer = enkf::EnKF::new(obs_op.clone());
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    let xs0 = da::replica(&truth[0], setting.r.sqrt(), setting.k);
    let enkf = obs.iter().scan(xs0, |xs, y| Some(da::iterate(&teo, &analyzer, xs, y)));

    let mut pb = ProgressBar::on(stderr(), setting.count as u64);
    println!("time,X,Y,Z,Bx,By,Bz");
    for (t, (tr, (_, xs_a))) in truth.iter().zip(enkf).enumerate() {
        pb.inc();
        let time = step * (t as f64);
        let xm_a = stat::mean(&xs_a);
        let bias = xm_a - tr;
        println!("{},{},{},{},{},{},{}",
                 time,
                 tr[0],
                 tr[1],
                 tr[1],
                 bias[0],
                 bias[1],
                 bias[2]);
    }
    pb.finish_print("Done!\n");
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);
    enkf_bias(setting);
}
