use std::{fs, path::Path};

use toml;
use serde::Deserialize;
use colored::*;

#[derive(Debug, Deserialize)]
pub struct ConfWorkspace {
    pub exe_name:String,
    pub source_dirs:Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Config  {
    pub workspace:ConfWorkspace,
}

impl Config {
    pub fn new(path: &str) -> Result<Config, String> {
        if !Path::new(path).exists() {
            return Err(format!(
                "{} {} {}",
                "Failed to find".truecolor(128, 128, 128),
                path.underline(),
                "because it doesn't exist".truecolor(128, 128, 128),
            ));
        }

        let contents = fs::read_to_string(path).map_err(|err| {
            format!(
                "{} '{}': {}",
                "Error when opening file".truecolor(128, 128, 128),
                path.underline(),
                err
            )
        })?;

        let conf: Config = toml::from_str(&contents).map_err(|err| {
            format!(
                "{} '{}': {}",
                "Error when parsing".truecolor(128, 128, 128),
                path.underline(),
                err
            )
        })?;

        Ok(conf)
    }
}