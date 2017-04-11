
use super::types::*;
use super::da::{EnsembleForecaster, EnsembleAnalyzer};
use super::stat;

use std::mem;

pub fn iterate<F, A>(forecaster: &F, analyzer: &A, mut xs: &mut Ensemble, y: &V, t: &V) -> (Ensemble, Ensemble)
    where F: EnsembleForecaster + ?Sized,
          A: EnsembleAnalyzer + ?Sized
{
    let xs_a = analyzer.analysis(xs.clone(), y);
    let xs_f = forecaster.forecast(remove_bias(xs_a.clone(), t));
    let xs_f_pre: Ensemble = mem::replace(xs, xs_f);
    (xs_f_pre, xs_a)
}

pub fn series<'a, F, A>(forecaster: &'a F,
                        analyzer: &'a A,
                        xs0: Ensemble,
                        obs: &'a Vec<V>,
                        truth: &'a Vec<V>)
                        -> Box<Iterator<Item = (Ensemble, Ensemble)> + 'a>
    where F: EnsembleForecaster + ?Sized,
          A: EnsembleAnalyzer + ?Sized
{
    Box::new(obs.iter().zip(truth.iter()).scan(xs0,
                                               move |xs, (y, t)| Some(iterate(forecaster, analyzer, xs, y, t))))
}

fn remove_bias(mut xs: Ensemble, truth: &V) -> Ensemble {
    let dev = stat::mean(&xs) - truth;
    for x in xs.iter_mut() {
        *x = &*x - &dev;
    }
    xs
}
