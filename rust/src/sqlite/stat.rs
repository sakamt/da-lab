
use rusqlite::Connection;
use super::super::{io, stat};
use super::util;

pub fn create_table(conn: &Connection, table_name: &str) {
    let sql = format!(r#"CREATE TABLE '{}' (
                           time REAL NOT NULL,
                           rmse_f REAL NOT NULL,
                           rmse_a REAL NOT NULL,
                           std_f REAL NOT NULL,
                           std_a REAL NOT NULL,
                           bias REAL NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create ensemble timeserise table");
}

pub fn insert(time: f64, st: &stat::Stat, conn: &Connection, table_name: &str) {
    let sql = format!("INSERT INTO '{}' values (?1, ?2, ?3, ?4, ?5, ?6);",
                      &table_name);
    conn.execute(&sql,
                 &[&time, &st.rmse_f, &st.rmse_a, &st.std_f, &st.std_a, &st.bias])
        .expect("miss to insert stat");
}

impl io::StatStorage for Connection {
    type Key = String;
    fn save_stat(&self, stat: &[(f64, stat::Stat)]) -> Self::Key {
        let table_name = util::generate_table_name("stat");
        create_table(self, &table_name);
        for &(time, ref st) in stat.iter() {
            insert(time, st, self, &table_name);
        }
        table_name
    }
}
