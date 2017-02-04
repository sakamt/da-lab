
extern crate ndarray;
extern crate rustc_serialize;
extern crate rusqlite;
extern crate aics_da;
extern crate docopt;
extern crate pbr;

use std::io::stderr;
use docopt::Docopt;
use aics_da::*;
use aics_da::io::*;
use aics_da::settings::*;
use pbr::ProgressBar;

const USAGE: &'static str = "
EnKF for Lorenz63 model

Usage:
  l63_enkf <setting> <db>
";

#[derive(RustcDecodable)]
struct Args {
    arg_setting: String,
    arg_db: String,
}

fn thin_out<T: Clone>(tl: &Vec<T>, n: usize) -> Vec<T> {
    tl.iter().enumerate().filter(|&(i, _)| i % n == 0).map(|(_, v)| v.clone()).collect()
}

fn enkf(setting: da::Setting, conn: &rusqlite::Connection) {
    let step = setting.dt * setting.tau as f64;

    let truth = l63::generate_truth(&setting);
    let obs_op = observation::LinearNormal::isotropic(3, setting.r);
    let obs = observation::eval_series(&obs_op, &setting, &truth, setting.dt);

    let tid = conn.save_truth(&setting.induce(), &truth);
    let oid = conn.save_observation(&setting.induce(), tid, &obs);

    let truth = thin_out(&truth, setting.tau);

    let analyzer = enkf::EnKF::new(obs_op.clone());
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    let xs0 = da::replica(&truth[0], setting.r.sqrt(), setting.k);
    let enkf = obs.iter().scan(xs0, |xs, y| Some(da::iterate(&teo, &analyzer, xs, y)));

    let mut pb = ProgressBar::on(stderr(), setting.count as u64);
    let everyn = setting.everyn.unwrap_or(1);
    let mut ensemble_series = Vec::new();
    let mut stat_series = Vec::new();
    for (t, ((tr, ob), (xs_f, xs_a))) in truth.iter().zip(obs.iter()).zip(enkf).enumerate() {
        pb.inc();
        let time = step * (t as f64);
        let st = stat::Stat::eval(&obs_op, &xs_f, &xs_a, tr, ob);
        stat_series.push((time, st));
        if t % everyn == 0 {
            let tb_xsb = conn.save_ensemble(&xs_f);
            let tb_xsa = conn.save_ensemble(&xs_a);
            ensemble_series.push((time, tb_xsb, tb_xsa));
        }
    }
    let tb_ensemble = conn.commit_ensemble_series(ensemble_series.as_slice());
    let tb_stat = conn.save_stat(stat_series.as_slice());
    sqlite::da::insert_enkf(&setting, tid, oid, &tb_ensemble, &tb_stat, &conn);
    pb.finish_print("Done!\n");
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);
    let mut conn = sqlite::open_with_init(&args.arg_db);
    let tx = conn.transaction().unwrap();
    enkf(setting, &tx);
    tx.commit().unwrap();
}
