#![allow(non_snake_case)]

extern crate ndarray;
extern crate rustc_serialize;
extern crate rusqlite;
extern crate aics_da;
extern crate docopt;
extern crate pbr;
extern crate itertools;

use docopt::Docopt;
use ndarray::prelude::*;
use aics_da::*;
use aics_da::types::V;
use aics_da::sqlite as sql;
use pbr::ProgressBar;
use itertools::iterate;

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

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);

    let mut conn = rusqlite::Connection::open(args.arg_db).unwrap();
    let postfix = sql::util::now_str();

    let x0 = arr1(&[1.0, 0.0, 0.0]);
    let truth: Vec<V> = iterate(x0, |x| l63::teo(setting.dt, setting.tau, x.clone()))
        .take(setting.count)
        .collect();

    let h = Array::<f64, _>::eye(3);
    let rs = setting.r.sqrt() * Array::<f64, _>::eye(3);

    let obs: Vec<V> = truth.iter()
        .map(|x| x + &observation::noise(&rs))
        .collect();

    let tb_truth;
    let tb_obs;
    {
        let tx = conn.transaction().unwrap();
        tb_truth = sql::save_ensemble(&truth, &tx, &format!("truth_{}", postfix));
        tb_obs = sql::save_ensemble(&obs, &tx, &format!("obs_{}", postfix));
        tx.commit().unwrap();
    }

    let obs_op = observation::ObsOperator::new(h, rs);
    let analyzer = enkf::EnKF::new(obs_op);
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    let xs0 = da::replica(&truth[0], setting.r.sqrt(), setting.k);
    let enkf = obs.iter().scan(xs0, |xs, y| Some(da::iterate(&teo, &analyzer, xs, y)));

    let mut pb = ProgressBar::new(setting.count as u64);
    let everyn = setting.everyn.unwrap_or(1);
    let tb_ensemble = sql::ensemble_ts::generate_table_name(&postfix);
    sql::ensemble_ts::create_table(&conn, &tb_ensemble);
    let tx = conn.transaction().unwrap();
    for (t, (xs_b, xs_a)) in enkf.enumerate() {
        pb.inc();
        if t % everyn == 0 {
            let time = setting.dt * (setting.tau * t) as f64;
            let tb_xsb = sql::save_ensemble(&xs_b, &tx, &format!("{}_b{:05}", postfix, t / everyn));
            let tb_xsa = sql::save_ensemble(&xs_a, &tx, &format!("{}_a{:05}", postfix, t / everyn));
            sql::ensemble_ts::insert(time, &tb_xsb, &tb_xsa, &tx, &tb_ensemble);
        }
    }
    sql::da::insert_enkf(&setting, &tb_ensemble, &tb_truth, &tb_obs, &tx);
    tx.commit().unwrap();
    pb.finish_print("done!\n");
}
