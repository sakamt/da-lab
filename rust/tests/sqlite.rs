
extern crate ndarray;
extern crate aics_da;
extern crate rusqlite;

use ndarray::arr1;
use aics_da::*;
use aics_da::io::*;

#[test]
fn io_truth() {
    let conn = sqlite::open_with_init("test_io_truth.db");
    let truth = vec![arr1(&[1.0, 0.0, 0.0]), arr1(&[0.0, 1.0, 0.0])];
    let setting = settings::Truth {
        dt: 0.01,
        length: 100,
    };
    let tid = conn.save_truth(&setting, &truth);
    let _ = conn.load_truth(tid);
}

#[test]
fn io_observation() {
    let conn = sqlite::open_with_init("test_io_observation.db");
    let obs = vec![arr1(&[1.0, 0.0, 0.0]), arr1(&[0.0, 1.0, 0.0])];
    let tid = 1;
    let setting = settings::Observation {
        dt: 0.01,
        tau: 8,
        count: 10,
        r: 1.0,
    };
    let oid = conn.save_observation(&setting, tid, &obs);
    let _ = conn.load_observation(oid);
}

#[test]
fn io_ensemble() {
    let conn = sqlite::open_with_init("test_io_ensemble.db");
    let ens = vec![arr1(&[1.0, 0.0, 0.0]), arr1(&[0.0, 1.0, 0.0])];
    let tbname = conn.save_ensemble(&ens);
    let _ = conn.load_ensemble(tbname);
}
