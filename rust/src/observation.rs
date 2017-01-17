
use ndarray::prelude::*;
use ndarray_linalg::prelude::*;
use rand::distributions::*;
use ndarray_rand::RandomExt;
use float_cmp::ApproxEqRatio;

use types::*;
use super::{da, weight};


pub fn noise(rs: &M) -> V {
    let (n, _) = rs.size();
    let dist = Normal::new(0., 1.0);
    let d = Array::random(n, dist);
    rs.dot(&d)
}

/// Observation Operator
/// - Linear operator (and expressed as a matrix)
/// - Gaussian noise
#[derive(Clone, Debug)]
pub struct ObsOperator {
    h: M,
    rs: M,
}

impl ObsOperator {
    pub fn new(h: M, rs: M) -> Self {
        ObsOperator { h: h, rs: rs }
    }
    pub fn isotropic(n: usize, r: f64) -> Self {
        let h = Array::<f64, _>::eye(n);
        let rs = r * &h;
        Self::new(h, rs)
    }
    pub fn generate(&self, truth: &V) -> V {
        self.h.dot(truth) + noise(&self.rs)
    }
    pub fn info_gain(&self) -> M {
        // FIXME
        self.rs.dot(&self.rs)
    }
    pub fn log_weight(&self, xs: &Ensemble, y: &V) -> weight::LogWeight {
        let ws: Vec<_> = xs.iter()
            .map(|x| {
                let dev = y - &self.h.dot(x);
                -0.5 * self.rs.dot(&dev).norm()
            })
            .collect();
        ws.into()
    }
}

pub fn generate(setting: da::Setting, truth: &Vec<V>, truth_dt: f64) -> Vec<V> {
    let step = setting.tau as f64 * setting.dt;
    let n = get_ratio(step, truth_dt).expect("dt are imcompatible");
    truth.iter()
        .enumerate()
        .filter(|&(i, _)| i as i64 % n == 0)
        .map(|(_, v)| v.clone())
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
