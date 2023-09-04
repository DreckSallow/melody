use std::{fs, path::PathBuf};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

use crate::dirs::local_data_dir;

#[derive(Deserialize, Serialize)]
pub struct RawPlaylistToml {
    pub playlists: Vec<RawPlaylist>,
}
#[derive(Deserialize, Serialize)]
pub struct RawPlaylist {
    pub name: String,
    pub songs: Vec<String>,
}
pub struct PlaylistStore;

impl PlaylistStore {
    pub const FILE: &str = "data.toml";
    pub fn path() -> Result<PathBuf> {
        match local_data_dir() {
            Some(mut p) => {
                p.push(Self::FILE);
                if !p.exists() {
                    fs::File::create(&p)?;
                    // This is for create the toml structure
                    Self::save(RawPlaylistToml {
                        playlists: Vec::new(),
                    })?
                }
                Ok(p)
            }
            None => Err(anyhow!("Not was posible get the config")),
        }
    }
    pub fn load() -> Result<RawPlaylistToml> {
        let p = Self::path()?;
        let data: RawPlaylistToml = toml::from_str(&fs::read_to_string(p)?)?;
        Ok(data)
    }
    pub fn save(data: RawPlaylistToml) -> Result<()> {
        let p = Self::path()?;
        let data_toml = toml::to_string(&data)?;
        fs::write(p, data_toml)?;
        Ok(())
    }
}
