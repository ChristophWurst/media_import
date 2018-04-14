use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use failure;
use serde_json;
use xdg::{BaseDirectories, BaseDirectoriesError};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "XDG error: {}", err)]
    XDGError { err: BaseDirectoriesError },
    #[fail(display = "config parsing error: {}", err)]
    ParsingError { err: serde_json::Error },
}

#[derive(Debug, Deserialize)]
pub struct UserConfig {
    target_path: PathBuf,
}

impl UserConfig {
    pub fn target_path(&self) -> &PathBuf {
        &self.target_path
    }
}

fn get_config_path() -> Result<PathBuf, failure::Error> {
    let xdg = BaseDirectories::new().map_err(|err| Error::XDGError { err: err })?;
    Ok(xdg.get_config_home().join("media_import.json"))
}

fn read_config_file(path: &PathBuf) -> Result<String, failure::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse_user_config(config_str: &String) -> Result<UserConfig, failure::Error> {
    let config = serde_json::from_str(config_str).map_err(|err| Error::ParsingError { err: err })?;
    Ok(config)
}

pub fn get_user_config() -> Result<UserConfig, failure::Error> {
    let path = get_config_path()?;
    let config_str = read_config_file(&path)?;

    parse_user_config(&config_str)
}
