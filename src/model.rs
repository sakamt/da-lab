use ndarray::*;
use ndarray_odeint::{TimeEvolution, explicit, model};

use super::da;
use super::types::V;

pub struct L63 {
    teo: explicit::RK4<model::lorenz63::Lorenz63, f64>,
    step: usize,
}

impl<'a> TimeEvolution<OwnedRepr<f64>, Ix1> for &'a L63 {
    fn iterate(self, mut x: &mut V) -> &mut V {
        for _ in 0..self.step {
            self.teo.iterate(&mut x);
        }
        x
    }
}

pub fn select_model(setting: &da::Setting) -> Box<da::EnsembleForecaster> {
    match setting.model.as_str() {
        "l63" => {
            let p = model::lorenz63::Lorenz63::default();
            let rk4 = explicit::rk4(p, setting.dt);
            Box::new(L63 {
                teo: rk4,
                step: setting.tau,
            })
        }
        _ => panic!("unsupported model: {}", setting.model),
    }
}

pub fn generate_init(_setting: &da::Setting) -> V {
    V::zeros(3) // TODO moc
}

pub fn generate_truth(_init: &V, _setting: &da::Setting) -> Vec<V> {
    vec![V::zeros(3); 2]
}
