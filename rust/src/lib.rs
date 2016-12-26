
extern crate rand;
extern crate ndarray;
extern crate ndarray_linalg;
extern crate ndarray_odeint;
extern crate ndarray_rand;
extern crate rmp_serialize;
extern crate rustc_serialize;
extern crate num_traits;

pub mod types;
pub mod einsum;
pub mod io;
pub mod stat;
pub mod observation;
pub mod weight;

// data assimilation
pub mod da;
pub mod mpf;
pub mod enkf;

// models
pub mod l63;
