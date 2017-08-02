use ndarray::*;
use ndarray_odeint::*;

use super::{ready_obs, ready_truth};
use super::da::*;
use super::types::*;
use {io, stat};

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

#[derive(Serialize)]
struct Output {
    state: V,
    increment: V,
}

pub fn model_bias(setting: Setting) {
    let truth = ready_truth(&setting);
    let obs = ready_obs(&truth, &setting);
    let saver = io::MsgpackSaver::new("run");
    saver.save_as_map("setting", &setting);
    saver.save("truth", &truth);
    saver.save("obs", &obs);

    let f = biased_l63(&setting);
    let a = select_analyzer(&setting);
    let mut xs = replica(&truth[0], setting.r, setting.k);
    let mut outs = Vec::new();
    for (truth, y) in truth.iter().zip(obs.iter()) {
        let xb = stat::mean(&xs);
        xs = a.analysis(xs, &y);
        let xa = stat::mean(&xs);
        xs = f.forecast(xs);
        outs.push(Output {
            state: truth.clone(),
            increment: xa - xb,
        });
    }
    saver.save("inc", &outs);
}
