use super::da::{EnsembleAnalyzer, EnsembleForecaster};
use super::method::mpf;
use super::stat;
use super::types::*;
use super::weight;

use std::mem;

fn correct_bias(mut xs: &mut Ensemble, truth: &V) {
    let dev = stat::mean(&xs) - truth;
    for x in xs.iter_mut() {
        *x = &*x - &dev;
    }
}

fn shake_ensemble(xs: &Ensemble) -> Ensemble {
    let res = mpf::MergeResampler::default();
    let w = weight::Weight::uniform(xs.len());
    res.resampling(&w, xs)
}
