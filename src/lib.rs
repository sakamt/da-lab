
#[macro_use]
extern crate derive_new;
extern crate rustc_serialize;
extern crate serde;
extern crate rmp_serialize;

extern crate rand;
extern crate float_cmp;
extern crate num_traits;

extern crate ndarray;
extern crate ndarray_linalg;
extern crate ndarray_odeint;
extern crate itertools;

pub mod types;
pub mod linalg;
pub mod io;
pub mod stat;
pub mod observation;
pub mod weight;
pub mod da;
pub mod l63;
pub mod bias_correct;

pub fn select_analyzer(method_name: &str, setting: da::Setting) -> Box<da::EnsembleAnalyzer> {
    match method_name {
        "etkf" => Box::new(method::ETKF::from(setting)),
        "enkf" => Box::new(method::EnKF::from(setting)),
        "mpf" => Box::new(method::MPF::from(setting)),
        _ => panic!("unsupported method"),
    }
}
