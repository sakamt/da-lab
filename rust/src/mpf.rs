
use rand;
use rand::distributions::IndependentSample;
use num_traits::float::Float;
use ndarray::prelude::*;

use types::*;
use weight::*;
use observation::*;
use da::EnsembleAnalyzer;

/// Coefficient for merge resampling
#[derive(Clone, Debug)]
pub struct MergeResampler {
    coef: Vec<f64>,
}

impl Default for MergeResampler {
    fn default() -> Self {
        MergeResampler { coef: vec![3.0 / 4.0, (13.0.sqrt() + 1.0) / 8.0, -(13.0.sqrt() - 1.0) / 8.0] }
    }
}

impl MergeResampler {
    pub fn resampling(&self, w: &Weight, xs: &Ensemble) -> Ensemble {
        let n = xs[0].len();
        let k = xs.len();
        let dist = w.to_dist();
        let mut rng = rand::thread_rng();
        (0..k)
            .map(|_| {
                self.coef
                    .iter()
                    .map(|&coef| {
                        let idx = dist.ind_sample(&mut rng);
                        coef * &xs[idx]
                    })
                    .fold(Array::zeros(n), |sum, x| sum + x)
            })
            .collect()
    }
}

/// merging particle filter
#[derive(Clone, Debug)]
pub struct MPF {
    resampler: MergeResampler,
    obs: LinearNormal,
}

impl MPF {
    pub fn new(obs: LinearNormal, n: usize) -> Self {
        if n != 3 {
            panic!("MPF: only n=3 is supported now.");
        }
        MPF {
            resampler: MergeResampler::default(),
            obs: obs,
        }
    }
}

impl EnsembleAnalyzer for MPF {
    fn analysis(&self, xs: Ensemble, y: &V) -> Ensemble {
        let w: Weight = self.obs.log_weight(&xs, y).into();
        self.resampler.resampling(&w, &xs)
    }
}
