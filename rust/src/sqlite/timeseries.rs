
use rusqlite::Connection;
use super::super::types::V;

pub fn save_truth(dt: f64, x_tl: &Vec<V>, conn: &Connection, postfix: &str) -> i64 {
    let table_name = save_timeseries(dt, x_tl, conn, postfix);
    register_truth(dt, &table_name, conn)
}

pub fn save_observation(dt: f64, x_tl: &Vec<V>, truth_id: i64, conn: &Connection, postfix: &str) -> i64 {
    let table_name = save_timeseries(dt, x_tl, conn, postfix);
    register_observation(dt, truth_id, &table_name, conn)
}

fn save_timeseries(dt: f64, x_tl: &Vec<V>, conn: &Connection, postfix: &str) -> String {
    let table_name = generate_table_name(postfix);
    create_table(conn, &table_name);
    for (t, x) in x_tl.iter().enumerate() {
        insert(t as f64 * dt, x, conn, &table_name);
    }
    table_name
}

fn generate_table_name(postfix: &str) -> String {
    format!("_ts_{}", postfix)
}

fn register_truth(dt: f64, table_name: &str, conn: &Connection) -> i64 {
    conn.execute("INSERT INTO truth(table_name, dt) VALUES (?1, ?2);",
                 &[&table_name, &dt])
        .expect("Failed to register truth");
    conn.last_insert_rowid()
}

fn register_observation(dt: f64, truth_id: i64, table_name: &str, conn: &Connection) -> i64 {
    conn.execute("INSERT INTO observation(table_name, dt, truth_id) VALUES (?1, ?2, ?3);",
                 &[&table_name, &dt, &truth_id])
        .expect("Failed to register observation");
    conn.last_insert_rowid()
}

fn create_table(conn: &Connection, table_name: &str) {
    let sql = format!(r#"CREATE TABLE {} (
                           time REAL NOT NULL,
                           X REAL NOT NULL,
                           Y REAL NOT NULL,
                           Z REAL NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create timeseries table");
}

fn insert(time: f64, x: &V, conn: &Connection, table_name: &str) {
    let sql = format!("INSERT INTO {} values (?1, ?2, ?3, ?4);", &table_name);
    conn.execute(&sql, &[&time, &x[0], &x[1], &x[2]]).expect("miss to insert snapshot");
}
