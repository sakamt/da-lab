
#[macro_use]
extern crate derive_new;
extern crate rand;
extern crate ndarray;
extern crate ndarray_linalg;
extern crate ndarray_odeint;
extern crate ndarray_rand;
extern crate rmp_serialize;
extern crate rustc_serialize;
extern crate num_traits;
extern crate itertools;
extern crate float_cmp;

pub mod types;
pub mod settings;
pub mod linalg;
pub mod io;
pub mod stat;
pub mod observation;
pub mod weight;
pub mod lyapunov;

// data assimilation
pub mod da;
pub mod mpf;
pub mod enkf;
pub mod etkf;

// models
pub mod l63;
