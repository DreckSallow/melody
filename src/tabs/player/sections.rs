use crate::{
    component::{Component, FrameType},
    event::AppEvent,
    select,
    utils::Condition,
    view::ui::ui_block,
};
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    style::{Color, Style},
    widgets::{block::Title, Cell, Gauge, List, ListItem, Paragraph, Row, Table},
};

use super::state::PlayerState;

pub struct PlayerLibrary;
impl Component for PlayerLibrary {
    type State = PlayerState;
    fn render(
        &mut self,
        frame: &mut FrameType,
        area: ratatui::prelude::Rect,
        state: &mut Self::State,
    ) {
        let section = ui_block(
            "Playlists",
            select!(state.focus_i == 0, Color::Cyan, Color::White),
        );
        let items: Vec<ListItem> = state
            .playlists
            .iter()
            .map(|playlist| ListItem::new(playlist.name.as_str()))
            .collect();

        let list_block = List::new(items)
            .block(section)
            .highlight_style(Style::default().bg(Color::Cyan))
            .highlight_symbol("🚀 ");

        frame.render_stateful_widget(list_block, area, state.list_playlists.state())
    }
    fn on_event(&mut self, event: &AppEvent, state: &mut Self::State) {
        match *event {
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                }
                match key_event.code {
                    KeyCode::Down => state.list_playlists.next(state.playlists.len()),
                    KeyCode::Up => state.list_playlists.previous(state.playlists.len()),
                    KeyCode::Enter => {
                        state.update_songs();
                    }
                    _ => {}
                }
            }
            AppEvent::Quit => {}
        }
    }
}

pub struct Playlist;
impl Playlist {
    pub fn check_audio(&self, state: &mut PlayerState) {
        if state.audio_handler.is_end_song() {
            if let Some(songs_len) = state.current_playlist().map(|p| p.songs.len()) {
                let not_is_last = state
                    .table_songs
                    .selected()
                    .map_or(false, |s| s < songs_len - 1);
                if not_is_last {
                    state.table_songs.next(songs_len);
                    state.append_song();
                }
            }
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
        self.check_audio(state);
        let data = if let Some(playlist) = state
            .list_playlists
            .selected()
            .and_then(|s| state.playlists.get(s))
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
            select!(state.focus_i == 1, Color::Cyan, Color::White),
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
        frame.render_stateful_widget(table_block, area, state.table_songs.state())
    }

    fn on_event(&mut self, event: &AppEvent, state: &mut Self::State) {
        match *event {
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                }
                let songs = state
                    .list_playlists
                    .selected()
                    .and_then(|s| state.playlists.get(s).map(|p| p.songs.len()))
                    .unwrap_or(0);
                match key_event.code {
                    KeyCode::Down => state.table_songs.next(songs),
                    KeyCode::Up => state.table_songs.previous(songs),
                    KeyCode::Enter => {
                        state.append_song();
                    }
                    _ => {}
                }
            }
            AppEvent::Quit => {}
        }
    }
}

pub struct AudioPlayer;

impl Component for AudioPlayer {
    type State = PlayerState;
    fn render(
        &mut self,
        frame: &mut FrameType,
        area: ratatui::prelude::Rect,
        state: &mut Self::State,
    ) {
        let block = ui_block(
            Title::from(select!(
                state.audio_handler.song(),
                format!(" Playing (volume: {}%) ", state.audio_handler.volume().1),
                "Not Song".to_string()
            ))
            .alignment(Alignment::Center),
            select!(state.focus_i == 2, Color::Cyan, Color::White),
        );
        frame.render_widget(block, area);

        match state.audio_handler.song().cloned() {
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

                let percent = state.audio_handler.percentage(song.duration);

                let gauge = Gauge::default()
                    .gauge_style(Style::default().fg(Color::Cyan).bg(Color::DarkGray))
                    .percent(percent as u16)
                    .label(format!(
                        "{} / {}",
                        state.audio_handler.time_format(),
                        song.duration_format
                    ));
                frame.render_widget(gauge, chunks[1]);
            }
            None => {}
        }
    }
    fn on_event(&mut self, event: &AppEvent, state: &mut Self::State) {
        match event {
            AppEvent::Quit => {
                state.audio_handler.finish();
            }
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                };
                match key_event.code {
                    KeyCode::Char(' ') => state.audio_handler.toggle_action(),
                    KeyCode::Char('m') => state.audio_handler.toggle_volume(),
                    KeyCode::Down => state.audio_handler.down_volume(),
                    KeyCode::Up => state.audio_handler.up_volume(),
                    _ => {}
                }
            }
        }
    }
}
