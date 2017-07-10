//! Run DA
//!
//! Outputs
//! -------
//! Sequential data will be saved in "$DATADIR/run/YYYYMMDD-HHMMSS/"
//! - setting.json
//! - truth.msg : sequence of true state
//! - obs.msg   : sequence of observations
//! - ts00001.msg ... : msgpacka of dictionary
//!   - time
//!   - state
//!   - obs
//!   - forecast
//!   - analysis
//!   - rmse
#![allow(unused_variables, dead_code)]

#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate dotenv;
#[macro_use]
extern crate log;
extern crate time;

extern crate aics_da;

use clap::App;
use std::path::PathBuf;

use aics_da::*;
use aics_da::types::*;

struct Info {
    truth: Vec<V>,
    obs: Vec<V>,
    output: PathBuf,
}

// filename of generated data
const INIT_MSG: &'static str = "init.msg";
const TRUTH_MSG: &'static str = "truth.msg";
const OBS_MSG: &'static str = "obs.msg";

fn generate_init(setting: &da::Setting) -> V {
    let init = V::zeros(3); // TODO moc
    io::save_msg(&init, INIT_MSG);
    info!("init is generated: {}", INIT_MSG);
    init
}

fn generate_truth(init: &V, setting: &da::Setting) -> Vec<V> {
    let truth = vec![V::zeros(3); 2]; // TODO moc
    io::save_msg(&truth, TRUTH_MSG);
    info!("truth is generated: {}", TRUTH_MSG);
    truth
}

fn generate_obs(truth: &Vec<V>, setting: &da::Setting) -> Vec<V> {
    let obs = vec![V::zeros(3); 2]; // TODO moc
    io::save_msg(&obs, OBS_MSG);
    info!("observation is generated: {}", OBS_MSG);
    obs
}

fn run(info: &Info, setting: &da::Setting) -> Result<(), Box<std::error::Error>> {
    Ok(())
}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().unwrap();

    let cli = load_yaml!("run.yml");
    let matches = App::from_yaml(cli).get_matches();

    // setting JSON
    let setting_json = matches.value_of("config").unwrap_or("setting.json");
    let setting_path = ::std::path::Path::new(setting_json);
    if !setting_path.exists() {
        println!("Setting file '{}' is not found", setting_json);
        ::std::process::exit(1);
    }
    let setting = io::read_json(setting_path.to_str().unwrap());

    // output
    let output = PathBuf::from(format!(
        "{}/run/{}",
        std::env::var("DATADIR").unwrap_or(".".to_string()),
        time::now().strftime("%F-%T").unwrap()
    ));
    std::fs::create_dir_all(&output).expect("Cannot create output directory");
    info!("output directory = {:?}", &output);

    // data
    let init = matches.value_of("init");
    let truth = matches.value_of("truth");
    let obs = matches.value_of("obs");

    let truth: Vec<V> = if truth.is_some() {
        let truth = truth.unwrap();
        if init.is_some() {
            info!("init file '{}' will be ignored", init.unwrap());
        }
        std::fs::copy(truth, output.join(TRUTH_MSG)).expect("Cannot copy truth.msg");
        io::load_msg(truth)
    } else {
        let init = if init.is_some() {
            io::load_msg(init.unwrap())
        } else {
            generate_init(&setting)
        };
        let truth = generate_truth(&init, &setting);
        std::fs::copy(TRUTH_MSG, output.join(TRUTH_MSG)).expect("Cannot copy truth.msg");
        truth
    };
    let obs = if obs.is_some() {
        let obs = obs.unwrap();
        std::fs::copy(obs, output.join(OBS_MSG)).expect("Cannot copy obs.msg");
        io::load_msg(obs)
    } else {
        let obs = generate_obs(&truth, &setting);
        std::fs::copy(OBS_MSG, output.join(OBS_MSG)).expect("Cannot copy obs.msg");
        obs
    };

    // run
    let info = Info {
        truth: truth,
        obs: obs,
        output: output,
    };
    run(&info, &setting).unwrap();
}
