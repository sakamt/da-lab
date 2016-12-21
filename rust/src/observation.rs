
use ndarray::prelude::*;
use ndarray_linalg::prelude::*;
use rand::distributions::*;
use ndarray_rand::RandomExt;

use ensemble::*;


pub fn noise(rs: &M) -> V {
    let (n, _) = rs.size();
    let dist = Normal::new(0., 1.0);
    let d = Array::random(n, dist);
    rs.dot(&d)
}

/// Observation Operator
pub struct ObsOperator {
    h: M,
    rs: M,
}

impl ObsOperator {
    pub fn new(h: M, rs: M) -> Self {
        ObsOperator { h: h, rs: rs }
    }
    pub fn generate(&self, truth: &V) -> V {
        self.h.dot(truth) + noise(&self.rs)
    }
    pub fn info_gain(&self) -> M {
        // FIXME
        self.rs.dot(&self.rs)
    }
}
