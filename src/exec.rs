//! misc utilities for executables
//!

use super::{da, io, model, observation, types};
use std::path::*;

pub const SETTING_JSON: &'static str = "setting.json";
pub const INIT_MSG: &'static str = "init.msg";
pub const TRUTH_MSG: &'static str = "truth.msg";
pub const OBS_MSG: &'static str = "obs.msg";

pub fn init() {
    ::dotenv::dotenv().ok();
    ::env_logger::init().unwrap();
}

/// generate output directory from current timestamp and $DATADIR env value
pub fn ready_out_dir(prefix: &str) -> PathBuf {
    let out_dir = PathBuf::from(format!(
        "{}/{}/{}",
        ::std::env::var("DATADIR").unwrap_or(".".to_string()),
        prefix,
        ::time::now().strftime("%F-%T").unwrap()
    ));
    ::std::fs::create_dir_all(&out_dir).expect("Cannot create out_dir directory");
    info!("out_dir directory = {:?}", &out_dir);
    out_dir
}

/// read and copy setting JSON file
pub fn ready_setting(setting_json: Option<&str>, out_dir: &Path) -> da::Setting {
    let setting_json = setting_json.unwrap_or(SETTING_JSON);
    let setting_path = ::std::path::Path::new(setting_json);
    if !setting_path.exists() {
        println!("Setting file '{}' is not found", setting_json);
        ::std::process::exit(1);
    }
    ::std::fs::copy(setting_json, out_dir.join(SETTING_JSON)).expect("Cannot copy setting file");
    io::read_json(setting_path.to_str().unwrap())
}

/// read or generate and save truth
pub fn ready_truth(init: Option<&str>, truth: Option<&str>, out_dir: &Path, setting: &da::Setting) -> types::Truth {
    if truth.is_some() {
        let truth = truth.unwrap();
        if init.is_some() {
            info!("init file '{}' will be ignored", init.unwrap());
        }
        ::std::fs::copy(truth, out_dir.join(TRUTH_MSG)).expect("Cannot copy truth");
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
        ::std::fs::copy(TRUTH_MSG, out_dir.join(TRUTH_MSG)).expect("Cannot copy truth");
        truth
    }
}

/// read or generate and save observation
pub fn ready_obs(obs: Option<&str>, truth: &types::Truth, out_dir: &Path, setting: &da::Setting) -> types::Observation {
    if obs.is_some() {
        let obs = obs.unwrap();
        ::std::fs::copy(obs, out_dir.join(OBS_MSG)).expect("Cannot copy obs");
        io::load_msg(obs)
    } else {
        let obs = observation::generate_obs(&truth, &setting);
        io::save_msg(&obs, OBS_MSG);
        info!("observation is generated: {}", OBS_MSG);
        ::std::fs::copy(OBS_MSG, out_dir.join(OBS_MSG)).expect("Cannot copy obs");
        obs
    }
}
