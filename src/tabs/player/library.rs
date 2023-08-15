// use anyhow::Result;

use std::{cell::RefCell, rc::Rc};

use crate::{
    component::{Component, FrameType},
    view::controllers::list::ListController,
};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem},
};

use super::state::{PlayerStateAction, PlayerStateReactive};

pub struct PlayerLibrary {
    playlists: Vec<String>,
    list_controller: ListController,
    pub is_focus: bool,
    parent_state: Rc<RefCell<PlayerStateReactive>>,
}

impl PlayerLibrary {
    pub fn build(playlists: &[String], state: &Rc<RefCell<PlayerStateReactive>>) -> Self {
        let library = Self {
            playlists: playlists.into(),
            list_controller: ListController::default(),
            is_focus: false,
            parent_state: Rc::clone(&state),
        };

        library
    }
}

impl Component for PlayerLibrary {
    fn render(&mut self, frame: &mut FrameType, area: ratatui::prelude::Rect) {
        let styled = if self.is_focus {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };
        let section = Block::default()
            .title("Playlist")
            .borders(Borders::ALL)
            .border_style(styled);

        let items: Vec<ListItem> = self
            .playlists
            .iter()
            .map(|playlist| ListItem::new(playlist.as_str()))
            .collect();

        let list_block = List::new(items)
            .block(section)
            .highlight_style(Style::default().bg(Color::Cyan))
            .highlight_symbol("🚀 ");
        frame.render_stateful_widget(list_block, area, self.list_controller.state())
    }
    fn on_event(&mut self, event: &crossterm::event::KeyEvent) {
        if event.kind != KeyEventKind::Press {
            return;
        }
        match event.code {
            KeyCode::Down => self.list_controller.next(self.playlists.len()),
            KeyCode::Up => self.list_controller.previous(self.playlists.len()),
            KeyCode::Enter => {
                self.parent_state
                    .borrow_mut()
                    .dispatch(PlayerStateAction::SetPlaylist, |state| {
                        if let Some(index) = self.list_controller.selected() {
                            if let Some(playlist) = self.playlists.get(index) {
                                state.playlist_selected = Some(playlist.into())
                            }
                        }
                    })
            }
            _ => {}
        }
    }
    fn is_focus(&self) -> bool {
        self.is_focus
    }
}
