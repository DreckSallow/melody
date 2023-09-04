use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::*,
    style::Style,
    widgets::{List, ListItem},
};

use crate::{
    component::Component,
    event::AppEvent,
    select,
    utils::Condition,
    view::{
        ui::ui_block,
        widgets::{
            input::Input,
            list::{SelectList, WRow},
        },
    },
};

use super::MusicManagerState;

pub struct PlaylistsManager;

impl Component for PlaylistsManager {
    type State = MusicManagerState;
    fn render(
        &mut self,
        frame: &mut crate::component::FrameType,
        area: Rect,
        state: &mut Self::State,
    ) {
        let playlists: Vec<ListItem> = state
            .playlists
            .iter()
            .map(|p| ListItem::new(p.name.as_str()))
            .collect();
        let playlist_list = List::new(playlists)
            .block(ui_block(
                format!(" Playlists ({})", state.playlists.len()),
                select!(state.focus_i == 1, Color::Cyan, Color::White),
            ))
            .highlight_symbol("🚀 ")
            .highlight_style(Style::default().bg(Color::Blue));
        frame.render_stateful_widget(playlist_list, area, state.list_playlists.state());
    }
    fn on_event(&mut self, event: &AppEvent, state: &mut Self::State) {
        if let AppEvent::Key(key_event) = event {
            if key_event.kind != KeyEventKind::Press {
                return;
            }
            match key_event.code {
                KeyCode::Down => {
                    state.update_playlist();
                    state.list_playlists.next(state.playlists.len());
                    state.update_select_list();
                }
                KeyCode::Up => {
                    state.update_playlist();
                    state.list_playlists.previous(state.playlists.len());
                    state.update_select_list();
                }
                KeyCode::Enter => {
                    // FIXME: Change the selected index of ListController
                    // because, the select index is changed when The controller
                    // state.update_select_list();

                    //We can change the focus section
                    // state.focus_i = 2
                }
                KeyCode::Char('d') => {
                    state.delete_playlist();
                }

                _ => {}
            }
        }
    }
}

pub struct InputPlaylist;

impl Component for InputPlaylist {
    type State = MusicManagerState;
    fn render(
        &mut self,
        frame: &mut crate::component::FrameType,
        area: Rect,
        state: &mut Self::State,
    ) {
        let input = Input::default()
            .block(ui_block(
                " Create ",
                select!(state.focus_i == 0, Color::Cyan, Color::White),
            ))
            .cursor_visibility(state.focus_i == 0)
            .cursor_style(Style::default().bg(Color::Blue));

        frame.render_stateful_widget(input, area, &mut state.input_state)
    }
    fn on_event(&mut self, event: &AppEvent, state: &mut Self::State) {
        if let AppEvent::Key(key_event) = event {
            if key_event.kind != KeyEventKind::Press {
                return;
            }
            match key_event.code {
                KeyCode::Right => {
                    state.input_state.next_index();
                }
                KeyCode::Left => {
                    state.input_state.back_index();
                }
                KeyCode::Char(l) => {
                    if let KeyModifiers::CONTROL = key_event.modifiers {
                        if l == '1' {
                            return;
                        }
                    }
                    state.input_state.insert(l.to_string().as_str());
                    state.input_state.next_index();
                }
                KeyCode::Delete | KeyCode::Backspace => {
                    state.input_state.remove_ch();
                    state.input_state.back_index();
                }
                KeyCode::Enter => {
                    state.create_playlist();
                }

                _ => {}
            }
        }
    }
}

pub struct SongsManager;

impl Component for SongsManager {
    type State = MusicManagerState;
    fn render(
        &mut self,
        frame: &mut crate::component::FrameType,
        area: ratatui::prelude::Rect,
        state: &mut Self::State,
    ) {
        let songs_rows = state.songs.iter().map(|s| {
            let cells = [s.file_name.clone().unwrap_or("----".into())];
            WRow::new(cells)
        });

        let songs_table = SelectList::new(songs_rows)
            .header(WRow::new(["Name"]).with_height(1))
            .widths(&[Constraint::Percentage(100)])
            .index_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::ITALIC),
            )
            .highlight_style(Style::default().bg(Color::Blue))
            .block(ui_block(
                format!(" Songs ({})", state.songs.len()),
                select!(state.focus_i == 2, Color::Cyan, Color::White),
            ))
            .highlight_symbol("🎵 ");
        frame.render_stateful_widget(songs_table, area, &mut state.list_songs);
    }
    fn on_event(&mut self, event: &crate::event::AppEvent, state: &mut Self::State) {
        if let AppEvent::Key(key_event) = event {
            if key_event.kind != KeyEventKind::Press {
                return;
            }
            match key_event.code {
                KeyCode::Down => {
                    state.list_songs.next();
                }
                KeyCode::Up => {
                    state.list_songs.previous();
                }
                KeyCode::Enter => state.list_songs.toggle_select(),

                _ => {}
            }
        }
    }
}
