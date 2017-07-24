
#[macro_use]
extern crate clap;

extern crate aics_da;

use aics_da::*;
use clap::App;

pub const SETTING_JSON: &'static str = "setting.json";

/// read setting JSON file
fn ready_setting(setting_json: Option<&str>) -> da::Setting {
    let setting_json = setting_json.unwrap_or(SETTING_JSON);
    let setting_path = ::std::path::Path::new(setting_json);
    io::read_json(setting_path.to_str().unwrap())
}

fn main() {
    task::init();
    let cli = load_yaml!("exec.yml");
    let m = App::from_yaml(cli).get_matches();
    let setting = ready_setting(m.value_of("setting"));
    task::execute(setting);
}
