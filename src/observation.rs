use float_cmp::ApproxEqRatio;
use ndarray::*;
use ndarray_linalg::*;

use super::{da, linalg, weight};
use super::types::*;

pub trait ObservationOperator {
    fn noisy_eval(&self, x: &V) -> V;
    fn eval(&self, x: &V) -> V;
}

pub trait WeightEvaluator {
    fn log_weight(&self, xs: &Ensemble, y: &V) -> weight::LogWeight {
        self.weight(xs, y).into()
    }
    fn weight(&self, xs: &Ensemble, y: &V) -> weight::Weight {
        self.log_weight(xs, y).into()
    }
}

pub trait LinearTheory {
    /// correlation matrix
    fn corr(&self) -> M;
    /// information gain $\Omega = H^TR^{-1}H$
    fn info_gain(&self) -> M;
    /// Kalman gain matrix
    fn kalman_gain(&self, p: &M) -> M;
    /// execute an analysis step only for covariance matrix $ P \to (1-KH)P$
    fn covariance_analysis(&self, p: &M) -> M;
}

/// Linear observation operator with Gaussian noise
#[derive(Clone, Debug)]
pub struct LinearNormal {
    h: M,
    rs: M,
}

impl ObservationOperator for LinearNormal {
    fn noisy_eval(&self, truth: &V) -> V {
        self.h.dot(truth) + noise(&self.rs)
    }
    fn eval(&self, x: &V) -> V {
        self.h.dot(x)
    }
}

impl WeightEvaluator for LinearNormal {
    fn log_weight(&self, xs: &Ensemble, y: &V) -> weight::LogWeight {
        let rs_inv = self.rs.clone().inv().unwrap();
        let ws: Vec<_> = xs.iter()
            .map(|x| {
                let dev = y - &self.h.dot(x);
                -0.5 * rs_inv.dot(&dev).norm().powi(2)
            })
            .collect();
        ws.into()
    }
}

impl LinearTheory for LinearNormal {
    fn corr(&self) -> M {
        self.rs.dot(&self.rs)
    }
    fn info_gain(&self) -> M {
        let r_inv = self.corr().inv().unwrap();
        linalg::bracket(&r_inv, &self.h)
    }
    fn kalman_gain(&self, p: &M) -> M {
        let v = linalg::bracket(p, &self.h.t().to_owned()) + self.rs.dot(&self.rs);
        p.dot(&self.h.t()).dot(&v.inv().unwrap())
    }
    fn covariance_analysis(&self, p: &M) -> M {
        let k = self.kalman_gain(p);
        let n = k.rows();
        (Array::eye(n) - k.dot(&self.h)).dot(p)
    }
}

impl LinearNormal {
    pub fn new(h: M, rs: M) -> Self {
        LinearNormal { h: h, rs: rs }
    }
    pub fn isotropic(n: usize, r: f64) -> Self {
        let h = Array::<f64, _>::eye(n);
        let rs = r.sqrt() * &h;
        Self::new(h, rs)
    }
}

pub fn eval_series<Obs>(obs: &Obs, setting: &da::Setting, truth: &Vec<V>, truth_dt: f64) -> Vec<V>
where
    Obs: ObservationOperator,
{
    let step = setting.tau as f64 * setting.dt;
    let n = get_ratio(step, truth_dt).expect("dt are imcompatible");
    if n as usize * setting.count > truth.len() {
        panic!("truth is too short");
    }
    truth
        .iter()
        .enumerate()
        .filter(|&(i, _)| i as i64 % n == 0)
        .map(|(_, v)| obs.noisy_eval(v))
        .collect()
}

/// test $\exists n \in N, s.t. a = nb$ and return $n$ if exists
fn get_ratio(a: f64, b: f64) -> Option<i64> {
    let n = (a / b).round() as i64;
    if (n as f64 * b).approx_eq_ratio(&a, 1e-7) {
        Some(n)
    } else {
        None
    }
}

/// DEPRICATED: will be private
pub fn noise(rs: &M) -> V {
    let n = rs.rows();
    let d: Array1<f64> = generate::random(n);
    rs.dot(&d)
}

pub fn generate_obs(truth: &Vec<V>, setting: &da::Setting) -> Vec<V> {
    truth
        .iter()
        .map(|x| setting.r.sqrt() * generate::random(x.len()) + x)
        .collect()
}
