
#[macro_use]
extern crate clap;
#[macro_use]
extern crate log;
extern crate nix;

extern crate serde_json;

extern crate da_lab;

use clap::App;
use da_lab::*;
use nix::fcntl;
use std::fmt::Debug;
use std::fs::*;
use std::os::unix::io::AsRawFd;
use std::path::*;
use std::thread::sleep;
use std::time::Duration;

const SETTING_DIR: &'static str = "settings";

fn get_exlocked_setting<P: AsRef<Path> + Debug>(path: P) -> Option<da::Setting> {
    let mut f = match File::open(&path) {
        Ok(f) => f,
        Err(_) => {
            warn!("Cannot open file: {:?}", path);
            return None;
        }
    };
    let fd = f.as_raw_fd();
    match fcntl::flock(fd, fcntl::FlockArg::LockExclusiveNonblock) {
        Ok(_) => {}
        Err(_) => {
            info!("Cannot get lock: {:?}", path);
            return None;
        }
    }
    let setting = match ::serde_json::from_reader(&mut f) {
        Ok(setting) => setting,
        Err(_) => {
            warn!("Cannot decode JSON: {:?}", path);
            return None;
        }
    };
    Some(setting)
}

fn fetch_setting<P: AsRef<Path> + Debug>(setting_dir: P, interval: u64) -> (PathBuf, da::Setting) {
    loop {
        for s in read_dir(&setting_dir).expect("Cannot open setting directory") {
            let path = s.unwrap().path();
            match get_exlocked_setting(&path) {
                Some(setting) => return (path, setting),
                None => {
                    continue;
                }
            }
        }
        sleep(Duration::from_secs(interval));
    }
}

fn main() {
    task::init();
    let cli = load_yaml!("worker.yml");
    let m = App::from_yaml(cli).get_matches();

    let setting_dir = PathBuf::from(m.value_of("setting-dir").unwrap_or(SETTING_DIR));
    let interval = m.value_of("interval")
        .map(|s| s.parse::<u64>().expect("cannot parse interval"))
        .unwrap_or(10);

    if !setting_dir.exists() {
        create_dir_all(&setting_dir).expect("Cannot create setting directory");
    }

    info!("Setting directory = {:?}", setting_dir);
    info!("Watch interval = {}", interval);

    loop {
        let (path, setting) = fetch_setting(&setting_dir, interval);
        task::execute(setting);
        remove_file(path).expect("Cannot remove file");
        info!("Done, wait next task");
    }
}
