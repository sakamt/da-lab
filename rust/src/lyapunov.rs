
use super::types::{V, M};
use ndarray::{stack, ShapeError};
use ndarray::prelude::*;
use ndarray_linalg::prelude::*;

pub fn hstack(xs: &Vec<V>) -> Result<M, ShapeError> {
    let views: Vec<_> = xs.iter()
        .map(|x| {
            let n = x.len();
            x.view().into_shape((n, 1)).unwrap()
        })
        .collect();
    stack(Axis(1), &views)
}

pub fn jacobi_cached<F>(f: &F, x0: &V, alpha: f64) -> M
    where F: Fn(V) -> V
{
    let n = x0.len();
    let nx0 = x0.norm();
    let fx0 = f(x0.clone());
    hstack(&(0..n)
            .map(|i| {
                let mut ei = Array::zeros(n);
                ei[i] = nx0 * alpha;
                f(ei + x0) - &fx0
            })
            .collect())
        .unwrap() / (nx0 * alpha)
}
