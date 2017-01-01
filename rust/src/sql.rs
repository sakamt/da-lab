
use time;
use rusqlite::Connection;

pub fn now_str() -> String {
    let tm = time::now();
    time::strftime("%Y%m%d_%H%M%S", &tm).unwrap()
}

pub fn create_ensemble_table(conn: &Connection, postfix: &str) -> String {
    let table_name = format!("_ensemble_{}", postfix);
    let sql = format!(r#"CREATE TABLE {} (
                           id INTERGER PRIMARY KEY,
                           X REAL NOT NULL,
                           Y REAL NOT NULL,
                           Z REAL NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).unwrap();
    table_name
}

pub fn create_timeseries_table(conn: &Connection, postfix: &str) -> String {
    let table_name = format!("_ts_{}", postfix);
    let sql = format!(r#"CREATE TABLE {} (
                           id INTERGER PRIMARY KEY,
                           time REAL NOT NULL,
                           truth INTERGER,
                           observed INTERGER,
                           forecasted INTERGER,
                           analysized INTERGER,
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).unwrap();
    table_name
}
