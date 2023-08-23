use crate::loaders::player::{PlaylistInfo, PlaylistSong, PlaylistsData};

#[derive(Debug)]
pub struct PlayerState {
    pub library: PlaylistsData,
    pub playlist_selected: Option<usize>,
    pub audio_selected: Option<usize>,
}

impl PlayerState {
    pub fn create(playlist_data: PlaylistsData) -> Self {
        Self {
            library: playlist_data,
            playlist_selected: None,
            audio_selected: None,
        }
    }
    pub fn selected_playlist(&self) -> Option<&PlaylistInfo> {
        match self.playlist_selected {
            Some(index) => self.library.playlists.get(index),
            None => None,
        }
    }
    pub fn selected_audio(&self) -> Option<&PlaylistSong> {
        match self.selected_playlist() {
            Some(playlist) => match self.audio_selected {
                Some(i) => playlist.songs.get(i),
                None => None,
            },
            None => None,
        }
    }
    pub fn indices(&self) -> Option<(usize, usize)> {
        let playlist = self.playlist_selected;
        let audio = self.audio_selected;

        if playlist.is_some() && audio.is_some() {
            Some((playlist.unwrap(), audio.unwrap()))
        } else {
            None
        }
    }
}
