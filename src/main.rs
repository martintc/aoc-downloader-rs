use std::{env::home_dir, process::exit, io::Write};

use anyhow::Error;
use serde::Serialize;
use serde::Deserialize;
use anyhow::Result;

#[derive(Debug)]
struct Args {
    year: u32,
    day: u16,
    apiKey: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    apiKey: String
}

fn get_path() -> std::path::PathBuf {
    let config_path = match home_dir() {
            Some(home_path) => home_path,
            None => {
                exit(1)
            }
    };

    let config_path = config_path.join(".config/aoc-downloader");

    config_path
}

fn create_blank_config() -> anyhow::Result<(), anyhow::Error> {
    let config = Configuration {
        apiKey: String::from("")
    };

    let mut file = std::fs::File::create(get_path())?;

    file.write_all(toml::to_string(&config).unwrap().as_bytes())?;

    Ok(())
}

fn read_config() -> Result<Configuration, Error> {
    let file_contents = std::fs::read_to_string(get_path())?;
    let config: Configuration = toml::from_str(file_contents.as_str())?;
    Ok(config)
}

fn get_config() -> Result<Configuration, Error> {
    let config_path = get_path();
    if !std::path::Path::exists(&config_path) {
        match create_blank_config() {
            Ok(()) => (),
            Err(e) => panic!("Error: {}", e)
        };
    }

    let config = match read_config() {
        Ok(config) => config,
        Err(e) => panic!("Error: {}", e)
    };
    Ok(config)
}


fn main() -> anyhow::Result<(), anyhow::Error> {
    let config = get_config();
    match config {
        Ok(con) => println!("{:#?}", con),
        Err(e) => println!("{}", e)
    };

    Ok(())
    
}
