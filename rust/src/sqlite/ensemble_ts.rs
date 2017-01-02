
use rusqlite::Connection;

pub fn generate_table_name(postfix: &str) -> String {
    format!("_ensemble_ts_{}", postfix)
}

pub fn create_table(conn: &Connection, table_name: &str) {
    let sql = format!(r#"CREATE TABLE {} (
                           time REAL NOT NULL,
                           forecasted TEXT NOT NULL,
                           analysized TEXT NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create ensemble timeserise table");
}

pub fn insert(time: f64, forecasted: &str, analysized: &str, conn: &Connection, table_name: &str) {
    let sql = format!("INSERT INTO {} values (?1, ?2, ?3);", &table_name);
    conn.execute(&sql, &[&time, &forecasted, &analysized]).expect("miss to insert ensemble_ts");
}
