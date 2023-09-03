use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::prelude::*;

use crate::{
    app::AppState,
    component::{Component, FinishableComp, FrameType},
    event::AppEvent,
    handlers::music::MusicHandler,
};

mod sections;
mod state;
use self::{
    sections::{AudioPlayer, PlayerLibrary, Playlist},
    state::PlayerState,
};

pub struct PlayerTab {
    state: PlayerState,
    library_section: PlayerLibrary,
    playlist_section: Playlist,
    audio_section: AudioPlayer,
}

impl PlayerTab {
    pub fn build(_app_state: &AppState) -> Result<Self> {
        let state = PlayerState::new(MusicHandler::load_playlists()?);

        Ok(Self {
            state,
            library_section: PlayerLibrary,
            playlist_section: Playlist,
            audio_section: AudioPlayer,
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
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
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
                            self.state.focus_i = 1;
                        }
                        KeyCode::Char('1') => {
                            self.state.focus_i = 0;
                        }
                        KeyCode::Char('3') => {
                            self.state.focus_i = 2;
                        }
                        _ => {}
                    }
                }

                match self.state.focus_i {
                    0 => self.library_section.on_event(event, &mut self.state),
                    1 => self.playlist_section.on_event(event, &mut self.state),
                    2 => self.audio_section.on_event(event, &mut self.state),
                    _ => {}
                }
            }
        }
    }
}

impl FinishableComp for PlayerTab {
    type Res = ();
    fn finish(&mut self) -> Result<Self::Res> {
        self.state.audio_handler.finish();
        Ok(())
    }
}
