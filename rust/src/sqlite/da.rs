
use rusqlite::Connection;
use super::super::da::Setting;

pub fn insert_enkf(setting: &Setting, ensemble: &str, truth: &str, observable: &str, conn: &Connection) -> i32 {
    conn.execute(&"INSERT INTO enkf(K, r, dt, tau, ensemble, truth, observable) values (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
                 &[&(setting.k as i64), &setting.r, &setting.dt, &(setting.tau as i64), &ensemble, &truth, &observable])
        .unwrap()
}
