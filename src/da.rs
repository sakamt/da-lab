
extern crate ndarray;
extern crate ndarray_odeint;

use self::ndarray::prelude::*;
use self::ndarray_odeint::*;
use einsum;

pub type V = Array<f64, Ix>;
pub type M = Array<f64, (Ix, Ix)>;
pub type Ensemble = Vec<V>;

fn teo(dt: f64, step: usize, mut x: V) -> V {
    let l = |y| lorenz63(10., 28., 8.0 / 3.0, y);
    let u = |y| rk4(&l, dt, y);
    for _ in 0..step {
        x = u(x);
    }
    x
}

/// calc mean and covariance matrix
pub fn stat2(xs: &Ensemble) -> (V, M) {
    let k = xs.len();
    let n = xs[0].len();
    let mut v = Array::zeros(n);
    for x in xs.iter() {
        v = v + x;
    }
    v /= k as f64;
    let mut m = Array::zeros((n, n));
    for x in xs.iter() {
        let dx = x - &v;
        m = m + einsum::a_b__ab(&dx, &dx);
    }
    m /= k as f64 - 1.0;
    (v, m)
}

pub fn forcast(xs: Ensemble, dt: f64, step: usize) -> Ensemble {
    xs.into_iter().map(|y| teo(dt, step, y)).collect()
}
