
#[macro_use]
extern crate clap;

extern crate da_lab;

use da_lab::*;
use clap::App;

pub const SETTING_JSON: &'static str = "setting.json";

fn main() {
    task::init();
    let cli = load_yaml!("exec.yml");
    let m = App::from_yaml(cli).get_matches();
    let setting_json = m.value_of("setting").unwrap_or(SETTING_JSON);
    let setting_path = ::std::path::Path::new(setting_json);
    let setting = io::read_json(&setting_path).expect("Invalid JSON");
    task::execute(setting);
}
