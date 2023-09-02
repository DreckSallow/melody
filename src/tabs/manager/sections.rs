use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    style::Style,
    widgets::{List, ListItem},
};

use crate::{
    component::Component,
    event::AppEvent,
    handlers::music::PlaylistInfo,
    select,
    utils::Condition,
    view::{
        ui::ui_block,
        widgets::{
            input::Input,
            list::{SelectList, WRow},
            state::input::InputState,
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
                "Playlists",
                select!(state.focus_i == 1, Color::Cyan, Color::White),
            ))
            .highlight_symbol("> ")
            .highlight_style(Style::default().bg(Color::Cyan));
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
                    // Is down or UP
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
        let input = Input::default().block(ui_block(
            "Create",
            select!(state.focus_i == 0, Color::Cyan, Color::White),
        ));
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
                    state.input_state.insert(l.to_string().as_str());
                    state.input_state.next_index();
                }
                KeyCode::Delete | KeyCode::Backspace => {
                    state.input_state.remove_ch();
                    state.input_state.back_index();
                }
                KeyCode::Enter => {
                    let input = state.input_state.text().to_string();
                    let mut contains = false;
                    // Store the playlists
                    for play in &state.playlists {
                        if play.name == input {
                            contains = true;
                            break;
                        }
                    }
                    if !contains {
                        state.playlists.push(PlaylistInfo {
                            name: state.input_state.text().into(),
                            songs: Vec::new(),
                        });
                        state.input_state = InputState::default();
                    }
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
            .index_style(Style::default().bg(Color::LightGreen))
            .highlight_style(Style::default().bg(Color::Cyan))
            .block(ui_block(
                "Songs",
                select!(state.focus_i == 2, Color::Cyan, Color::White),
            ))
            .highlight_symbol("> ");
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
