
extern crate aics_da;
extern crate rusqlite;

use rusqlite::Connection;

fn main() {
    let date = aics_da::sql::now_str();
    println!("date = {:?}", &date);
    let conn = Connection::open("test.db").unwrap();
    aics_da::sql::create_ensemble_table(&conn, &date);
}
