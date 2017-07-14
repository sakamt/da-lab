//! Replica mean
//!
//! Outputs
//! -------
//! Sequential data will be saved in "$DATADIR/replica_mean/YYYY-MM-DD-HH:MM:SS/"
//! - setting.json
//! - truth.msg : sequence of true state
//! - rm00001.msg ... : msgpack of dictionary
//!   - time
//!   - state
//!   - me
//!   - rmse

#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;

extern crate ndarray_linalg;
extern crate aics_da;

use clap::App;
use ndarray_linalg::*;
use std::path::*;
use std::sync::Arc;

use aics_da::*;
use aics_da::types::*;

#[derive(Serialize)]
struct Output {
    time: f64,
    state: V,
    mev: V,
    rmse: f64,
}

fn replica_mean(truth: Truth, out_dir: PathBuf, setting: da::Setting) {
    let truth = Arc::new(truth);
    let num_replica = setting.replica.unwrap_or(100);

    let handlers: Vec<_> = (0..num_replica)
        .map(|_tid| {
            let truth = truth.clone();
            let setting = setting.clone();
            let (sender, receiver) = ::std::sync::mpsc::channel();
            let h = std::thread::spawn(move || {
                let obs = observation::generate_obs(&truth, &setting);
                let f = model::select_model(&setting);
                let a = da::select_analyzer(&setting);
                let mut xs = da::replica(&truth[0], setting.r, setting.k);
                for y in obs.iter() {
                    xs = a.analysis(xs, &y);
                    let xa = stat::mean(&xs);
                    xs = f.forecast(xs);
                    sender.send(xa).unwrap();
                }
            });
            (receiver, h)
        })
        .collect();

    for (m, t) in truth.iter().enumerate() {
        let xa_: Vec<_> = handlers
            .iter()
            .map(|&(ref rcv, _)| rcv.recv().unwrap())
            .collect();
        let xam = stat::mean(&xa_);
        let output = Output {
            time: (m * setting.tau) as f64 * setting.dt,
            state: t.clone(),
            mev: xam - t,
            rmse: xa_.iter().map(|x| (x - t).norm()).sum::<f64>() / num_replica as f64,
        };
        let out_fn = format!("rm{:05}.msg", t);
        io::save_msg(&output, out_dir.join(out_fn).to_str().unwrap());
    }

    for h in handlers {
        h.1.join().unwrap();
    }
}

fn main() {
    exec::init();
    let yaml = load_yaml!("replica_mean.yml");
    let m = App::from_yaml(yaml).get_matches();
    let out_dir = exec::ready_out_dir("replica_mean");
    let setting = exec::ready_setting(m.value_of("config"), &out_dir);
    let truth = exec::ready_truth(m.value_of("init"), m.value_of("truth"), &out_dir, &setting);
    replica_mean(truth, out_dir, setting);
}
