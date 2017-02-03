
use rusqlite::Connection;

pub struct EnsembleTS<'a> {
    table_name: String,
    conn: &'a Connection,
}

impl<'a> EnsembleTS<'a> {
    pub fn new(conn: &'a Connection, postfix: &str) -> Self {
        let tb_name = generate_table_name(postfix);
        create_table(conn, &tb_name);
        EnsembleTS {
            table_name: tb_name,
            conn: conn,
        }
    }
    pub fn table_name(&self) -> &str {
        &self.table_name
    }
    pub fn insert(&self, time: f64, forecasted: &str, analysized: &str) {
        insert(time, forecasted, analysized, self.conn, &self.table_name);
    }
}

fn generate_table_name(postfix: &str) -> String {
    format!("_ensemble_series_{}", postfix)
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
    conn.execute(&sql, &[&time, &forecasted, &analysized]).expect("miss to insert ensemble_series");
}
