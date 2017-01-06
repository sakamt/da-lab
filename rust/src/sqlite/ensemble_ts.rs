
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

    pub fn insert(&self, time: f64, forecasted: &str, analysized: &str) {
        insert(time, forecasted, analysized, self.conn, &self.table_name);
    }

    pub fn register(&self, dt: f64, k: usize, truth_id: i64, observation_id: i64) -> i64 {
        register(dt, k, truth_id, observation_id, &self.table_name, self.conn)
    }
}

fn generate_table_name(postfix: &str) -> String {
    format!("_ensemble_ts_{}", postfix)
}

fn create_table(conn: &Connection, table_name: &str) {
    let sql = format!(r#"CREATE TABLE {} (
                           time REAL NOT NULL,
                           forecasted TEXT NOT NULL,
                           analysized TEXT NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create ensemble timeserise table");
}

fn register(dt: f64, k: usize, truth_id: i64, observation_id: i64, table_name: &str, conn: &Connection) -> i64 {
    let sql = "INSERT INTO ensemble (table_name, dt, K, truth_id, observation_id) VALUES (?1, ?2, ?3, ?4, ?5)";
    conn.execute(&sql,
                 &[&table_name, &dt, &(k as i64), &truth_id, &observation_id])
        .expect("Failed to register ensemble timeserise");
    conn.last_insert_rowid()
}

fn insert(time: f64, forecasted: &str, analysized: &str, conn: &Connection, table_name: &str) {
    let sql = format!("INSERT INTO {} values (?1, ?2, ?3);", &table_name);
    conn.execute(&sql, &[&time, &forecasted, &analysized]).expect("miss to insert ensemble_ts");
}
