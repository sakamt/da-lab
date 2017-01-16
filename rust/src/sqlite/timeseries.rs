
use ndarray::prelude::*;
use rusqlite::Connection;
use super::super::types::V;

pub fn get_truth(id: i64, conn: &Connection) -> (f64, Vec<V>) {
    let mut st = conn.prepare("SELECT * WHERE id=?1 FROM truth").unwrap();
    let mut rows = st.query(&[&id]).unwrap();
    let row = rows.next().unwrap().unwrap();
    let table_name: String = row.get(1);
    let dt = row.get(2);
    let data = load_table(&table_name, conn);
    (dt, data.into_iter().map(|(_, v)| v).collect())
}

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

pub fn load_truth(truth_id: i64, conn: &Connection) -> (f64, Vec<V>) {
    let mut st = conn.prepare("SELECT dt,table_name FROM truth WHERE id=?").unwrap();
    let (dt, tbname) = st.query_map(&[&truth_id], |row| {
            let dt: f64 = row.get(0);
            let tbname: String = row.get(1);
            (dt, tbname)
        })
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    let v: Vec<_> = load_table(&tbname, conn).into_iter().map(|v| v.1).collect();
    (dt, v)
}

pub fn load_table(table_name: &str, conn: &Connection) -> Vec<(f64, V)> {
    let sql = format!("SELECT * FROM {} ORDER BY time", table_name);
    let mut st = conn.prepare(&sql).unwrap();
    let data = st.query_map(&[],
                   |row| (row.get(0), arr1(&[row.get(1), row.get(2), row.get(3)])))
        .unwrap()
        .map(|v: Result<(f64, _), _>| v.unwrap())
        .collect();
    data
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
