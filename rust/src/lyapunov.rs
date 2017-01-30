
use super::types::{V, M};
use ndarray::{Data, stack, ShapeError};
use ndarray::linalg::Dot;
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

pub struct Jacobian<'a, 'b, TEO>
    where TEO: 'a + Fn(V) -> V
{
    f: &'a TEO,
    x: &'b V,
    fx: V,
    alpha: f64,
}

pub trait NumDifferentiable: Sized + Fn(V) -> V {
    fn jacobian<'a, 'b>(&'a self, x: &'b V, alpha: f64) -> Jacobian<'a, 'b, Self>;
}

impl<TEO> NumDifferentiable for TEO
    where TEO: Fn(V) -> V
{
    fn jacobian<'a, 'b>(&'a self, x: &'b V, alpha: f64) -> Jacobian<'a, 'b, Self> {
        let fx = self(x.clone());
        Jacobian {
            f: self,
            x: x,
            fx: fx,
            alpha: alpha,
        }
    }
}

impl<'a, 'b, S, TEO> Dot<ArrayBase<S, Ix1>> for Jacobian<'a, 'b, TEO>
    where TEO: 'a + Fn(V) -> V,
          S: Data<Elem = f64>
{
    type Output = V;
    fn dot(&self, dx: &ArrayBase<S, Ix1>) -> V {
        let nrm = self.x.norm_l2().max(dx.norm_l2());
        let n = self.alpha / nrm;
        let x = n * dx + self.x;
        ((self.f)(x) - &self.fx) / n
    }
}

impl<'a, 'b, S, TEO> Dot<ArrayBase<S, Ix2>> for Jacobian<'a, 'b, TEO>
    where TEO: 'a + Fn(V) -> V,
          S: Data<Elem = f64>
{
    type Output = M;
    fn dot(&self, dxs: &ArrayBase<S, Ix2>) -> M {
        hstack(&dxs.axis_iter(Axis(1))
                .map(|dx| self.dot(&dx))
                .collect())
            .unwrap()
    }
}
