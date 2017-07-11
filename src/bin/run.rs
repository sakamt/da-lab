//! Run DA
//!
//! Outputs
//! -------
//! Sequential data will be saved in "$DATADIR/run/YYYY-MM-DD-HH:MM:SS/"
//! - setting.json
//! - truth.msg : sequence of true state
//! - obs.msg   : sequence of observations
//! - data00001.msg ... : msgpack of dictionary:
//!   - time
//!   - state
//!   - obs
//!   - forecast
//!   - analysis
//!   - rmse

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate time;
extern crate rustc_serialize;
extern crate ndarray_linalg;

extern crate aics_da;

use clap::App;
use ndarray_linalg::*;
use std::path::PathBuf;

use aics_da::*;
use aics_da::types::*;

const SETTING_JSON: &'static str = "setting.json";
const INIT_MSG: &'static str = "init.msg";
const TRUTH_MSG: &'static str = "truth.msg";
const OBS_MSG: &'static str = "obs.msg";

// input for DA process
struct Input {
    truth: Vec<V>,
    obs: Vec<V>,
    out_dir: PathBuf,
}

#[derive(RustcEncodable)]
struct Output {
    time: f64,
    state: V,
    obs: V,
    forecast: V,
    analysis: V,
    rmse: f64,
}

// run DA process
fn run(input: &Input, setting: &da::Setting) {
    let f = model::select_model(&setting);
    let a = da::select_analyzer(&setting);
    let mut xs = da::replica(&input.truth[0], setting.r, setting.k);
    for (t, (truth, y)) in input.truth.iter().zip(input.obs.iter()).enumerate() {
        let xb = stat::mean(&xs);
        xs = a.analysis(xs, &y);
        let xa = stat::mean(&xs);
        xs = f.forecast(xs);
        let rmse = (truth - &xa).norm();
        let output = Output {
            time: (t * setting.tau) as f64 * setting.dt,
            state: truth.clone(),
            obs: y.clone(),
            forecast: xb,
            analysis: xa,
            rmse: rmse,
        };
        let out_fn = format!("data{:05}.msg", t);
        io::save_msg(&output, input.out_dir.join(out_fn).to_str().unwrap());
    }
}

// generate output directory from current timestamp and $DATADIR env value
fn ready_out_dir() -> PathBuf {
    let out_dir = PathBuf::from(format!(
        "{}/run/{}",
        std::env::var("DATADIR").unwrap_or(".".to_string()),
        time::now().strftime("%F-%T").unwrap()
    ));
    std::fs::create_dir_all(&out_dir).expect("Cannot create out_dir directory");
    info!("out_dir directory = {:?}", &out_dir);
    out_dir
}

// Load or generate truth and observation
fn ready_input(
    init: Option<&str>,
    truth: Option<&str>,
    obs: Option<&str>,
    out_dir: PathBuf,
    setting: &da::Setting,
) -> Input {
    let truth: Vec<V> = if truth.is_some() {
        let truth = truth.unwrap();
        if init.is_some() {
            info!("init file '{}' will be ignored", init.unwrap());
        }
        std::fs::copy(truth, out_dir.join(TRUTH_MSG)).expect("Cannot copy truth");
        io::load_msg(truth)
    } else {
        let init = if init.is_some() {
            io::load_msg(init.unwrap())
        } else {
            let init = model::generate_init(&setting);
            io::save_msg(&init, INIT_MSG);
            info!("init is generated: {}", INIT_MSG);
            init
        };
        let truth = model::generate_truth(&init, &setting);
        io::save_msg(&truth, TRUTH_MSG);
        info!("truth is generated: {}", TRUTH_MSG);
        std::fs::copy(TRUTH_MSG, out_dir.join(TRUTH_MSG)).expect("Cannot copy truth");
        truth
    };
    let obs = if obs.is_some() {
        let obs = obs.unwrap();
        std::fs::copy(obs, out_dir.join(OBS_MSG)).expect("Cannot copy obs");
        io::load_msg(obs)
    } else {
        let obs = observation::generate_obs(&truth, &setting);
        io::save_msg(&obs, OBS_MSG);
        info!("observation is generated: {}", OBS_MSG);
        std::fs::copy(OBS_MSG, out_dir.join(OBS_MSG)).expect("Cannot copy obs");
        obs
    };

    Input {
        truth: truth,
        obs: obs,
        out_dir: out_dir,
    }
}

// read and copy setting JSON file
fn ready_setting(setting_json: &str, out_dir: &PathBuf) -> da::Setting {
    let setting_path = ::std::path::Path::new(setting_json);
    if !setting_path.exists() {
        println!("Setting file '{}' is not found", setting_json);
        ::std::process::exit(1);
    }
    std::fs::copy(setting_json, out_dir.join(SETTING_JSON)).expect("Cannot copy setting file");
    io::read_json(setting_path.to_str().unwrap())
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().unwrap();

    let cli = load_yaml!("run.yml");
    let matches = App::from_yaml(cli).get_matches();

    let out_dir = ready_out_dir();
    let setting = ready_setting(matches.value_of("config").unwrap_or(SETTING_JSON), &out_dir);

    let input = ready_input(
        matches.value_of("init"),
        matches.value_of("truth"),
        matches.value_of("obs"),
        out_dir,
        &setting,
    );

    run(&input, &setting);
}
