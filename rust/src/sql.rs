
use time;
use rusqlite::Connection;
use super::types::Ensemble;

pub fn now_str() -> String {
    let tm = time::now();
    time::strftime("%Y%m%d_%H%M%S", &tm).unwrap()
}

pub fn save_ensemble(xs: &Ensemble, conn: &mut Connection) -> String {
    let now = now_str();
    let tb_name = create_ensemble_table(conn, &now);
    let sql = format!("INSERT INTO {} values (?1, ?2, ?3);", &tb_name);
    let tx = conn.transaction().unwrap();
    for x in xs.iter() {
        tx.execute(&sql, &[&x[0], &x[1], &x[2]]).expect("missing insertion");
    }
    tx.commit().expect("commit");
    tb_name
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
                           id INTERGER PRIMARY KEY,
                           time REAL NOT NULL,
                           truth INTERGER,
                           observed INTERGER,
                           forecasted INTERGER,
                           analysized INTERGER,
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create timeserise table");
    table_name
}
