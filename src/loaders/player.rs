use std::{path::PathBuf, time::Duration};

use anyhow::{Error, Result};
use lofty::{Accessor, AudioFile, Probe, TaggedFileExt};
use serde::Deserialize;

#[derive(Clone, Debug)]
pub struct PlaylistsData {
    pub playlists: Vec<PlaylistInfo>,
}
#[derive(Clone, Debug)]
pub struct PlaylistInfo {
    pub name: String,
    pub songs: Vec<PlaylistSong>,
}
#[derive(Clone, Debug)]
pub struct PlaylistSong {
    pub title: Option<String>,
    pub file_name: Option<String>,
    pub path: PathBuf,
    pub duration: Duration,
}

impl TryFrom<RawToml> for PlaylistsData {
    type Error = Error;
    fn try_from(raw: RawToml) -> Result<Self, Self::Error> {
        let mut data = PlaylistsData {
            playlists: Vec::new(),
        };

        for raw_playlist in raw.playlists {
            let mut playlist = PlaylistInfo {
                name: raw_playlist.name,
                songs: Vec::new(),
            };

            for path_song in raw_playlist.songs {
                let audio_song = get_audio_data(&path_song);
                if audio_song.is_ok() {
                    playlist.songs.push(audio_song.unwrap());
                }
            }

            data.playlists.push(playlist);
        }
        Ok(data)
    }
}

fn get_audio_data(p: &str) -> Result<PlaylistSong> {
    let tagged_file = Probe::open(p)?.read()?;
    let properties = tagged_file.properties();
    let raw_title = tagged_file
        .primary_tag()
        .or_else(|| tagged_file.first_tag())
        .and_then(|t| t.title().as_deref().map(|o| o.to_string()));
    let path_buf = PathBuf::from(p);
    Ok(PlaylistSong {
        title: raw_title,
        file_name: path_buf
            .file_name()
            .and_then(|n| n.to_str())
            .map(|st| st.to_string()),
        path: path_buf,
        duration: properties.duration(),
    })
}

#[derive(Deserialize)]
pub(crate) struct RawToml {
    pub playlists: Vec<RawPlaylists>,
}
#[derive(Deserialize)]
pub(crate) struct RawPlaylists {
    name: String,
    songs: Vec<String>,
}
