
extern crate ndarray;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;
extern crate rusqlite;

use docopt::Docopt;
use aics_da::*;
use aics_da::sqlite as sql;

const USAGE: &'static str = "
Generate truth of Lorenz63 model

Usage:
  l63_truth <setting> <db>
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_setting: String,
    arg_db: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);
    let mut conn = rusqlite::Connection::open(args.arg_db).unwrap();
    let tx = conn.transaction().unwrap();
    let truth = l63::generate_truth(&setting);
    let tid = sql::save_truth(setting.dt,
                              (setting.tau * setting.count) as f64 * setting.dt,
                              &truth,
                              &tx,
                              &sql::util::now_str());
    tx.commit().unwrap();
    println!("{}", tid);
}
