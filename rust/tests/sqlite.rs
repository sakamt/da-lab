
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
