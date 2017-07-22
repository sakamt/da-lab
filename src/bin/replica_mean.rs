//! Calculate replica-mean
//!
//! Outputs
//! -------
//! Sequential data will be saved in "$DATADIR/replica_mean/YYYY-MM-DD-HH:MM:SS/"
//! - setting.json
//! - truth.msg : sequence of true state
//! - obs.msg   : sequence of observations
//! - out.msg   : msgpack of `Output` time series

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate ndarray;
extern crate ndarray_linalg;
extern crate aics_da;

use clap::App;
use ndarray::*;
use ndarray_linalg::*;

use aics_da::*;
use aics_da::types::*;

#[derive(Serialize)]
struct Output {
    time: f64,
    /// true state
    state: V,
    /// mean-error vector
    vme: V,
    /// root-mean-square error
    rmse: f64,
}

fn replica_mean(truth: Truth, saver: io::MsgpackSaver, setting: da::Setting) {
    let replica = setting.replica.expect("setting.replica is needed");
    let f = model::select_model(&setting);
    let a = da::select_analyzer(&setting);
    let mut xss: Vec<_> = (0..replica)
        .map(|_| {
            let xs = da::replica(&truth[0], setting.r, setting.k);
            let obs = observation::generate_obs(&truth, &setting);
            (xs, obs)
        })
        .collect();
    let tl: Vec<Output> = truth
        .into_iter()
        .enumerate()
        .map(|(t, truth)| {
            let res = xss.iter_mut()
                .map(|item| {
                    let xs = &mut item.0;
                    let obs = &item.1[t];
                    a.analysis_mut(xs, obs);
                    let xa = stat::mean(xs);
                    f.forecast_mut(xs);
                    let rmse = (&xa - &truth).norm() / (xa.len() as f64).sqrt();
                    (xa, rmse)
                })
                .fold((Array::zeros(truth.dim()), 0.0), |(a, b), (c, d)| {
                    (a + c, b + d)
                });
            let vme = res.0 / replica as f64 - &truth;
            let rmse = res.1 / replica as f64;
            Output {
                time: (t * setting.tau) as f64 * setting.dt,
                state: truth,
                vme: vme,
                rmse: rmse,
            }
        })
        .collect();
    saver.save("out.msg", &tl);
}

fn main() {
    exec::init();
    let cli = load_yaml!("replica_mean.yml");
    let m = App::from_yaml(cli).get_matches();
    let saver = io::MsgpackSaver::new("replica_mean");
    let setting = exec::ready_setting(m.value_of("config"), &saver.path);
    let truth = exec::ready_truth(
        m.value_of("init"),
        m.value_of("truth"),
        &saver.path,
        &setting,
    );
    replica_mean(truth, saver, setting);
}
