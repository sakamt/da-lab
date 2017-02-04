
use time;
use uuid::Uuid;
use rusqlite::Connection;

const INIT_SQL: &'static str = r#"
BEGIN;
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
COMMIT;
"#;

pub fn now_str() -> String {
    let tm = time::now();
    time::strftime("%Y%m%d_%H%M%S", &tm).unwrap()
}

/// generate table name using UUID v4 (rnadom UUID)
pub fn generate_table_name(prefix: &str) -> String {
    format!("{}_{}", prefix, Uuid::new_v4())
}

/// Create pre-defined Tables
pub fn init_db(conn: &Connection) {
    conn.execute_batch(&INIT_SQL).expect("Failed to init");
}

/// Open SQLite DB with initialization
pub fn open_with_init(dbname: &str) -> Connection {
    let db = Connection::open(dbname).expect("Failed to open DB");
    init_db(&db);
    db
}
