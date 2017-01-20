
use ndarray::{Array, Ix1, Ix2};

pub type V = Array<f64, Ix1>;
pub type M = Array<f64, Ix2>;
pub type Ensemble = Vec<V>;

pub type Truth = Vec<V>;
pub type Observation = Vec<V>;
