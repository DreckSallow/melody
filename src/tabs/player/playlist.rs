use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::Constraint,
    style::{Color, Style},
    widgets::{Block, Borders, Cell, Row, Table},
};

use crate::{component::Component, view::controllers::table::TableController};

use super::state::{PlayerState, PlayerStateAction};

pub struct Playlist {
    songs: Vec<Vec<String>>,
    table_controller: TableController,
    pub is_focus: bool,
}

impl Playlist {
    pub fn build(songs: &[Vec<String>]) -> Self {
        let index = if songs.is_empty() { None } else { Some(0) };
        Self {
            songs: songs.into(),
            table_controller: TableController::default().with_select(index),
            is_focus: false,
        }
    }
    pub fn set_songs(&mut self, songs: &[Vec<String>]) {
        self.songs = songs.into();
        let index = if songs.is_empty() { None } else { Some(0) };
        self.table_controller.select(index);
    }
    pub fn list_changes(&mut self, action: &PlayerStateAction, state: &PlayerState) {
        if let PlayerStateAction::SetPlaylist = *action {
            if let Some(ref selected) = state.playlist_selected {
                let playlist = state
                    .library
                    .playlists
                    .iter()
                    .find(|play| play.name == *selected);
                if let Some(playlist) = playlist {
                    let songs: Vec<Vec<String>> = playlist
                        .songs
                        .iter()
                        .map(|song| vec![song.to_string()])
                        .collect();
                    self.set_songs(&songs)
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

        let title_block = match Some("List") {
            Some(s) => s,
            None => "List",
        };

        let playlist_block = Block::default()
            .title(title_block)
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
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .highlight_symbol("🎵 ");
        frame.render_stateful_widget(table_block, area, self.table_controller.state())
    }

    fn on_event(&mut self, event: &crossterm::event::KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }
        match event.code {
            KeyCode::Down => self.table_controller.next(self.songs.len()),
            KeyCode::Up => self.table_controller.previous(self.songs.len()),
            _ => {}
        }
    }
    fn is_focus(&self) -> bool {
        self.is_focus
    }
}
