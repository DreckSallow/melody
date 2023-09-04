use std::{
    path::{Path, PathBuf},
    time::Duration,
};

use anyhow::Result;
use lofty::{Accessor, AudioFile, Probe, TaggedFileExt};

use crate::{
    data::playlists::{PlaylistStore, RawPlaylist, RawPlaylistToml},
    utils,
};

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
    pub duration_format: String,
}

pub struct MusicHandler;

impl MusicHandler {
    pub fn load_playlists() -> Result<Vec<PlaylistInfo>> {
        let raw_toml = PlaylistStore::load()?;
        let mut playlists = Vec::new();

        for raw_playlist in raw_toml.playlists {
            let mut playlist = PlaylistInfo {
                name: raw_playlist.name,
                songs: Vec::new(),
            };

            for path_song in raw_playlist.songs {
                let audio_song = Self::get_audio_data(&path_song);
                if let Ok(audio) = audio_song {
                    playlist.songs.push(audio);
                }
            }
            // Filter the empty playlists
            if !playlist.songs.is_empty() {
                playlists.push(playlist);
            }
        }
        Ok(playlists)
    }
    pub fn save_playlists(playlists: &[PlaylistInfo]) -> Result<()> {
        let mut data_vec = Vec::new();
        for playlist in playlists {
            let songs = playlist
                .songs
                .iter()
                .filter_map(|s| s.path.to_str().map(|s| s.to_string()))
                .collect();
            data_vec.push(RawPlaylist {
                name: playlist.name.clone(),
                songs,
            });
        }
        PlaylistStore::save(RawPlaylistToml {
            playlists: data_vec,
        })
    }

    pub fn load_songs<P: AsRef<Path>>(path: P) -> Result<Vec<PlaylistSong>> {
        let exts = ["mp3"];
        let mut songs = Vec::new();

        for entry in (path.as_ref().read_dir()?).flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if exts.contains(&ext) {
                    if let Ok(s) = Self::get_audio_data(path) {
                        songs.push(s);
                    }
                }
            }
        }
        Ok(songs)
    }
    fn get_audio_data<P: AsRef<Path>>(p: P) -> Result<PlaylistSong> {
        let tagged_file = Probe::open(&p)?.read()?;
        let properties = tagged_file.properties();
        let raw_title = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag())
            .and_then(|t| t.title().as_deref().map(|o| o.to_string()));
        let path_buf = PathBuf::from(p.as_ref());
        let d = properties.duration();
        Ok(PlaylistSong {
            title: raw_title,
            file_name: path_buf
                .file_name()
                .and_then(|n| n.to_str())
                .map(|st| st.to_string()),
            path: path_buf,
            duration_format: utils::format_time(d.as_secs()),
            duration: d,
        })
    }
}
