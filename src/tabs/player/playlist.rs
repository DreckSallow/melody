use std::{cell::RefCell, rc::Rc};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::{component::Component, event::AppEvent, view::controllers::table::TableController};

use super::state::{PlayerState, PlayerStateAction, PlayerStateReactive};

pub struct Playlist {
    playlist_name: String,
    songs: Vec<Vec<String>>,
    table_controller: TableController,
    pub is_focus: bool,
    parent_state: Rc<RefCell<PlayerStateReactive>>,
}

impl Playlist {
    pub fn build(songs: &[Vec<String>], state: &Rc<RefCell<PlayerStateReactive>>) -> Self {
        let index = if songs.is_empty() { None } else { Some(0) };
        Self {
            playlist_name: "List".into(),
            songs: songs.into(),
            table_controller: TableController::default().with_select(index),
            is_focus: false,
            parent_state: Rc::clone(&state),
        }
    }
    pub fn set_songs(&mut self, songs: &[Vec<String>]) {
        self.songs = songs.into();
        let index = if songs.is_empty() { None } else { Some(0) };
        self.table_controller.select(index);
    }
    pub fn list_changes(&mut self, action: &PlayerStateAction, state: &PlayerState) {
        if let PlayerStateAction::SetPlaylist = *action {
            if let Some(ref index_play) = state.playlist_selected {
                if let Some(playlist) = state.library.playlists.get(*index_play) {
                    let songs: Vec<Vec<String>> = playlist
                        .songs
                        .iter()
                        .map(|song| {
                            let copy_song = song.clone();
                            vec![copy_song.title.unwrap_or("---".into())]
                        })
                        .collect();
                    self.playlist_name = playlist.name.clone();
                    self.set_songs(&songs);
                    self.is_focus = true;
                }
            }
        }
    }
}

impl Component for Playlist {
    fn render(&mut self, frame: &mut crate::component::FrameType, area: ratatui::prelude::Rect) {
        let styled = if self.is_focus {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };

        let playlist_block = Block::default()
            .title(self.playlist_name.as_str())
            .borders(Borders::ALL)
            .border_style(styled);

        let headers_cells = ["Name"].iter().map(|header| Cell::from(*header));
        let header = Row::new(headers_cells)
            .height(1)
            .style(Style::default().fg(ratatui::style::Color::Blue));

        let items = self.songs.iter().map(|item| {
            let cells = item.iter().map(|text| Cell::from(text.clone()));
            Row::new(cells).height(1)
        });
        let table_block = Table::new(items)
            .header(header)
            .block(playlist_block)
            .highlight_style(Style::default().bg(ratatui::style::Color::Cyan))
            .highlight_symbol(">")
            .widths(&[
                Constraint::Percentage(70),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ])
            .highlight_symbol("🎵 ");
        frame.render_stateful_widget(table_block, area, self.table_controller.state())
    }

    fn on_event(&mut self, event: &AppEvent) {
        match *event {
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                }
                match key_event.code {
                    KeyCode::Down => self.table_controller.next(self.songs.len()),
                    KeyCode::Up => self.table_controller.previous(self.songs.len()),
                    KeyCode::Enter => {
                        // let song_opt = &self
                        //     .table_controller
                        //     .selected()
                        //     .and_then(|index| self.songs.get(index))
                        //     .and_then(|play| play.get(0).cloned());

                        self.parent_state.borrow_mut().dispatch(
                            PlayerStateAction::SetAudio,
                            |state| {
                                state.audio_selected = self.table_controller.selected();
                                // state.audio_selected = song_opt.clone()
                            },
                        );
                    }
                    _ => {}
                }
            }
            AppEvent::Quit => {}
        }
    }
    fn is_focus(&self) -> bool {
        self.is_focus
    }
}
