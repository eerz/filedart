use std::{ error::Error, process };
use std::{ fs, env };
use serde_json;
use serde::{ Deserialize, Serialize };
use std::path::Path;
use std::ffi::OsStr;

use std::fs::File;

use crate::DEFAULT_API_URL;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub url: String,
    // Other fields can be added here if needed
}

impl ConfigFile {
    pub fn get_url() -> String {
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

        let config = ConfigFile::read_config_from_file(file_path.to_str().unwrap()).unwrap_or_else(
            |err| {
                eprintln!("{}", err);
                process::exit(0)
            }
        );

        config.url
    }

    pub fn get_filename(location: &str) -> String {
        let file_name = Path::new(location)
            .file_name()
            .and_then(OsStr::to_str)
            .ok_or("Error reading file name")
            .unwrap_or_else(|err| {
                eprintln!("{}", err);
                process::exit(1);
            });

        file_name.to_string()
    }

    pub fn open_file(location: &str) -> File {
        let file: File = File::open(location).unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(0)
        });

        file
    }

    pub fn write_config_to_file(
        file_path: &str,
        config: &ConfigFile
    ) -> Result<(), Box<dyn Error>> {
        let json_content = serde_json::to_string_pretty(config)?;
        fs::write(file_path, json_content)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn read_config_from_file(file_path: &str) -> Result<ConfigFile, &str> {
        if let Ok(contents) = fs::read_to_string(file_path) {
            if let Ok(config) = serde_json::from_str::<ConfigFile>(&contents) {
                Ok(config)
            } else {
                return Err("Invalid JSON Format");
            }
        } else {
            Ok(ConfigFile { url: DEFAULT_API_URL.to_string() })
        }
    }
}
