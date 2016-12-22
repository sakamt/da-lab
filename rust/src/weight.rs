
use ndarray::prelude::*;
use ensemble::*;
use einsum::a_b__ab;
use rand::*;
use rand::distributions::*;


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
    pub fn mean(&self, xs: &Ensemble) -> V {
        let n = xs[0].len();
        xs.iter().zip(self.weight.iter()).fold(Array::zeros(n), |a, (b, w)| a + b * *w)
    }
    pub fn stat2(&self, xs: &Ensemble) -> (V, M) {
        let n = xs[0].len();
        let xm = self.mean(xs);
        let cov = xs.iter().zip(self.weight.iter()).fold(Array::zeros((n, n)), |a, (b, w)| {
            let dx = b - &xm;
            a + *w * a_b__ab(&dx, &dx)
        });
        (xm, cov)
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

impl Into<LogWeight> for Weight {
    fn into(self) -> LogWeight {
        LogWeight { logweight: self.weight.into_iter().map(|x| x.ln()).collect() }
    }
}

impl Into<Weight> for LogWeight {
    fn into(self) -> Weight {
        let ws: Vec<f64> = self.logweight.into_iter().map(|x| x.exp()).collect();
        let total: f64 = ws.iter().sum();
        Weight { weight: ws.into_iter().map(|x| x / total).collect() }
    }
}

#[derive(Clone, Debug)]
pub struct DiscreteDist {
    cumprob: Vec<f64>,
}

impl Into<DiscreteDist> for Weight {
    fn into(self) -> DiscreteDist {
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

impl Sample<usize> for DiscreteDist {
    fn sample<R: Rng>(&mut self, rng: &mut R) -> usize {
        let a = rng.next_f64();
        match self.cumprob.binary_search_by(|v| v.partial_cmp(&a).expect("Couldn't compare values")) {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
    }
}
