use ndarray_linalg::*;
use ndarray_odeint::*;

use super::da;
use super::types::V;

fn l63(setting: &da::Setting) -> NStep<explicit::RK4<model::lorenz63::Lorenz63, f64>> {
    let p = model::lorenz63::Lorenz63::default();
    let rk4 = explicit::rk4(p, setting.dt);
    nstep(rk4, setting.tau)
}

pub fn select_model(setting: &da::Setting) -> Box<da::EnsembleForecaster> {
    match setting.model.as_str() {
        "l63" => Box::new(l63(&setting)),
        _ => panic!("unsupported model: {}", setting.model),
    }
}

pub fn generate_init(setting: &da::Setting) -> V {
    match setting.model.as_str() {
        "l63" => {
            let teo = l63(&setting);
            let mut x0 = generate::random(teo.model_size());
            for _ in 0..setting.count / 10 {
                teo.iterate(&mut x0);
            }
            x0
        }
        _ => panic!("unsupported model: {}", setting.model),
    }
}

pub fn generate_truth(init: &V, setting: &da::Setting) -> Vec<V> {
    match setting.model.as_str() {
        "l63" => {
            let x = init.to_owned();
            let teo = l63(&setting);
            time_series(x, &teo).take(setting.count).collect()
        }
        _ => panic!("unsupported model: {}", setting.model),
    }
}
