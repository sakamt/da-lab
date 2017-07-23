//! misc utilities for executables

use super::{da, io, model, observation, types};

pub const SETTING_JSON: &'static str = "setting.json";

pub fn init() {
    ::dotenv::dotenv().ok();
    ::env_logger::init().unwrap();
}

/// read setting JSON file
pub fn ready_setting(setting_json: Option<&str>) -> da::Setting {
    let setting_json = setting_json.unwrap_or(SETTING_JSON);
    let setting_path = ::std::path::Path::new(setting_json);
    io::read_json(setting_path.to_str().unwrap())
}

/// read or generate truth
pub fn ready_truth(init: Option<&str>, truth: Option<&str>, setting: &da::Setting) -> types::Truth {
    if truth.is_some() {
        let truth = truth.unwrap();
        if init.is_some() {
            info!("init file '{}' will be ignored", init.unwrap());
        }
        io::load_msg(truth)
    } else {
        let init = if init.is_some() {
            io::load_msg(init.unwrap())
        } else {
            let init = model::generate_init(&setting);
            init
        };
        let truth = model::generate_truth(&init, &setting);
        truth
    }
}

/// read or generate observation
pub fn ready_obs(obs: Option<&str>, truth: &types::Truth, setting: &da::Setting) -> types::Observation {
    if obs.is_some() {
        let obs = obs.unwrap();
        io::load_msg(obs)
    } else {
        let obs = observation::generate_obs(&truth, &setting);
        obs
    }
}
