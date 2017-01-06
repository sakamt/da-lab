
use rusqlite::Connection;
use super::super::da::Setting;

pub fn insert_enkf(setting: &Setting, truth_id: i64, observable_id: i64, ensemble_id: i64, conn: &Connection) -> i32 {
    conn.execute(&"INSERT INTO enkf(K, tau, count, r, dt, truth_id, observation_id, ensemble_id) values (?1, ?2, ?3, \
                   ?4, ?5, ?6, ?7, ?8);",
                 &[&(setting.k as i64),
                   &(setting.tau as i64),
                   &(setting.count as i64),
                   &setting.r,
                   &setting.dt,
                   &truth_id,
                   &observable_id,
                   &ensemble_id])
        .expect("Missing to insert EnKF result")
}
