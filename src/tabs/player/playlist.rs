use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::Constraint,
    style::{Color, Style},
    widgets::{Cell, Row, Table},
};

use crate::{
    component::Component,
    event::AppEvent,
    select,
    utils::Condition,
    view::{controllers::table::TableController, ui::ui_block},
};

use super::state::PlayerState;

pub struct Playlist {
    indices: Option<(usize, usize)>,
    table_controller: TableController,
    pub is_focus: bool,
}

impl Playlist {
    pub fn build(playlist: Option<(usize, usize)>, index: Option<usize>) -> Self {
        Self {
            indices: playlist,
            table_controller: TableController::default().with_select(index),
            is_focus: false,
        }
    }
    pub fn on_tick(&mut self, state: &PlayerState) {
        if self.indices != state.indices() {
            self.table_controller.select(state.audio_selected);
            self.indices = state.indices()
        }
    }
}

impl Component for Playlist {
    type State = PlayerState;
    fn render(
        &mut self,
        frame: &mut crate::component::FrameType,
        area: ratatui::prelude::Rect,
        state: &mut Self::State,
    ) {
        self.on_tick(state);

        let data = if let Some(playlist) = state.playlists.get(state.playlist_selected.unwrap_or(0))
        {
            let songs_info: Vec<Vec<String>> = playlist
                .songs
                .iter()
                .map(|s| vec![s.file_name.clone().unwrap_or("----".into())])
                .collect();

            (playlist.name.clone(), songs_info)
        } else {
            ("List".into(), Vec::new())
        };

        let playlist_block = ui_block(
            data.0.as_str(),
            select!(self.is_focus, Color::Cyan, Color::White),
        );
        let headers_cells = ["Name"].iter().map(|header| Cell::from(*header));
        let header = Row::new(headers_cells)
            .height(1)
            .style(Style::default().fg(ratatui::style::Color::Blue));

        let items = data.1.iter().map(|item| {
            let cells = item.iter().map(|text| Cell::from(text.clone()));
            Row::new(cells).height(1)
        });
        let table_block = Table::new(items)
            .header(header)
            .block(playlist_block)
            .highlight_style(Style::default().bg(ratatui::style::Color::Cyan))
            .widths(&[
                Constraint::Percentage(70),
                Constraint::Percentage(15),
                Constraint::Percentage(15),
            ])
            .highlight_symbol("🎵 ");
        frame.render_stateful_widget(table_block, area, self.table_controller.state())
    }

    fn on_event(&mut self, event: &AppEvent, state: &mut Self::State) {
        match *event {
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                }
                let songs = state
                    .playlists
                    .get(state.playlist_selected.unwrap_or(0))
                    .map(|p| p.songs.len())
                    .unwrap_or(0);
                match key_event.code {
                    KeyCode::Down => self.table_controller.next(songs),
                    KeyCode::Up => self.table_controller.previous(songs),
                    KeyCode::Enter => {
                        state.audio_selected = self.table_controller.selected();
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
