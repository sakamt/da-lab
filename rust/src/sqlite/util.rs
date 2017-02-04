
use std::path::Path;
use time;
use uuid::Uuid;
use rusqlite::Connection;

const INIT_SQL: &'static str = r#"
CREATE TABLE IF NOT EXISTS enkf (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  K INTEGER NOT NULL,
  tau INTEGER NOT NULL,
  count INTEGER NOT NULL,
  r REAL NOT NULL,
  dt REAL NOT NULL,
  truth_id INTEGER NOT NULL,
  observation_id INTEGER NOT NULL,
  ensemble_table TEXT NOT NULL,
  stat_table TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS truth (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  table_name TEXT NOT NULL,
  dt REAL NOT NULL,
  length INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS observation (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  table_name TEXT NOT NULL,
  dt REAL NOT NULL,
  tau INTEGER NOT NULL,
  count INTEGER NOT NULL,
  r REAL NOT NULL,
  truth_id INTEGER NOT NULL
);
"#;

pub fn now_str() -> String {
    let tm = time::now();
    time::strftime("%Y%m%d_%H%M%S", &tm).unwrap()
}

pub fn generate_table_name(prefix: &str) -> String {
    format!("{}_{}", prefix, Uuid::new_v4())
}

pub fn open_with_init(dbname: &str) -> Connection {
    let dbpath = Path::new(dbname);
    if dbpath.exists() {
        Connection::open(dbpath).expect("Failed to open DB")
    } else {
        let db = Connection::open(dbpath).expect("Failed to open DB");
        db.execute(&INIT_SQL, &[]).expect("Failed to init");
        db
    }
}
