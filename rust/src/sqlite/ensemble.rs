
use rusqlite::Connection;
use ndarray::arr1;
use super::super::types::*;
use super::super::io;
use super::util;

fn load(table_name: &str, conn: &Connection) -> Ensemble {
    let sql = format!("SELECT * FROM '{}';", table_name);
    let mut st = conn.prepare(&sql).unwrap();
    let data = st.query_map(&[], |row| arr1(&[row.get(0), row.get(1), row.get(2)]))
        .unwrap()
        .map(|v| v.unwrap())
        .collect();
    data
}

fn create_table(conn: &Connection, table_name: &str) {
    let sql = format!(r#"CREATE TABLE '{}' (
                           X REAL NOT NULL,
                           Y REAL NOT NULL,
                           Z REAL NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create ensemble table");
}

fn insert(xs: &Ensemble, conn: &Connection, table_name: &str) {
    let sql = format!("INSERT INTO '{}' values (?1, ?2, ?3);", &table_name);
    for x in xs.iter() {
        conn.execute(&sql, &[&x[0], &x[1], &x[2]]).expect("miss to insert ensmble member");
    }
}

mod series {
    use rusqlite::Connection;

    pub fn create_table(conn: &Connection, table_name: &str) {
        let sql = format!(r#"CREATE TABLE '{}' (
                           time REAL NOT NULL,
                           forecasted TEXT NOT NULL,
                           analysized TEXT NOT NULL
                         );"#,
                          table_name);
        conn.execute(&sql, &[]).expect("Fail to create ensemble timeserise table");
    }

    pub fn insert(time: f64, forecasted: &str, analysized: &str, conn: &Connection, table_name: &str) {
        let sql = format!("INSERT INTO '{}' values (?1, ?2, ?3);", &table_name);
        conn.execute(&sql, &[&time, &forecasted, &analysized]).expect("miss to insert ensemble_series");
    }

    pub fn load(table_name: &str, conn: &Connection) -> Vec<(f64, String, String)> {
        let sql = format!("SELECT * FROM '{}';", table_name);
        let mut st = conn.prepare(&sql).unwrap();
        let data = st.query_map(&[], |row| {
                let time: f64 = row.get(0);
                let forecasted: String = row.get(1);
                let analysized: String = row.get(2);
                (time, forecasted, analysized)
            })
            .unwrap()
            .map(|v| v.unwrap())
            .collect();
        data
    }
}

impl io::EnsembleStorage for Connection {
    type SeriesKey = String;
    type Key = String;
    fn save_ensemble(&self, xs: &Ensemble) -> Self::Key {
        let table_name = util::generate_table_name("ensemble");
        create_table(self, &table_name);
        insert(xs, self, &table_name);
        table_name
    }
    fn load_ensemble(&self, table_name: Self::Key) -> Ensemble {
        load(&table_name, self)
    }
    fn commit_ensemble_series(&self, series: &[(f64, Self::Key, Self::Key)]) -> Self::SeriesKey {
        let table_name = util::generate_table_name("ensemble_series");
        series::create_table(self, &table_name);
        for &(time, ref forecasted, ref analysized) in series.iter() {
            series::insert(time, &forecasted, &analysized, self, &table_name);
        }
        table_name
    }
    fn query_ensemble_series(&self, table_name: Self::SeriesKey) -> Vec<(f64, Self::Key, Self::Key)> {
        series::load(&table_name, self)
    }
}
