use anyhow::{anyhow, Result};
use std::fs;

use crate::dirs::playlists_path;

use self::player::{PlaylistsData, RawToml};

pub mod player;

pub fn load_playlists() -> Result<PlaylistsData> {
    match playlists_path() {
        Some(path) => {
            let data = fs::read_to_string(path)?;
            let datin: RawToml = toml::from_str(&data)?;
            PlaylistsData::try_from(datin)
        }
        None => Err(anyhow!("Not was posible get the data info")),
    }
}
