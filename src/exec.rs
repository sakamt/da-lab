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
pub fn ready_truth(setting: &da::Setting) -> types::Truth {
    let truth = &setting.truth;
    let init = &setting.init;
    match *truth {
        Some(ref truth) => {
            match *init {
                Some(ref init) => info!("init file '{}' is ignored", init),
                None => {}
            }
            io::load_msg(&truth)
        }
        None => {
            let init = match *init {
                Some(ref init) => io::load_msg(init),
                None => model::generate_init(&setting),
            };
            model::generate_truth(&init, &setting)
        }
    }
}

/// read or generate observation
pub fn ready_obs(truth: &types::Truth, setting: &da::Setting) -> types::Observation {
    let obs = &setting.observation;
    match *obs {
        Some(ref obs) => io::load_msg(&obs),
        None => observation::generate_obs(&truth, &setting),
    }
}
