
extern crate aics_da;
extern crate ndarray;
extern crate rusqlite;

use aics_da::*;
use rusqlite::Connection;
use ndarray::prelude::*;

fn main() {
    let x0 = arr1(&[1.0, 0.0, 0.0]);
    let xs0 = da::replica(&x0, 1.0, 10000);
    let mut conn = Connection::open("test.db").unwrap();
    let tx = conn.transaction().unwrap();
    let now = aics_da::sql::now_str();
    let tb_name = aics_da::sql::save_ensemble(&xs0, &tx, &now);
    println!("table name = {:?}", &tb_name);
    tx.commit().unwrap();
}
