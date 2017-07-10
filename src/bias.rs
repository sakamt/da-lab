use super::mpf;
use super::stat;
use super::types::*;
use super::weight;

pub fn correct(mut xs: &mut Ensemble, truth: &V) {
    let dev = stat::mean(&xs) - truth;
    for x in xs.iter_mut() {
        *x = &*x - &dev;
    }
}

pub fn shake(xs: &Ensemble) -> Ensemble {
    let res = mpf::MergeResampler::default();
    let w = weight::Weight::uniform(xs.len());
    res.resampling(&w, xs)
}
