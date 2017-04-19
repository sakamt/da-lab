
use super::types::*;
use super::da::{EnsembleForecaster, EnsembleAnalyzer};
use super::stat;
use super::mpf;
use super::weight;

use std::mem;

pub fn series<'a, F, A>(forecaster: &'a F,
                        analyzer: &'a A,
                        xs0: Ensemble,
                        obs: &'a Vec<V>,
                        truth: &'a Vec<V>,
                        correct: bool,
                        shake: bool)
                        -> Box<Iterator<Item = (Ensemble, Ensemble)> + 'a>
    where F: EnsembleForecaster + ?Sized,
          A: EnsembleAnalyzer + ?Sized
{
    Box::new(obs.iter().zip(truth.iter()).scan(xs0, move |xs, (y, t)| {
        let xs_a = analyzer.analysis(xs.clone(), y);
        let mut xs_a_new = if shake {
            shake_ensemble(&xs_a)
        } else {
            xs_a.clone()
        };
        if correct {
            correct_bias(&mut xs_a_new, t);
        }
        let xs_f = forecaster.forecast(xs_a_new);
        let xs_f_pre: Ensemble = mem::replace(xs, xs_f);
        Some((xs_f_pre, xs_a))
    }))
}

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
