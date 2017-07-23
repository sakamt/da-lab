//! Run DA
//!
//! Outputs
//! -------
//! Sequential data will be saved in "$DATADIR/run/YYYY-MM-DD-HH:MM:SS/"
//! - setting.json
//! - truth.msg : sequence of true state
//! - obs.msg   : sequence of observations
//! - data00001.msg ... : msgpack of dictionary:
//!   - time
//!   - state
//!   - obs
//!   - forecast
//!   - analysis
//!   - rmse

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate ndarray_linalg;
extern crate aics_da;

use clap::App;
use ndarray_linalg::*;

use aics_da::*;
use aics_da::types::*;

#[derive(Serialize)]
struct Output {
    time: f64,
    state: V,
    obs: V,
    forecast: V,
    analysis: V,
    rmse: f64,
}

// run DA process
fn run(truth: Truth, obs: Observation, saver: io::MsgpackSaver, setting: da::Setting) {
    let f = model::select_model(&setting);
    let a = da::select_analyzer(&setting);
    let mut xs = da::replica(&truth[0], setting.r, setting.k);
    let mut rmse_ts = Vec::new();
    for (t, (truth, y)) in truth.iter().zip(obs.iter()).enumerate() {
        let xb = stat::mean(&xs);
        xs = a.analysis(xs, &y);
        let xa = stat::mean(&xs);
        xs = f.forecast(xs);
        let rmse = (truth - &xa).norm() / (xa.len() as f64).sqrt();
        let output = Output {
            time: (t * setting.tau) as f64 * setting.dt,
            state: truth.clone(),
            obs: y.clone(),
            forecast: xb,
            analysis: xa,
            rmse: rmse,
        };
        let out_fn = format!("data{:05}.msg", t);
        saver.save(&out_fn, &output);
        rmse_ts.push(rmse);
    }
    println!(
        "mean RMSE = {}",
        rmse_ts.iter().sum::<f64>() / truth.len() as f64
    );
}

fn main() {
    exec::init();

    let cli = load_yaml!("run.yml");
    let m = App::from_yaml(cli).get_matches();

    let saver = io::MsgpackSaver::new("run");
    let setting = exec::ready_setting(m.value_of("config"), &saver.path);

    let truth = exec::ready_truth(
        m.value_of("init"),
        m.value_of("truth"),
        &setting,
    );
    saver.save("truth", &truth);
    let obs = exec::ready_obs(m.value_of("obs"), &truth, &setting);
    saver.save("obs", &obs);

    run(truth, obs, saver, setting);
}
