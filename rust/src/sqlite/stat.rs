
use rusqlite::Connection;

pub struct StatTS<'a> {
    table_name: String,
    conn: &'a Connection,
}

impl<'a> StatTS<'a> {
    pub fn new(conn: &'a Connection, postfix: &str) -> Self {
        let tb_name = generate_table_name(postfix);
        create_table(conn, &tb_name);
        StatTS {
            table_name: tb_name,
            conn: conn,
        }
    }
    pub fn table_name(&self) -> &str {
        &self.table_name
    }
    pub fn insert(&self, time: f64, rmse_b: f64, rmse_a: f64, std_b: f64, std_a: f64, bias: f64) {
        insert(time,
               rmse_b,
               rmse_a,
               std_b,
               std_a,
               bias,
               self.conn,
               &self.table_name);
    }
}

fn generate_table_name(postfix: &str) -> String {
    format!("_stat_ts_{}", postfix)
}

fn create_table(conn: &Connection, table_name: &str) {
    let sql = format!(r#"CREATE TABLE {} (
                           time REAL NOT NULL,
                           rmse_b REAL NOT NULL,
                           rmse_a REAL NOT NULL,
                           std_b REAL NOT NULL,
                           std_a REAL NOT NULL,
                           bias REAL NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create ensemble timeserise table");
}

fn insert(time: f64,
          rmse_b: f64,
          rmse_a: f64,
          std_b: f64,
          std_a: f64,
          bias: f64,
          conn: &Connection,
          table_name: &str) {
    let sql = format!("INSERT INTO {} values (?1, ?2, ?3, ?4, ?5, ?6);",
                      &table_name);
    conn.execute(&sql, &[&time, &rmse_b, &rmse_a, &std_b, &std_a, &bias]).expect("miss to insert stat");
}
