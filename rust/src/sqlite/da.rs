
use rusqlite::Connection;
use super::super::da::Setting;

pub fn insert_enkf(setting: &Setting, ensemble: &str, truth: &str, observable: &str, conn: &Connection) -> i32 {
    conn.execute(&"INSERT INTO enkf(K, tau, count, r, dt, ensemble, truth, observable) values (?1, ?2, ?3, ?4, ?5, \
                   ?6, ?7, ?8);",
                 &[&(setting.k as i64),
                   &(setting.tau as i64),
                   &(setting.count as i64),
                   &setting.r,
                   &setting.dt,
                   &ensemble,
                   &truth,
                   &observable])
        .expect("Missing to insert EnKF result")
}
