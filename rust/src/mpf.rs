
use ensemble::*;
use weight::*;
use num_traits::float::Float;

/// Coefficient for merge resampling
#[derive(Clone, Debug)]
pub struct MergeCoef(Vec<f64>);

impl Default for MergeCoef {
    fn default() -> Self {
        MergeCoef(vec![3.0 / 4.0, (13.0.sqrt() + 1.0) / 8.0, -(13.0.sqrt() - 1.0) / 8.0])
    }
    // fn random(n: usize) -> Self {}
}

impl MergeCoef {
    pub fn resampling(_: &Weight, xs: Ensemble) -> Ensemble {
        // TODO
        xs
    }
}
