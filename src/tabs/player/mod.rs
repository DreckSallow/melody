use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::prelude::*;

use crate::{
    component::{Component, FrameType},
    event::AppEvent,
    loaders::load_playlists,
};

use self::{
    audio::AudioPlayer,
    library::PlayerLibrary,
    playlist::Playlist,
    state::{PlayerState, PlayerStateReactive},
};

mod audio;
mod library;
mod playlist;
mod state;

pub struct PlayerTab {
    // state: PlayerStateType,
    library_section: Rc<RefCell<PlayerLibrary>>,
    playlist_section: Rc<RefCell<Playlist>>,
    audio_section: Rc<RefCell<AudioPlayer>>,
}

impl PlayerTab {
    pub fn build() -> Result<Self> {
        let state = Rc::new(RefCell::new(PlayerState::create(load_playlists()?)));
        let reactive_state = Rc::new(RefCell::new(PlayerStateReactive::from(&state)));
        let library = {
            let playlists: Vec<String> = state
                .borrow()
                .library
                .playlists
                .iter()
                .map(|pl| pl.name.clone())
                .collect();
            let mut l = PlayerLibrary::build(&playlists, &reactive_state);
            l.is_focus = true;
            Rc::new(RefCell::new(l))
        };
        let playlist = {
            let p = Playlist::build(&[], &reactive_state);
            Rc::new(RefCell::new(p))
        };
        let audio = {
            let a = AudioPlayer::build()?;
            Rc::new(RefCell::new(a))
        };
        {
            let playlist_cloned = Rc::clone(&playlist);
            let audio_cloned = Rc::clone(&audio);
            reactive_state
                .borrow_mut()
                .subscribe(move |act, st| match *act {
                    state::PlayerStateAction::SetPlaylist => {
                        let mut play_sec = playlist_cloned.borrow_mut();
                        play_sec.list_changes(act, st);
                    }
                    state::PlayerStateAction::SetAudio => {
                        audio_cloned.borrow_mut().on_change(act, st)
                    }
                });
        }

        Ok(Self {
            // state,
            library_section: library,
            playlist_section: playlist,
            audio_section: audio,
        })
    }
}

impl Component for PlayerTab {
    fn render(&mut self, frame: &mut FrameType, area: ratatui::prelude::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
            .split(area);

        let content_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(chunks[0]);
        self.library_section
            .borrow_mut()
            .render(frame, content_chunks[0]);
        self.playlist_section
            .borrow_mut()
            .render(frame, content_chunks[1]);

        self.audio_section.borrow_mut().render(frame, chunks[1])
    }
    fn on_event(&mut self, event: &AppEvent) {
        match *event {
            AppEvent::Quit => {
                self.playlist_section.borrow_mut().on_event(event);
                self.library_section.borrow_mut().on_event(event);
                self.audio_section.borrow_mut().on_event(event);
            }
            AppEvent::Key(key_event) => {
                if let KeyModifiers::CONTROL = key_event.modifiers {
                    match key_event.code {
                        KeyCode::Char('2') => {
                            self.playlist_section.borrow_mut().is_focus = true;
                            self.library_section.borrow_mut().is_focus = false;
                            self.audio_section.borrow_mut().is_focus = false
                        }
                        KeyCode::Char('1') => {
                            self.library_section.borrow_mut().is_focus = true;
                            self.playlist_section.borrow_mut().is_focus = false;
                            self.audio_section.borrow_mut().is_focus = false
                        }
                        KeyCode::Char('3') => {
                            self.playlist_section.borrow_mut().is_focus = false;
                            self.library_section.borrow_mut().is_focus = false;
                            self.audio_section.borrow_mut().is_focus = true
                        }
                        _ => {}
                    }
                }

                if self.playlist_section.borrow().is_focus() {
                    self.playlist_section.borrow_mut().on_event(event);
                }
                if self.library_section.borrow().is_focus() {
                    self.library_section.borrow_mut().on_event(event);
                }
                if self.audio_section.borrow().is_focus() {
                    self.audio_section.borrow_mut().on_event(event);
                }
            }
        }
    }
}
