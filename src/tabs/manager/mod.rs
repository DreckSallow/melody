use std::{path::PathBuf, rc::Rc};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::prelude::*;

use crate::{
    app::AppState,
    component::{Component, FinishableComp},
    event::AppEvent,
    handlers::music::MusicHandler,
    select,
    utils::Condition,
    view::{
        controllers::list::ListController,
        widgets::state::{input::InputState, SelectListState},
    },
};

mod sections;
mod state;
use self::{
    sections::{InputPlaylist, PlaylistsManager, SongsManager},
    state::MusicManagerState,
};

pub struct PlaylistManager {
    state: MusicManagerState,
    input_playlists: InputPlaylist,
    playlists: PlaylistsManager,
    songs: SongsManager,
}

impl PlaylistManager {
    pub fn build(app_state: &AppState, p: &PathBuf) -> Result<Self> {
        let songs = MusicHandler::load_songs(p)?;
        let playlists = MusicHandler::load_playlists()?;
        let selecteds = if let Some(play) = playlists.get(0) {
            let songs_paths: Vec<PathBuf> = play.songs.iter().map(|s| s.path.clone()).collect();
            songs
                .iter()
                .enumerate()
                .filter_map(|(i, s)| songs_paths.contains(&s.path).then_some(i))
                .collect()
        } else {
            Vec::new()
        };
        let focus_i = select!(playlists.is_empty(), 0, 1);
        let state = MusicManagerState {
            list_playlists: ListController::default().with_select(select!(
                playlists.is_empty(),
                None,
                Some(0)
            )),
            playlists,
            list_songs: SelectListState::default()
                .with_len(songs.len())
                .with_selecteds(selecteds)
                .with_index(select!(songs.is_empty(), None, Some(0))),
            input_state: InputState::default(),
            songs,
            focus_i,
            logger: Rc::clone(&app_state.log),
        };
        Ok(Self {
            state,
            input_playlists: InputPlaylist,
            playlists: PlaylistsManager,
            songs: SongsManager,
        })
    }

    fn save_data(&mut self) -> Result<()> {
        self.state.update_playlist();
        MusicHandler::save_playlists(&self.state.playlists)
    }
}

impl Component for PlaylistManager {
    type State = AppState;
    fn render(
        &mut self,
        frame: &mut crate::component::FrameType,
        area: ratatui::prelude::Rect,
        _state: &mut Self::State,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(50)])
            .split(area);

        let play_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Max(3), Constraint::Percentage(80)])
            .split(chunks[0]);

        self.input_playlists
            .render(frame, play_chunks[0], &mut self.state);

        self.playlists
            .render(frame, play_chunks[1], &mut self.state);

        self.songs.render(frame, chunks[1], &mut self.state);
    }
    fn on_event(&mut self, event: &AppEvent, _state: &mut Self::State) {
        if let AppEvent::Key(key_event) = event {
            if let KeyModifiers::CONTROL = key_event.modifiers {
                if let KeyCode::Char(n) = key_event.code {
                    match n {
                        '1' => self.state.focus_i = 0,
                        '2' => self.state.focus_i = 1,
                        '3' => self.state.focus_i = 2,
                        _ => {}
                    }
                }
            }
        }

        match self.state.focus_i {
            0 => self.input_playlists.on_event(event, &mut self.state),
            1 => self.playlists.on_event(event, &mut self.state),
            2 => self.songs.on_event(event, &mut self.state),
            _ => {}
        }
    }
}

impl FinishableComp for PlaylistManager {
    type Res = ();
    fn finish(&mut self) -> Result<Self::Res> {
        self.save_data()
    }
}
