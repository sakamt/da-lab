
use ndarray::prelude::*;
use rusqlite::Connection;

use super::super::types::V;
use super::super::settings;

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

pub fn save(dt: f64, ts: &Vec<V>, conn: &Connection, table_name: &str) {
    let sql = format!("INSERT INTO {} values (?1, ?2, ?3, ?4);", &table_name);
    let mut st = conn.prepare(&sql).unwrap();
    for (t, x) in ts.iter().enumerate() {
        let time = t as f64 * dt;
        st.execute(&[&time, &x[0], &x[1], &x[2]]).expect("miss to insert snapshot");
    }
}

pub fn load(table_name: &str, conn: &Connection) -> Vec<V> {
    let sql = format!("SELECT * FROM {} ORDER BY time", table_name);
    let mut st = conn.prepare(&sql).unwrap();
    let data = st.query_map(&[], |row| arr1(&[row.get(1), row.get(2), row.get(3)]))
        .unwrap()
        .map(|v| v.unwrap())
        .collect();
    data
}

pub fn register_truth(setting: &settings::Truth, conn: &Connection, table_name: &str) -> i64 {
    conn.execute("INSERT INTO truth(table_name, dt, length) VALUES (?1, ?2, ?3);",
                 &[&table_name, &setting.dt, &(setting.length as i64)])
        .expect("Failed to register truth");
    conn.last_insert_rowid()
}

pub fn lookup_truth(id: i64, conn: &Connection) -> (settings::Truth, String) {
    let mut st = conn.prepare("SELECT dt,length,table_name FROM truth WHERE id=?").unwrap();
    let (setting, tbname) = st.query_map(&[&id], |row| {
            let setting = settings::Truth {
                dt: row.get(0),
                length: row.get::<_, i64>(1) as usize,
            };
            let tbname: String = row.get(2);
            (setting, tbname)
        })
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    (setting, tbname)
}

pub fn register_observation(setting: &settings::Observation,
                            truth_id: i64,
                            conn: &Connection,
                            table_name: &str)
                            -> i64 {
    conn.execute("INSERT INTO observation(table_name, dt, tau, count, r, truth_id) VALUES (?1, ?2, ?3, ?4, ?5);",
                 &[&table_name, &setting.dt, &(setting.tau as i64), &(setting.count as i64), &setting.r, &truth_id])
        .expect("Failed to register observation");
    conn.last_insert_rowid()
}

pub fn lookup_observation(id: i64, conn: &Connection) -> (settings::Observation, i64, String) {
    let mut st = conn.prepare("SELECT dt,tau,count,r,table_name,truth_id FROM observation WHERE id=?").unwrap();
    let (setting, tbname, tid) = st.query_map(&[&id], |row| {
            let setting = settings::Observation {
                dt: row.get(0),
                tau: row.get::<_, i64>(1) as usize,
                count: row.get::<_, i64>(2) as usize,
                r: row.get(3),
            };
            let tbname: String = row.get(4);
            let tid: i64 = row.get(5);
            (setting, tbname, tid)
        })
        .unwrap()
        .next()
        .unwrap()
        .unwrap();
    (setting, tid, tbname)
}
