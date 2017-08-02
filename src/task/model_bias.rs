use ndarray::*;
use ndarray_linalg::*;
use ndarray_odeint::*;

use super::{ready_obs, ready_truth};
use super::da::*;
use {da, io, observation, stat};

#[derive(Clone, Copy, Debug)]
struct BiasedLorenz63 {
    pub p: f64,
    pub r: f64,
    pub b: f64,
    /// model error amplitude
    pub e: f64,
}

impl Default for BiasedLorenz63 {
    fn default() -> Self {
        Self {
            p: 10.0,
            r: 28.0,
            b: 8.0 / 3.0,
            e: 0.0,
        }
    }
}

impl ModelSize<Ix1> for BiasedLorenz63 {
    fn model_size(&self) -> usize {
        3
    }
}

impl<S> Explicit<S, Ix1> for BiasedLorenz63
where
    S: DataMut<Elem = f64>,
{
    type Scalar = f64;
    type Time = f64;

    fn rhs<'a>(&self, mut v: &'a mut ArrayBase<S, Ix1>) -> &'a mut ArrayBase<S, Ix1> {
        let x = v[0];
        let y = v[1];
        let z = v[2];
        v[0] = self.p * (y - x);
        v[1] = x * (self.r - z) - y;
        v[2] = x * y - self.b * z + self.e * x * y * z; // add e*xyz
        v
    }
}

fn biased_l63(setting: &Setting) -> NStep<explicit::RK4<BiasedLorenz63, f64>> {
    let mut p = BiasedLorenz63::default();
    p.e = setting.model_bias.expect("model_bias is necessary");
    let rk4 = explicit::rk4(p, setting.dt);
    nstep(rk4, setting.tau)
}

/// Data for model-bias learning
pub fn model_bias(setting: Setting) {
    let truth = ready_truth(&setting);
    let obs = ready_obs(&truth, &setting);
    let saver = io::MsgpackSaver::new("model_bias");
    saver.save_as_map("setting", &setting);
    saver.save("truth", &truth);
    saver.save("obs", &obs);

    let f = biased_l63(&setting);
    let a = select_analyzer(&setting);
    let mut xs = replica(&truth[0], setting.r, setting.k);
    let mut outs = Vec::new();
    let mut rmse_total = 0.0;
    for (truth, y) in truth.into_iter().zip(obs.iter()) {
        let xb = stat::mean(&xs);
        xs = a.analysis(xs, &y);
        let xa = stat::mean(&xs);
        rmse_total += (&truth - &xa).norm() / (xa.len() as f64).sqrt();
        xs = f.forecast(xs);
        outs.push((truth, xb, xa));
    }
    saver.save("model_bias", &outs);
    info!("mean RMSE = {}", rmse_total / setting.count as f64);
}

/// Generate an answer of model-bias learning
pub fn model_bias_replica(setting: Setting) {
    let truth = ready_truth(&setting);
    let saver = io::MsgpackSaver::new("model_bias_replica");
    saver.save_as_map("setting", &setting);
    saver.save("truth", &truth);

    let n = truth[0].len();

    let replica = setting.replica.expect("setting.replica is needed");
    let f = biased_l63(&setting);
    let a = select_analyzer(&setting);
    let mut xss: Vec<_> = (0..replica)
        .map(|_| {
            let xs = da::replica(&truth[0], setting.r, setting.k);
            let obs = observation::generate_obs(&truth, &setting);
            (xs, obs)
        })
        .collect();
    let tl: Vec<_> = truth
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
                .fold((Array::zeros(n), 0.0), |(x, r), (x_, r_)| (x + x_, r + r_));
            let vme = res.0 / replica as f64 - &truth;
            let rmse = res.1 / replica as f64;
            (truth, vme, rmse)
        })
        .collect();
    saver.save("model_bias_replica", &tl);
}
