//! Calculate replica-mean
//!
//! Outputs
//! -------
//! Sequential data will be saved in "$DATADIR/replica_mean/YYYY-MM-DD-HH:MM:SS/"
//! - setting.json
//! - truth.msg : sequence of true state
//! - obs.msg   : sequence of observations
//! - rm00001.msg ... : msgpack of `Output`

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
use std::path::PathBuf;

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

fn replica_mean(truth: Truth, out_dir: PathBuf, setting: da::Setting) {
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
    for (t, truth) in truth.into_iter().enumerate() {
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
        let output = Output {
            time: (t * setting.tau) as f64 * setting.dt,
            state: truth,
            vme: vme,
            rmse: rmse,
        };
        let out_fn = format!("rm{:05}.msg", t);
        io::save_msg(&output, out_dir.join(out_fn).to_str().unwrap());
    }
}

fn main() {
    exec::init();
    let cli = load_yaml!("replica_mean.yml");
    let m = App::from_yaml(cli).get_matches();
    let out_dir = exec::ready_out_dir("replica_mean");
    let setting = exec::ready_setting(m.value_of("config"), &out_dir);
    let truth = exec::ready_truth(m.value_of("init"), m.value_of("truth"), &out_dir, &setting);
    replica_mean(truth, out_dir, setting);
}
