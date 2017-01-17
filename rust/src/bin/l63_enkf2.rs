#![allow(non_snake_case)]

extern crate ndarray;
extern crate ndarray_linalg;
extern crate rustc_serialize;
extern crate rusqlite;
extern crate aics_da;
extern crate docopt;
extern crate pbr;
extern crate itertools;

use std::io::stderr;
use docopt::Docopt;
use ndarray::prelude::*;
use ndarray_linalg::prelude::*;
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

fn enkf(setting: da::Setting, conn: &rusqlite::Connection) {
    let step = setting.dt * setting.tau as f64;
    let postfix = sql::util::now_str();

    let x0 = arr1(&[1.0, 0.0, 0.0]);
    let truth: Vec<V> = iterate(x0, |x| l63::teo(setting.dt, setting.tau, x.clone()))
        .take(setting.count)
        .collect();

    let obs_op = observation::ObsOperator::isotropic(3, setting.r);
    let obs: Vec<V> = truth.iter()
        .map(|x| obs_op.generate(x))
        .collect();

    let tid = sql::save_truth(step, &truth, &conn, &format!("truth_{}", postfix));
    let oid = sql::save_observation(step,
                                    setting.r,
                                    &obs,
                                    tid,
                                    &conn,
                                    &format!("obs_{}", postfix));

    let analyzer = enkf::EnKF::new(obs_op.clone());
    let teo = |x| l63::teo(setting.dt, setting.tau, x);

    let xs0 = da::replica(&truth[0], setting.r.sqrt(), setting.k);
    let enkf = obs.iter().scan(xs0, |xs, y| Some(da::iterate(&teo, &analyzer, xs, y)));

    let mut pb = ProgressBar::on(stderr(), setting.count as u64);
    let everyn = setting.everyn.unwrap_or(1);
    let ents = sql::EnsembleTS::new(&conn, &postfix);
    let stts = sql::StatTS::new(&conn, &postfix);
    for (t, ((tr, ob), (xs_b, xs_a))) in truth.iter().zip(obs.iter()).zip(enkf).enumerate() {
        pb.inc();
        let time = step * (t as f64);
        let (xm_b, pb) = stat::stat2(&xs_b);
        let rmse = stat::rmse(&xm_b, tr);
        let std = pb.trace().unwrap().sqrt();
        let w: weight::Weight = obs_op.log_weight(&xs_b, &ob).into();
        let xm_mpf = w.mean(&xs_b);
        let xm_a = stat::mean(&xs_a);
        let bias = (xm_a - xm_mpf).norm();
        stts.insert(time, rmse, std, bias);
        if t % everyn == 0 {
            let tb_xsb = sql::save_ensemble(&xs_b, &conn, &format!("{}_b{:05}", postfix, t / everyn));
            let tb_xsa = sql::save_ensemble(&xs_a, &conn, &format!("{}_a{:05}", postfix, t / everyn));
            ents.insert(time, &tb_xsb, &tb_xsa);
        }
    }
    let tb_ensemble = ents.table_name();
    let tb_stat = stts.table_name();
    sql::da::insert_enkf(&setting, tid, oid, tb_ensemble, tb_stat, &conn);
    pb.finish_print("Done!\n");
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);

    let mut conn = rusqlite::Connection::open(args.arg_db).unwrap();
    let tx = conn.transaction().unwrap();
    enkf(setting, &tx);
    tx.commit().unwrap();
}
