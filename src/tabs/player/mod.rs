use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::prelude::*;

use crate::{
    app::AppState,
    component::{Component, FinishableComp, FrameType},
    event::AppEvent,
    handlers::music::MusicHandler,
};

use self::{audio::AudioPlayer, library::PlayerLibrary, playlist::Playlist, state::PlayerState};

mod audio;
mod library;
mod playlist;
mod state;

pub struct PlayerTab {
    state: PlayerState,
    library_section: PlayerLibrary,
    playlist_section: Playlist,
    audio_section: AudioPlayer,
}

impl PlayerTab {
    pub fn build(app_state: &AppState) -> Result<Self> {
        let mut state = PlayerState::create(MusicHandler::load_playlists()?);
        if !state.playlists.is_empty() {
            state.playlist_selected = Some(0);
        }
        let mut library = PlayerLibrary::build(state.playlist_selected);
        library.is_focus = true;

        if let Some(playlist) = state.selected_playlist() {
            if !playlist.songs.is_empty() {
                state.audio_selected = Some(0);
            }
        }
        let playlist = Playlist::build(state.indices(), state.audio_selected);
        let audio =
            AudioPlayer::build(app_state, state.selected_audio().cloned(), state.indices())?;

        Ok(Self {
            state,
            library_section: library,
            playlist_section: playlist,
            audio_section: audio,
        })
    }
}

impl Component for PlayerTab {
    type State = AppState;
    fn render(
        &mut self,
        frame: &mut FrameType,
        area: ratatui::prelude::Rect,
        _state: &mut Self::State,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(area);

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);
        self.library_section
            .render(frame, content_chunks[0], &mut self.state);
        self.playlist_section
            .render(frame, content_chunks[1], &mut self.state);

        self.audio_section.render(frame, chunks[1], &mut self.state)
    }
    fn on_event(&mut self, event: &AppEvent, _state: &mut Self::State) {
        match *event {
            AppEvent::Quit => {
                self.playlist_section.on_event(event, &mut self.state);
                self.library_section.on_event(event, &mut self.state);
                self.audio_section.on_event(event, &mut self.state);
            }
            AppEvent::Key(key_event) => {
                if let KeyModifiers::CONTROL = key_event.modifiers {
                    match key_event.code {
                        KeyCode::Char('2') => {
                            self.playlist_section.is_focus = true;
                            self.library_section.is_focus = false;
                            self.audio_section.is_focus = false
                        }
                        KeyCode::Char('1') => {
                            self.library_section.is_focus = true;
                            self.playlist_section.is_focus = false;
                            self.audio_section.is_focus = false
                        }
                        KeyCode::Char('3') => {
                            self.playlist_section.is_focus = false;
                            self.library_section.is_focus = false;
                            self.audio_section.is_focus = true
                        }
                        _ => {}
                    }
                }

                if self.playlist_section.is_focus() {
                    self.playlist_section.on_event(event, &mut self.state);
                }
                if self.library_section.is_focus() {
                    self.library_section.on_event(event, &mut self.state);
                }
                if self.audio_section.is_focus() {
                    self.audio_section.on_event(event, &mut self.state);
                }
            }
        }
    }
}

impl FinishableComp for PlayerTab {
    type Res = ();
    fn finish(&mut self) -> Result<Self::Res> {
        self.audio_section.finish();
        Ok(())
    }
}
