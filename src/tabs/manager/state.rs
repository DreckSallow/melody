use std::{cell::RefCell, path::PathBuf, rc::Rc};

use crate::{
    handlers::music::{PlaylistInfo, PlaylistSong},
    select,
    tabs::log::LogMessage,
    utils::Condition,
    view::{
        controllers::list::ListController,
        widgets::state::{input::InputState, SelectListState},
    },
};

pub struct MusicManagerState {
    pub(crate) list_songs: SelectListState,
    pub(crate) input_state: InputState,
    pub(crate) list_playlists: ListController,
    pub(crate) playlists: Vec<PlaylistInfo>,
    pub(crate) songs: Vec<PlaylistSong>,
    pub(crate) focus_i: u8,
    pub logger: Rc<RefCell<Vec<LogMessage>>>,
}

impl MusicManagerState {
    pub fn update_select_list(&mut self) {
        let selecteds: Vec<usize> = match self
            .list_playlists
            .selected()
            .and_then(|i| self.playlists.get(i))
        {
            Some(play) => {
                let songs_paths: Vec<PathBuf> = play.songs.iter().map(|s| s.path.clone()).collect();
                self.songs
                    .iter()
                    .enumerate()
                    .filter_map(|(i, s)| songs_paths.contains(&s.path).then_some(i))
                    .collect()
            }
            None => [].into(),
        };
        self.list_songs = SelectListState::default()
            .with_len(self.songs.len())
            .with_selecteds(selecteds)
            .with_index(select!(self.songs.is_empty(), None, Some(0)));
    }
    pub fn delete_playlist(&mut self) {
        if let Some(i) = self.list_playlists.selected() {
            let play = self.playlists.remove(i);
            if self.playlists.len() == 0 {
                self.list_playlists.select(None);
            } else if i >= self.playlists.len() {
                self.list_playlists.select(Some(self.playlists.len() - 1));
            }
            self.update_select_list();
            self.logger.borrow_mut().push(LogMessage::Warn(format!(
                "Playlist Deleted: {}, songs length: {}",
                play.name,
                play.songs.len()
            )))
        }
    }

    pub fn update_playlist(&mut self) {
        if let Some(i) = self.list_playlists.selected() {
            if let Some(playlist) = self.playlists.get_mut(i) {
                playlist.songs = [].into();
                let selecteds = self.list_songs.selecteds();
                for (i, song) in self.songs.iter().enumerate() {
                    if selecteds.contains(&i) {
                        playlist.songs.push(song.clone());
                    }
                }
            }
        }
    }
}
