//! Replica mean
//!
//! Outputs
//! -------
//! Sequential data will be saved in "$DATADIR/replica_mean/YYYY-MM-DD-HH:MM:SS/"
//! - setting.json
//! - truth.msg : sequence of true state
//! - rm00001.msg ... : msgpack of dictionary
//!   - time
//!   - state
//!   - me
//!   - rmse

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate ndarray_linalg;
extern crate aics_da;

use clap::App;
use ndarray_linalg::*;
use std::path::*;

use aics_da::*;
use aics_da::types::*;

fn replica_mean(truth: &Truth, out_dir: &Path, setting: &da::Setting) {
    //
}

fn main() {
    exec::init();
    let yaml = load_yaml!("replica_mean.yml");
    let m = App::from_yaml(yaml).get_matches();
    let out_dir = exec::ready_out_dir("replica_mean");
    let setting = exec::ready_setting(m.value_of("config"), &out_dir);
    let truth = exec::ready_truth(m.value_of("init"), m.value_of("truth"), &out_dir, &setting);
    replica_mean(&truth, &out_dir, &setting);
}
