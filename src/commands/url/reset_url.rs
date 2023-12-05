use std::{ process, env };

use crate::file_functions::ConfigFile;
use crate::DEFAULT_API_URL;

pub fn run_reseturl() {
    let config: ConfigFile = ConfigFile { url: DEFAULT_API_URL.to_string() };

    let exe_path = env::current_exe().unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(0)
    });

    let exe_dir = exe_path
        .parent()
        .ok_or("Unable to get parent directory")
        .unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(0)
        });

    let file_path = exe_dir.join("config.json");

    ConfigFile::write_config_to_file(file_path.to_str().unwrap(), &config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(0)
    });

    println!("Sucessfully reset the API URL to {}", DEFAULT_API_URL)
}
