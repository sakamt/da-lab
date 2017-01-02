
use time;

pub fn now_str() -> String {
    let tm = time::now();
    time::strftime("%Y%m%d_%H%M%S", &tm).unwrap()
}
