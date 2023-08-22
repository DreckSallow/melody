// use anyhow::Result;

use crate::{
    component::{Component, FrameType},
    event::AppEvent,
    view::controllers::list::ListController,
};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use super::state::PlayerState;

pub struct PlayerLibrary {
    list_controller: ListController,
    pub is_focus: bool,
}

impl PlayerLibrary {
    pub fn build(index: Option<usize>) -> Self {
        Self {
            list_controller: ListController::default().with_select(index),
            is_focus: false,
        }
    }
}

impl Component for PlayerLibrary {
    type State = PlayerState;
    fn render(&mut self, frame: &mut FrameType, area: ratatui::prelude::Rect, state: &Self::State) {
        let styled = if self.is_focus {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };
        let section = Block::default()
            .title("Playlist")
            .borders(Borders::ALL)
            .border_style(styled);

        let items: Vec<ListItem> = state
            .library
            .playlists
            .iter()
            .map(|playlist| ListItem::new(playlist.name.as_str()))
            .collect();

        let list_block = List::new(items)
            .block(section)
            .highlight_style(Style::default().bg(Color::Cyan))
            .highlight_symbol("🚀 ");
        frame.render_stateful_widget(list_block, area, self.list_controller.state())
    }
    fn on_event(&mut self, event: &AppEvent, state: &mut Self::State) {
        match *event {
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                }
                match key_event.code {
                    KeyCode::Down => self.list_controller.next(state.library.playlists.len()),
                    KeyCode::Up => self.list_controller.previous(state.library.playlists.len()),
                    KeyCode::Enter => {
                        state.playlist_selected = self.list_controller.selected();
                        if let Some(length) = state.selected_playlist().map(|p| p.songs.len()) {
                            if length > 0 {
                                state.audio_selected = Some(0);
                            }
                        }
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
