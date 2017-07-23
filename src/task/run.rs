//! run DA process

use ndarray_linalg::*;

use super::{ready_obs, ready_truth};
use super::types::*;
use {da, io, model, stat};

#[derive(Serialize)]
struct Output {
    time: f64,
    state: V,
    obs: V,
    forecast: V,
    analysis: V,
    rmse: f64,
}

/// Run DA
///
/// Outputs
/// -------
/// Sequential data will be saved in "$DATADIR/run/YYYY-MM-DD-HH:MM:SS/"
/// - setting.json
/// - truth.msg : sequence of true state
/// - obs.msg   : sequence of observations
/// - data00001.msg ... : msgpack of dictionary:
///   - time
///   - state
///   - obs
///   - forecast
///   - analysis
///   - rmse
pub fn run(setting: da::Setting) {
    let truth = ready_truth(&setting);
    let obs = ready_obs(&truth, &setting);
    let saver = io::MsgpackSaver::new("run");
    saver.save_as_map("setting", &setting);
    saver.save("truth", &truth);
    saver.save("obs", &obs);

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
        let out_fn = format!("data{:05}", t);
        saver.save(&out_fn, &output);
        rmse_ts.push(rmse);
    }
    info!(
        "mean RMSE = {}",
        rmse_ts.iter().sum::<f64>() / truth.len() as f64
    );
}
