
extern crate ndarray;
extern crate rustc_serialize;
extern crate docopt;
extern crate aics_da;
extern crate rusqlite;

use docopt::Docopt;
use aics_da::*;
use aics_da::io::SeriesStorage;
use aics_da::settings::Induce;

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

fn observation(args: Args, setting: da::Setting, conn: &rusqlite::Connection) {
    let (dt, truth, tid) = if args.flag_tid == 0 {
        let truth = l63::generate_truth(&setting);
        let tid = conn.save_truth(&setting.induce(), &truth);
        (setting.dt, truth, tid)
    } else {
        let (setting, truth) = conn.load_truth(args.flag_tid);
        (setting.dt, truth, args.flag_tid)
    };
    let obs_op = observation::LinearNormal::isotropic(3, setting.r);
    let obs = observation::eval_series(&obs_op, &setting, &truth, dt);
    conn.save_observation(&setting.induce(), tid, &obs);
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let setting: da::Setting = io::read_json(&args.arg_setting);
    let mut conn = sqlite::open_with_init(&args.arg_db);
    let tx = conn.transaction().unwrap();
    observation(args, setting, &tx);
    tx.commit().unwrap();
}
