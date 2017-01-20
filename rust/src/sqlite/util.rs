
use time;
use uuid::Uuid;

pub fn now_str() -> String {
    let tm = time::now();
    time::strftime("%Y%m%d_%H%M%S", &tm).unwrap()
}

pub fn generate_table_name(prefix: &str) -> String {
    format!("{}_{}", prefix, Uuid::new_v4())
}
