
use time;
use rusqlite::Connection;
use super::types::Ensemble;

pub fn now_str() -> String {
    let tm = time::now();
    time::strftime("%Y%m%d_%H%M%S", &tm).unwrap()
}

pub fn save_ensemble(xs: &Ensemble, conn: &Connection, postfix: &str) -> String {
    let table_name = create_ensemble_table(conn, postfix);
    let sql = format!("INSERT INTO {} values (?1, ?2, ?3);", &table_name);
    for x in xs.iter() {
        conn.execute(&sql, &[&x[0], &x[1], &x[2]]).expect("missing insert");
    }
    table_name
}

pub fn create_ensemble_table(conn: &Connection, postfix: &str) -> String {
    let table_name = format!("_ensemble_{}", postfix);
    let sql = format!(r#"CREATE TABLE {} (
                           X REAL NOT NULL,
                           Y REAL NOT NULL,
                           Z REAL NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create ensemble table");
    table_name
}

pub fn create_timeseries_table(conn: &Connection, postfix: &str) -> String {
    let table_name = format!("_ts_{}", postfix);
    let sql = format!(r#"CREATE TABLE {} (
                           time REAL NOT NULL,
                           X REAL NOT NULL,
                           Y REAL NOT NULL,
                           Z REAL NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create timeseries table");
    table_name
}

pub fn create_ensemble_timeseries_table(conn: &Connection, postfix: &str) -> String {
    let table_name = format!("_ensemble_ts_{}", postfix);
    let sql = format!(r#"CREATE TABLE {} (
                           time REAL NOT NULL,
                           forecasted TEXT NOT NULL,
                           analysized TEXT NOT NULL,
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create ensemble timeserise table");
    table_name
}
