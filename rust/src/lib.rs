
extern crate rand;
extern crate ndarray;
extern crate ndarray_linalg;
extern crate ndarray_odeint;
extern crate ndarray_rand;
extern crate rmp_serialize;
extern crate rustc_serialize;
extern crate num_traits;
extern crate rusqlite;
extern crate time;

pub mod types;
pub mod linalg;
pub mod io;
pub mod sqlite;
pub mod stat;
pub mod observation;
pub mod weight;
pub mod lyapunov;

// data assimilation
pub mod da;
pub mod mpf;
pub mod enkf;

// models
pub mod l63;
