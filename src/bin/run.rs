
extern crate clap;
extern crate aics_da;

use clap::*;

use aics_da::*;

fn run(setting: da::Setting) {
    //
}

fn main() {
    let matches = App::new("Run assimilation")
        .version("1.0")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("setting JSON (default: 'setting.json')")
                .takes_value(true),
        )
        .arg(Arg::with_name("progress").short("p").help(
            "show progress bar",
        ))
        .get_matches();

    let setting_json = matches.value_of("config").unwrap_or("setting.json");
    let setting_path = ::std::path::Path::new(setting_json);

    if !setting_path.exists() {
        println!("Setting file '{}' is not found", setting_json);
        ::std::process::exit(1);
    }

    let setting = io::read_json(setting_path.to_str().unwrap());
    run(setting);
}
