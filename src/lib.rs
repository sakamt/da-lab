//! da-lab: Data-Assimilation Laboratory

extern crate rand;
extern crate float_cmp;
extern crate num_traits;

extern crate ndarray;
extern crate ndarray_linalg;
extern crate ndarray_odeint;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate rmp;
extern crate rmp_serde;

// For executables
#[macro_use]
extern crate log;
extern crate time;
extern crate env_logger;
extern crate dotenv;
extern crate uuid;

pub mod types;
pub mod linalg;
pub mod io;
pub mod stat;
pub mod observation;
pub mod weight;
pub mod da;
pub mod model;
pub mod method;
pub mod task;
pub mod bias;
