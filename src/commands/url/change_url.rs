use std::{ process, env };

use std::path::PathBuf;
use crate::file_functions::ConfigFile;

// static CHANGE_URL_PREFIXES: [&str; 2] = ["changeurl", "chgurl"];

pub fn run_changeurl(url: String) {
    let config: ConfigFile = ConfigFile { url: url.to_string() };

    // Determine the location of the binary file
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

    let file_path: PathBuf = exe_dir.join("config.json");

    // Write the ConfigFile struct containing the URL to the JSON file
    ConfigFile::write_config_to_file(file_path.to_str().unwrap(), &config).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(0)
    });

    println!("Sucessfully changed the API URL to {url}")
}
