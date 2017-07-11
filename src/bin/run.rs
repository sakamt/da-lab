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

struct Input {
    truth: Vec<V>,
    obs: Vec<V>,
    output: PathBuf,
}

const SETTING_JSON: &'static str = "setting.json";
const INIT_MSG: &'static str = "init.msg";
const TRUTH_MSG: &'static str = "truth.msg";
const OBS_MSG: &'static str = "obs.msg";

fn run(input: &Input, setting: &da::Setting) -> Result<(), Box<std::error::Error>> {
    Ok(())
}

fn ready_output_dir() -> PathBuf {
    let output = PathBuf::from(format!(
        "{}/run/{}",
        std::env::var("DATADIR").unwrap_or(".".to_string()),
        time::now().strftime("%F-%T").unwrap()
    ));
    std::fs::create_dir_all(&output).expect("Cannot create output directory");
    info!("output directory = {:?}", &output);
    output
}

fn ready_input(
    init: Option<&str>,
    truth: Option<&str>,
    obs: Option<&str>,
    output: PathBuf,
    setting: &da::Setting,
) -> Input {
    let truth: Vec<V> = if truth.is_some() {
        let truth = truth.unwrap();
        if init.is_some() {
            info!("init file '{}' will be ignored", init.unwrap());
        }
        std::fs::copy(truth, output.join(TRUTH_MSG)).expect("Cannot copy truth");
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
        std::fs::copy(TRUTH_MSG, output.join(TRUTH_MSG)).expect("Cannot copy truth");
        truth
    };
    let obs = if obs.is_some() {
        let obs = obs.unwrap();
        std::fs::copy(obs, output.join(OBS_MSG)).expect("Cannot copy obs");
        io::load_msg(obs)
    } else {
        let obs = observation::generate_obs(&truth, &setting);
        io::save_msg(&obs, OBS_MSG);
        info!("observation is generated: {}", OBS_MSG);
        std::fs::copy(OBS_MSG, output.join(OBS_MSG)).expect("Cannot copy obs");
        obs
    };

    Input {
        truth: truth,
        obs: obs,
        output: output,
    }
}

fn ready_setting(setting_json: &str, output: &PathBuf) -> da::Setting {
    let setting_path = ::std::path::Path::new(setting_json);
    if !setting_path.exists() {
        println!("Setting file '{}' is not found", setting_json);
        ::std::process::exit(1);
    }
    std::fs::copy(setting_json, output.join(SETTING_JSON)).expect("Cannot copy setting file");
    io::read_json(setting_path.to_str().unwrap())

}

fn main() {
    dotenv::dotenv().ok();
    env_logger::init().unwrap();

    let cli = load_yaml!("run.yml");
    let matches = App::from_yaml(cli).get_matches();

    let output = ready_output_dir();
    let setting = ready_setting(matches.value_of("config").unwrap_or(SETTING_JSON), &output);

    // data
    let input = ready_input(
        matches.value_of("init"),
        matches.value_of("truth"),
        matches.value_of("obs"),
        output,
        &setting,
    );

    run(&input, &setting).unwrap();
}
