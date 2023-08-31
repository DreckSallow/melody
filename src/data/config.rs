use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::Deserialize;

use crate::dirs::config_dir;

#[derive(Deserialize, Debug)]
pub struct ConfigData {
    pub music_path: PathBuf,
}

impl ConfigData {
    pub fn load() -> Result<Self> {
        match config_dir() {
            Some(mut p) => {
                p.push("config.toml");
                if !p.exists() {
                    fs::File::create(&p)?;
                }
                let data: ConfigData = toml::from_str(&fs::read_to_string(p)?)?;
                Ok(data)
            }
            None => Err(anyhow!("Not was posible get the config")),
        }
    }
}
