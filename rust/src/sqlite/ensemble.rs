
use rusqlite::Connection;
use super::super::types::Ensemble;

pub fn save_ensemble(xs: &Ensemble, conn: &Connection, postfix: &str) -> String {

    let table_name = generate_table_name(postfix);
    create_table(conn, &table_name);
    insert(xs, conn, &table_name);
    table_name
}

pub fn generate_table_name(postfix: &str) -> String {
    format!("_ensemble_{}", postfix)
}

pub fn create_table(conn: &Connection, table_name: &str) {
    let sql = format!(r#"CREATE TABLE {} (
                           X REAL NOT NULL,
                           Y REAL NOT NULL,
                           Z REAL NOT NULL
                         );"#,
                      table_name);
    conn.execute(&sql, &[]).expect("Fail to create ensemble table");
}

pub fn insert(xs: &Ensemble, conn: &Connection, table_name: &str) {
    let sql = format!("INSERT INTO {} values (?1, ?2, ?3);", &table_name);
    for x in xs.iter() {
        conn.execute(&sql, &[&x[0], &x[1], &x[2]]).expect("miss to insert ensmble member");
    }
}
