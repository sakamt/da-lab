
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
