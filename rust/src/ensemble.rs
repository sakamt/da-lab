
use ndarray::prelude::*;
use rand::distributions::*;
use ndarray_rand::RandomExt;
use einsum;

pub type V = Array<f64, Ix1>;
pub type M = Array<f64, Ix2>;
pub type Ensemble = Vec<V>;

#[derive(Clone, Debug)]
pub struct Weight(Vec<f64>);
#[derive(Clone, Debug)]
pub struct LogWeight(Vec<f64>);

impl Into<LogWeight> for Weight {
    fn into(self) -> LogWeight {
        LogWeight(self.0.into_iter().map(|x| x.ln()).collect())
    }
}

impl Weight {
    pub fn mean(&self, xs: &Ensemble) -> V {
        let n = xs[0].len();
        xs.iter().zip(self.0.iter()).fold(Array::zeros(n), |a, (b, w)| a + b * *w)
    }
    pub fn stat2(&self, xs: &Ensemble) -> (V, M) {
        let n = xs[0].len();
        let xm = self.mean(xs);
        let cov = xs.iter().zip(self.0.iter()).fold(Array::zeros((n, n)), |a, (b, w)| {
            let dx = b - &xm;
            a + *w * einsum::a_b__ab(&dx, &dx)
        });
        (xm, cov)
    }
}

impl Into<Weight> for LogWeight {
    fn into(self) -> Weight {
        let ws: Vec<f64> = self.0.into_iter().map(|x| x.exp()).collect();
        let total: f64 = ws.iter().sum();
        Weight(ws.into_iter().map(|x| x / total).collect())
    }
}

pub fn replica(x: &V, r: f64, k: usize) -> Ensemble {
    let n = x.len();
    let dist = Normal::new(0.0, 1.0);
    (0..k).map(|_| r * Array::random(n, dist) + x).collect()
}

pub fn mean(xs: &Ensemble) -> V {
    let k = xs.len();
    let n = xs[0].len();
    let mut v = Array::zeros(n);
    for x in xs.iter() {
        v = v + x;
    }
    v / k as f64
}

pub fn covar(xs: &Ensemble, ys: &Ensemble) -> M {
    let xs_m = mean(xs);
    let ys_m = mean(ys);
    let n = xs_m.len();
    let m = ys_m.len();
    let mut c = Array::zeros((n, m));
    for (x, y) in xs.iter().zip(ys.iter()) {
        let dx = x - &xs_m;
        let dy = y - &ys_m;
        c = c + einsum::a_b__ab(&dx, &dy);
    }
    c / (xs.len() - 1) as f64
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
