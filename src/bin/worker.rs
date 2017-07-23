
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

extern crate aics_da;

use aics_da::*;
use clap::App;
use std::fs::*;
use std::path::*;
use std::thread::sleep;
use std::time::Duration;

const SETTING_DIR: &'static str = "settings";

fn get_setting<P: AsRef<Path>>(setting_dir: P) -> Option<da::Setting> {
    for s in read_dir(setting_dir).expect("Cannot open setting directory") {
        let path = s.unwrap().path();
        let setting: da::Setting = io::read_json(path.to_str().unwrap());
        match remove_file(path) {
            Ok(_) => return Some(setting),
            Err(_) => continue,
        }
    }
    info!("No setting files");
    None
}

fn main() {
    exec::init();
    let cli = load_yaml!("worker.yml");
    let m = App::from_yaml(cli).get_matches();

    let setting_dir = m.value_of("setting-dir").unwrap_or(SETTING_DIR);
    let interval = m.value_of("interval")
        .map(|s| s.parse::<u64>().expect("cannot parse interval"))
        .unwrap_or(10);

    info!("watch {}", setting_dir);
    loop {
        let setting = loop {
            match get_setting(setting_dir) {
                Some(setting) => break setting,
                None => sleep(Duration::from_secs(interval)),
            }
        };
        println!("{:?}", setting);
    }
}
