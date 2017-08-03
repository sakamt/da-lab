
mod run;
mod replica_mean;
mod model_bias;

pub use self::model_bias::*;
pub use self::replica_mean::*;
pub use self::run::*;

use super::{da, io, model, observation, types};

pub const SETTING_JSON: &'static str = "setting.json";

pub fn init() {
    ::dotenv::dotenv().ok();
    ::env_logger::init().unwrap();
}

pub fn execute(setting: da::Setting) {
    info!("Execute task: {}", setting.task);
    match setting.task.as_str() {
        "run" => run(setting),
        "replica_mean" => replica_mean(setting),
        "model_bias" => model_bias(setting),
        "model_bias_replica" => model_bias_replica(setting),
        _ => warn!("Invalid task name: {}", setting.task),
    };
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
    let obs = &setting.obs;
    match *obs {
        Some(ref obs) => io::load_msg(&obs),
        None => observation::generate_obs(&truth, &setting),
    }
}
