//! calc replica mean

use ndarray::*;
use ndarray_linalg::*;

use super::ready_truth;
use super::types::*;

use {da, io, model, observation, stat};

#[derive(Serialize)]
struct Output {
    time: f64,
    /// true state
    state: V,
    /// mean-error vector
    vme: V,
    /// root-mean-square error
    rmse: f64,
    /// spread of forecast
    sb: f64,
    /// spread of analysis
    sa: f64,
}

/// Calculate replica-mean
///
/// Outputs
/// -------
/// Sequential data will be saved in "$DATADIR/replica_mean/YYYY-MM-DD-HH:MM:SS/"
/// - setting.json
/// - truth.msg : sequence of true state
/// - obs.msg   : sequence of observations
/// - out.msg   : msgpack of `Output` time series
pub fn replica_mean(setting: da::Setting) {
    let truth = ready_truth(&setting);
    let saver = io::MsgpackSaver::new("replica_mean");
    saver.save_as_map("setting", &setting);
    saver.save("truth", &truth);

    let n = truth[0].len();

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
                    let (_, pb) = stat::stat2(xs);
                    let obs = &item.1[t];
                    a.analysis_mut(xs, obs);
                    let (xa, pa) = stat::stat2(xs);
                    f.forecast_mut(xs);
                    let rmse = (&xa - &truth).norm() / (xa.len() as f64).sqrt();
                    (xa, rmse, pb, pa)
                })
                .fold(
                    (
                        Array::zeros(n),
                        0.0,
                        Array::zeros((n, n)),
                        Array::zeros((n, n)),
                    ),
                    |(x, r, pa, pb), (x_, r_, pa_, pb_)| (x + x_, r + r_, pa + pa_, pb + pb_),
                );
            let vme = res.0 / replica as f64 - &truth;
            let rmse = res.1 / replica as f64;
            let sb = (res.2.trace().unwrap() / replica as f64).sqrt();
            let sa = (res.3.trace().unwrap() / replica as f64).sqrt();
            Output {
                time: (t * setting.tau) as f64 * setting.dt,
                state: truth,
                vme: vme,
                rmse: rmse,
                sb: sb,
                sa: sa,
            }
        })
        .collect();
    saver.save("out", &tl);
}
