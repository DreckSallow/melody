use crate::{utils::Condition, view::ui::ui_block};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{block::Title, Gauge, Paragraph},
};
use std::rc::Rc;

use crate::{
    app::AppState,
    component::{Component, FrameType},
    event::AppEvent,
    loaders::player::PlaylistSong,
    select,
    tabs::log::{LogMessage, LogsState},
};

use super::state::PlayerState;

mod handler;
use handler::AudioHandler;

pub struct AudioPlayer {
    handler: AudioHandler,
    logger: LogsState,
    indices: Option<(usize, usize)>,
    pub is_focus: bool,
}

impl AudioPlayer {
    pub fn build(
        app_state: &AppState,
        song: Option<PlaylistSong>,
        indices: Option<(usize, usize)>,
    ) -> Result<Self> {
        let logger = Rc::clone(&app_state.log);
        let mut handler = AudioHandler::try_default()?;
        if let Err(e) = handler.set_song(song) {
            logger.borrow_mut().push(LogMessage::Error(e.to_string()))
        }

        Ok(Self {
            handler,
            is_focus: false,
            indices,
            logger,
        })
    }
    fn on_tick(&mut self, state: &mut PlayerState) {
        if self.indices != state.indices() {
            let indices_opt = state.indices().and_then(|i| self.indices.map(|si| (si, i)));
            if let Some((state_indices, indices)) = indices_opt {
                if state_indices.0 != indices.0 {
                    // The playlist was changed.
                    self.handler.pause();
                }
            }

            self.indices = state.indices();
            if let Err(e) = self.handler.set_song(state.selected_audio().cloned()) {
                self.logger
                    .borrow_mut()
                    .push(LogMessage::Error(e.to_string()))
            };
        }
        if self.handler.is_end_song() {
            if let Some((p_i, s_i)) = self.indices {
                if state.library.playlists[p_i].songs.get(s_i + 1).is_some() {
                    state.audio_selected = Some(s_i + 1);
                    if let Err(e) = self.handler.set_song(state.selected_audio().cloned()) {
                        self.logger
                            .borrow_mut()
                            .push(LogMessage::Error(e.to_string()))
                    }
                };
            }
        }
    }
}

impl Component for AudioPlayer {
    type State = PlayerState;
    fn render(
        &mut self,
        frame: &mut FrameType,
        area: ratatui::prelude::Rect,
        state: &mut Self::State,
    ) {
        self.on_tick(state);

        let block = ui_block(
            Title::from(select!(self.handler.song(), "Now Playing", "Not Song"))
                .alignment(Alignment::Center),
            select!(self.is_focus, Color::Cyan, Color::White),
        );
        frame.render_widget(block, area);

        match self.handler.song().cloned() {
            Some(song) => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .vertical_margin(2)
                    .horizontal_margin(1)
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                    .split(area);
                let header_block =
                    Paragraph::new(song.file_name.clone().unwrap_or("No name".into()))
                        .style(Style::default())
                        .alignment(Alignment::Center);
                frame.render_widget(header_block, chunks[0]);

                let (seconds, percent) = self.handler.percentage_info(song.duration);

                let gauge = Gauge::default()
                    .gauge_style(Style::default().fg(Color::Red))
                    .percent(percent as u16)
                    .label(format!("{} / {}", seconds, song.duration.as_secs()));
                frame.render_widget(gauge, chunks[1]);
            }
            None => {}
        }
    }
    fn on_event(&mut self, event: &AppEvent, _state: &mut Self::State) {
        match *event {
            AppEvent::Quit => {
                self.handler.finish();
            }
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                };
                match key_event.code {
                    KeyCode::Char(' ') => self.handler.toggle_action(),

                    _ => {}
                }
            }
        }
    }
    fn is_focus(&self) -> bool {
        self.is_focus
    }
}
