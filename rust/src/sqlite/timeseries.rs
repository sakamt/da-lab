
use ndarray::prelude::*;
use rusqlite::Connection;

use super::super::types::V;
use super::super::da;

pub fn save_truth(setting: &da::Setting, x_tl: &Vec<V>, conn: &Connection, postfix: &str) -> i64 {
    let dt = setting.dt;
    let duration = (setting.tau * setting.count) as f64 * setting.dt;
    let postfix = format!("truth_{}", postfix);
    let table_name = save_timeseries(dt, x_tl, conn, &postfix);
    register_truth(dt, duration, &table_name, conn)
}

pub fn save_observation(setting: &da::Setting, x_tl: &Vec<V>, truth_id: i64, conn: &Connection, postfix: &str) -> i64 {
    let postfix = format!("obs_{}", postfix);
    let table_name = save_timeseries(setting.dt, x_tl, conn, &postfix);
    register_observation(setting.dt * setting.tau as f64,
                         setting.r,
                         setting.count as i64,
                         truth_id,
                         &table_name,
                         conn)
}

fn save_timeseries(dt: f64, x_tl: &Vec<V>, conn: &Connection, postfix: &str) -> String {
    let table_name = generate_table_name(postfix);
    create_table(conn, &table_name);
    for (t, x) in x_tl.iter().enumerate() {
        insert(t as f64 * dt, x, conn, &table_name);
    }
    table_name
}

pub fn get_truth(id: i64, conn: &Connection) -> (f64, Vec<V>) {
    let (dt, tbname) = lookup_truth(id, conn);
    let v: Vec<_> = load_table(&tbname, conn).into_iter().map(|v| v.1).collect();
    (dt, v)
}

pub fn get_observation(id: i64, conn: &Connection) -> (f64, Vec<V>, i64) {
    let (dt, tbname, tid) = lookup_observation(id, conn);
    let v: Vec<_> = load_table(&tbname, conn).into_iter().map(|v| v.1).collect();
    (dt, v, tid)
}

fn load_table(table_name: &str, conn: &Connection) -> Vec<(f64, V)> {
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

pub fn lookup_truth(id: i64, conn: &Connection) -> (f64, String) {
    let mut st = conn.prepare("SELECT dt,table_name FROM truth WHERE id=?").unwrap();
    let (dt, tbname) = st.query_map(&[&id], |row| {
            let dt: f64 = row.get(0);
            let tbname: String = row.get(1);
            (dt, tbname)
        })
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    (dt, tbname)
}

pub fn lookup_observation(id: i64, conn: &Connection) -> (f64, String, i64) {
    let mut st = conn.prepare("SELECT dt,table_name,id FROM observation WHERE id=?").unwrap();
    let (dt, tbname, tid) = st.query_map(&[&id], |row| {
            let dt: f64 = row.get(0);
            let tbname: String = row.get(1);
            let tid: i64 = row.get(2);
            (dt, tbname, tid)
        })
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    (dt, tbname, tid)
}

fn register_truth(dt: f64, duration: f64, table_name: &str, conn: &Connection) -> i64 {
    conn.execute("INSERT INTO truth(table_name, dt, duration) VALUES (?1, ?2, ?3);",
                 &[&table_name, &dt, &duration])
        .expect("Failed to register truth");
    conn.last_insert_rowid()
}

fn register_observation(dt: f64, r: f64, count: i64, truth_id: i64, table_name: &str, conn: &Connection) -> i64 {
    conn.execute("INSERT INTO observation(table_name, dt, r, truth_id, count) VALUES (?1, ?2, ?3, ?4, ?5);",
                 &[&table_name, &dt, &r, &truth_id, &count])
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
