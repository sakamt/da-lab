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
use std::path::PathBuf;

use aics_da::*;
use aics_da::types::*;

// input for DA process
struct Input {
    truth: Vec<V>,
    obs: Vec<V>,
    out_dir: PathBuf,
}

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
fn run(input: &Input, setting: &da::Setting) {
    let f = model::select_model(&setting);
    let a = da::select_analyzer(&setting);
    let mut xs = da::replica(&input.truth[0], setting.r, setting.k);
    for (t, (truth, y)) in input.truth.iter().zip(input.obs.iter()).enumerate() {
        let xb = stat::mean(&xs);
        xs = a.analysis(xs, &y);
        let xa = stat::mean(&xs);
        xs = f.forecast(xs);
        let rmse = (truth - &xa).norm();
        let output = Output {
            time: (t * setting.tau) as f64 * setting.dt,
            state: truth.clone(),
            obs: y.clone(),
            forecast: xb,
            analysis: xa,
            rmse: rmse,
        };
        let out_fn = format!("data{:05}.msg", t);
        io::save_msg(&output, input.out_dir.join(out_fn).to_str().unwrap());
    }
}

fn main() {
    exec::init();

    let cli = load_yaml!("run.yml");
    let m = App::from_yaml(cli).get_matches();

    let out_dir = exec::ready_out_dir("run");
    let setting = exec::ready_setting(m.value_of("config"), &out_dir);

    let truth = exec::ready_truth(m.value_of("init"), m.value_of("truth"), &out_dir, &setting);
    let obs = exec::ready_obs(m.value_of("obs"), &truth, &out_dir, &setting);

    let input = Input {
        truth: truth,
        obs: obs,
        out_dir: out_dir,
    };

    run(&input, &setting);
}
