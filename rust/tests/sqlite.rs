
extern crate ndarray;
extern crate aics_da;
extern crate rusqlite;

use ndarray::arr1;
use aics_da::*;
use aics_da::io::*;

#[test]
fn io_truth() {
    let conn = rusqlite::Connection::open("test.db").unwrap();
    let storage = sqlite::SqliteStorage::new(&conn);
    let truth = vec![arr1(&[1.0, 0.0, 0.0]), arr1(&[0.0, 1.0, 0.0])];
    let setting = settings::Truth {
        dt: 0.01,
        length: 100,
    };
    let tid = storage.save_truth(&setting, &truth);
    let _ = storage.load_truth(tid);
}

#[test]
fn io_observation() {
    let conn = rusqlite::Connection::open("test.db").unwrap();
    let storage = sqlite::SqliteStorage::new(&conn);
    let obs = vec![arr1(&[1.0, 0.0, 0.0]), arr1(&[0.0, 1.0, 0.0])];
    let tid = 1;
    let setting = settings::Observation {
        dt: 0.01,
        tau: 8,
        count: 10,
        r: 1.0,
    };
    let oid = storage.save_observation(&setting, tid, &obs);
    let _ = storage.load_observation(oid);
}

#[test]
fn io_ensemble() {
    let conn = rusqlite::Connection::open("test.db").unwrap();
    let storage = sqlite::SqliteStorage::new(&conn);
    let ens = vec![arr1(&[1.0, 0.0, 0.0]), arr1(&[0.0, 1.0, 0.0])];
    let tbname = storage.save_ensemble(&ens);
    let _ = storage.load_ensemble(tbname);
}
