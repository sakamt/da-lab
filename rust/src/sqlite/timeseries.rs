
use rusqlite::Connection;
use super::super::types::V;

pub fn generate_table_name(postfix: &str) -> String {
    format!("_ts_{}", postfix)
}

pub fn create_table(conn: &Connection, table_name: &str) {
    let sql = format!(r#"CREATE TABLE {} (
                           time REAL NOT NULL,
                           X REAL NOT NULL,
                           Y REAL NOT NULL,
                           Z REAL NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create timeseries table");
}

pub fn insert(time: f64, x: &V, conn: &Connection, table_name: &str) {
    let sql = format!("INSERT INTO {} values (?1, ?2, ?3, ?4);", &table_name);
    conn.execute(&sql, &[&time, &x[0], &x[1], &x[2]]).expect("miss to insert snapshot");
}
