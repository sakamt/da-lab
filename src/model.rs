use ndarray::*;
use ndarray_odeint::*;

use super::types::V;

pub struct L63 {
    param: explicit::RK4<model::lorenz63::Lorenz63, f64>,
    step: usize,
}

impl<'a> TimeEvolution<OwnedRepr<f64>, Ix1> for &'a L63 {
    fn iterate(self, mut x: &mut V) -> &mut V {
        for _ in 0..self.step {
            self.param.iterate(&mut x);
        }
        x
    }
}
