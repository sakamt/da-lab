
#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate log;
extern crate rustc_serialize;
extern crate serde;
extern crate rmp_serialize;

extern crate rand;
extern crate float_cmp;
extern crate num_traits;

extern crate ndarray;
extern crate ndarray_linalg;
extern crate ndarray_odeint;
extern crate ndarray_rand;
extern crate itertools;

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

// for study
pub mod bias_correct;

pub fn select_analyzer(method_name: &str, setting: da::Setting) -> Box<da::EnsembleAnalyzer> {
    match method_name {
        "etkf" => Box::new(etkf::ETKF::from(setting)),
        "enkf" => Box::new(enkf::EnKF::from(setting)),
        "mpf" => Box::new(mpf::MPF::from(setting)),
        _ => panic!("unsupported method"),
    }
}
