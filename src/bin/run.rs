
#[macro_use]
extern crate clap;
extern crate aics_da;

use clap::App;

use aics_da::*;

fn run(setting: da::Setting) {
    //
}

fn main() {
    let cli = load_yaml!("run.yml");
    let matches = App::from_yaml(cli).get_matches();

    let setting_json = matches.value_of("config").unwrap_or("setting.json");
    let setting_path = ::std::path::Path::new(setting_json);

    if !setting_path.exists() {
        println!("Setting file '{}' is not found", setting_json);
        ::std::process::exit(1);
    }

    let setting = io::read_json(setting_path.to_str().unwrap());
    run(setting);
}
