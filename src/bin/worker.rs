
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;

extern crate da_lab;

use da_lab::*;
use clap::App;
use std::fs::*;
use std::path::*;
use std::thread::sleep;
use std::time::Duration;

const SETTING_DIR: &'static str = "settings";

fn get_setting<P: AsRef<Path>>(setting_dir: P) -> Option<da::Setting> {
    for s in read_dir(setting_dir).expect("Cannot open setting directory") {
        let path = s.unwrap().path();
        info!("Try fetch: {:?}", path);
        let setting = match io::read_json(&path) {
            Ok(setting) => setting,
            Err(_) => {
                warn!("Cannot read, skip (remove manually)");
                continue;
            }
        };
        match remove_file(path.clone()) {
            Ok(_) => {
                info!("Fetched");
                return Some(setting);
            }
            Err(_) => {
                info!("Cannot delete JSON file, Maybe fetched by another process");
                continue;
            }
        }
    }
    debug!("No setting files");
    None
}

fn main() {
    task::init();
    let cli = load_yaml!("worker.yml");
    let m = App::from_yaml(cli).get_matches();

    let setting_dir = m.value_of("setting-dir").unwrap_or(SETTING_DIR);
    let interval = m.value_of("interval")
        .map(|s| s.parse::<u64>().expect("cannot parse interval"))
        .unwrap_or(10);

    info!("Start watching (every {}sec): {}", interval, setting_dir);
    loop {
        let setting = loop {
            match get_setting(setting_dir) {
                Some(setting) => break setting,
                None => sleep(Duration::from_secs(interval)),
            }
        };
        task::execute(setting);
        info!("Done, wait next task");
    }
}
