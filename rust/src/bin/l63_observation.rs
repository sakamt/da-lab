
extern crate ndarray;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;
extern crate rusqlite;

use docopt::Docopt;
use aics_da::*;
use aics_da::sqlite as sql;

const USAGE: &'static str = "
Generate observation of Lorenz63 model

Usage:
  l63_observation <setting> <db> [--tid=<truth_id>]
  l63_observation (-h|--help)

Options:
  -h --help        Show this screen.
  --tid=<truth_id>  truth id [default: 0].
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_tid: i64,
    arg_setting: String,
    arg_db: String,
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);
    let postfix = sql::util::now_str();
    let mut conn = rusqlite::Connection::open(args.arg_db).unwrap();
    let tx = conn.transaction().unwrap();
    let (dt, truth, tid) = if args.flag_tid == 0 {
        let truth = l63::generate_truth(&setting);
        let tid = sql::save_truth(&setting, &truth, &tx, &postfix);
        (setting.dt, truth, tid)
    } else {
        let (dt, truth) = sql::timeseries::get_truth(args.flag_tid, &tx);
        (dt, truth, args.flag_tid)
    };
    let obs_op = observation::ObsOperator::isotropic(3, setting.r);
    let obs = obs_op.generate(&setting, &truth, dt);
    let oid = sql::save_observation(&setting, &obs, tid, &tx, &postfix);
    tx.commit().unwrap();
    println!("{}", oid);
}
