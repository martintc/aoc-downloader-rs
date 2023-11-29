use anyhow::Error;
use anyhow::Result;
use home::home_dir;
use serde::Deserialize;
use serde::Serialize;
use std::{io::Write, process::exit};

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub api_key: String,
}

pub fn get_path() -> std::path::PathBuf {
    let config_path = match home_dir() {
        Some(home_path) => home_path,
        None => exit(1),
    };

    let config_path = config_path.join(".config/aoc-downloader");

    config_path
}

pub fn create_blank_config() -> anyhow::Result<(), anyhow::Error> {
    let config = Configuration {
        api_key: String::from(""),
    };

    let mut file = std::fs::File::create(get_path())?;

    file.write_all(toml::to_string(&config).unwrap().as_bytes())?;

    Ok(())
}

pub fn read_config() -> Result<Configuration, Error> {
    let file_contents = std::fs::read_to_string(get_path())?;
    let config: Configuration = toml::from_str(file_contents.as_str())?;
    Ok(config)
}

pub fn get_config() -> Result<Configuration, Error> {
    let config_path = get_path();
    if !std::path::Path::exists(&config_path) {
        match create_blank_config() {
            Ok(()) => (),
            Err(e) => panic!("Error: {}", e),
        };
    }

    let config = match read_config() {
        Ok(config) => config,
        Err(e) => panic!("Error: {}", e),
    };
    Ok(config)
}

pub fn write_config(config: &Configuration) -> Result<(), anyhow::Error> {
    let mut file = std::fs::File::create(get_path())?;

    file.write_all(toml::to_string(&config).unwrap().as_bytes())?;

    Ok(())
}
