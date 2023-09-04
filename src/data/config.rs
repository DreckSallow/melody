use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use directories::UserDirs;
use serde::Deserialize;

use crate::dirs::config_dir;

#[derive(Deserialize, Debug)]
pub struct ConfigData {
    pub music_path: PathBuf,
}

impl ConfigData {
    pub fn try_default() -> Result<Self> {
        match UserDirs::new().and_then(|u| u.audio_dir().map(|p| p.to_owned())) {
            Some(p) => Ok(Self { music_path: p }),
            None => Err(anyhow!("Failed to find the music default path")),
        }
    }
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
