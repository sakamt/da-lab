use ndarray::*;
use rand::*;
use rand::distributions::*;

use linalg::outer;
use types::*;

#[derive(Clone, Debug)]
pub struct Weight {
    weight: Vec<f64>,
}

impl From<Vec<f64>> for Weight {
    fn from(w: Vec<f64>) -> Weight {
        Weight { weight: w }
    }
}

impl Weight {
    pub fn uniform(n: usize) -> Self {
        vec![1.0 / n as f64; n].into()
    }
    pub fn random(n: usize) -> Self {
        let dist = Range::new(0.0, 1.0);
        let mut rng = thread_rng();
        let w = Weight { weight: (0..n).map(|_| dist.ind_sample(&mut rng)).collect() };
        w.normalized()
    }
    pub fn get_raw_weight(&self) -> &Vec<f64> {
        &self.weight
    }
    pub fn normalize(&mut self) {
        let sum: f64 = self.weight.iter().sum();
        for x in self.weight.iter_mut() {
            *x /= sum;
        }
    }
    pub fn normalized(mut self) -> Self {
        self.normalize();
        self
    }
    pub fn mean(&self, xs: &Ensemble) -> V {
        let n = xs[0].len();
        xs.iter().zip(self.weight.iter()).fold(Array::zeros(n), |a,
         (b, w)| {
            a + b * *w
        })
    }
    pub fn stat2(&self, xs: &Ensemble) -> (V, M) {
        let n = xs[0].len();
        let xm = self.mean(xs);
        let cov = xs.iter().zip(self.weight.iter()).fold(
            Array::zeros((n, n)),
            |a, (b, w)| {
                let dx = b - &xm;
                a + *w * outer(&dx, &dx)
            },
        );
        (xm, cov)
    }
    pub fn to_dist(&self) -> DiscreteDist {
        DiscreteDist {
            cumprob: self.weight
                .iter()
                .scan(0.0, |st, &x| {
                    *st = *st + x;
                    Some(*st)
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LogWeight {
    logweight: Vec<f64>,
}

impl From<Vec<f64>> for LogWeight {
    fn from(w: Vec<f64>) -> LogWeight {
        LogWeight { logweight: w }
    }
}

impl LogWeight {
    pub fn get_raw_logweight(&self) -> &Vec<f64> {
        &self.logweight
    }
    pub fn drop_mean(&mut self) {
        let n = self.logweight.len();
        let mean = self.logweight.iter().sum::<f64>() / n as f64;
        for x in self.logweight.iter_mut() {
            *x -= mean;
        }
    }
}

impl Into<LogWeight> for Weight {
    fn into(self) -> LogWeight {
        let mut lw = LogWeight { logweight: self.weight.into_iter().map(|x| x.ln()).collect() };
        lw.drop_mean();
        lw
    }
}

impl Into<Weight> for LogWeight {
    fn into(self) -> Weight {
        let w = Weight { weight: self.logweight.into_iter().map(|x| x.exp()).collect() };
        w.normalized()
    }
}

#[derive(Clone, Debug)]
pub struct DiscreteDist {
    cumprob: Vec<f64>,
}

fn searchsorted(a: f64, cumprob: &Vec<f64>) -> usize {
    match cumprob.binary_search_by(|v| v.partial_cmp(&a).expect("Couldn't compare values")) {
        Ok(idx) => idx,
        Err(idx) => idx,
    }
}

impl Sample<usize> for DiscreteDist {
    fn sample<R: Rng>(&mut self, rng: &mut R) -> usize {
        searchsorted(rng.next_f64(), &self.cumprob)
    }
}

impl IndependentSample<usize> for DiscreteDist {
    fn ind_sample<R: Rng>(&self, rng: &mut R) -> usize {
        searchsorted(rng.next_f64(), &self.cumprob)
    }
}
