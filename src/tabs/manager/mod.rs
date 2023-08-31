use std::path::PathBuf;

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    prelude::*,
    style::Color,
    widgets::{List, ListItem},
};

use crate::{
    app::AppState,
    component::{Component, FinishableComp},
    event::AppEvent,
    handlers::music::{MusicHandler, PlaylistInfo, PlaylistSong},
    select,
    utils::Condition,
    view::{
        controllers::list::ListController,
        ui::ui_block,
        widgets::{
            list::{SelectList, WRow},
            state::SelectListState,
        },
    },
};

pub struct PlaylistManager {
    list_songs: SelectListState,
    list_playlists: ListController,
    playlists: Vec<PlaylistInfo>,
    songs: Vec<PlaylistSong>,
    focus_i: u8,
}

impl PlaylistManager {
    pub fn build(p: &PathBuf) -> Result<Self> {
        let songs = MusicHandler::load_songs(p)?;
        let playlists = MusicHandler::load_playlists()?;
        let selecteds = if let Some(play) = playlists.get(0) {
            let songs_paths: Vec<PathBuf> = play.songs.iter().map(|s| s.path.clone()).collect();
            songs
                .iter()
                .enumerate()
                .filter_map(|(i, s)| songs_paths.contains(&s.path).then_some(i))
                .collect()
        } else {
            Vec::new()
        };
        Ok(Self {
            list_playlists: ListController::default().with_select(select!(
                playlists.is_empty(),
                None,
                Some(0)
            )),
            playlists,
            list_songs: SelectListState::default()
                .with_len(songs.len())
                .with_selecteds(selecteds)
                .with_index(select!(songs.is_empty(), None, Some(0))),
            songs,
            focus_i: 0,
        })
    }
    fn update_select_list(&mut self) {
        if let Some(play) = self
            .list_playlists
            .selected()
            .and_then(|i| self.playlists.get(i))
        {
            let songs_paths: Vec<PathBuf> = play.songs.iter().map(|s| s.path.clone()).collect();
            let selecteds: Vec<usize> = self
                .songs
                .iter()
                .enumerate()
                .filter_map(|(i, s)| songs_paths.contains(&s.path).then_some(i))
                .collect();

            self.list_songs = SelectListState::default()
                .with_len(self.songs.len())
                .with_selecteds(selecteds)
                .with_index(select!(self.songs.is_empty(), None, Some(0)));
        }
    }
    fn update_playlist(&mut self) {
        if let Some(i) = self.list_playlists.selected() {
            if let Some(playlist) = self.playlists.get_mut(i) {
                playlist.songs = [].into();
                let selecteds = self.list_songs.selecteds();
                for (i, song) in self.songs.iter().enumerate() {
                    if selecteds.contains(&i) {
                        playlist.songs.push(song.clone());
                    }
                }
            }
        }
    }
    fn save_data(&mut self) -> Result<()> {
        self.update_playlist();
        MusicHandler::save_playlists(self.playlists.clone())
    }
}

impl Component for PlaylistManager {
    type State = AppState;
    fn render(
        &mut self,
        frame: &mut crate::component::FrameType,
        area: ratatui::prelude::Rect,
        _state: &mut Self::State,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(40), Constraint::Percentage(50)])
            .split(area);

        let playlists: Vec<ListItem> = self
            .playlists
            .iter()
            .map(|p| ListItem::new(p.name.as_str()))
            .collect();
        let playlist_list = List::new(playlists)
            .block(ui_block(
                "Playlists",
                select!(self.focus_i == 0, Color::Cyan, Color::White),
            ))
            .highlight_symbol("> ")
            .highlight_style(Style::default().bg(Color::Cyan));
        frame.render_stateful_widget(playlist_list, chunks[0], self.list_playlists.state());

        let songs_rows = self.songs.iter().map(|s| {
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
                select!(self.focus_i == 1, Color::Cyan, Color::White),
            ))
            .highlight_symbol("> ");
        frame.render_stateful_widget(songs_table, chunks[1], &mut self.list_songs);
    }
    fn on_event(&mut self, event: &AppEvent, _state: &mut Self::State) {
        match event {
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                }

                if let KeyModifiers::CONTROL = key_event.modifiers {
                    match key_event.code {
                        KeyCode::Char('1') => {
                            self.focus_i = 0;
                        }
                        KeyCode::Char('2') => {
                            self.focus_i = 1;
                        }
                        _ => {}
                    }
                }
                match key_event.code {
                    KeyCode::Down => {
                        match self.focus_i {
                            0 => {
                                self.update_playlist();
                                self.list_playlists.next(self.playlists.len());
                                self.update_select_list()
                            }
                            1 => self.list_songs.next(),
                            _ => {}
                        };
                    }
                    KeyCode::Up => {
                        match self.focus_i {
                            0 => {
                                self.update_playlist();
                                self.list_playlists.previous(self.playlists.len());
                                self.update_select_list()
                            }
                            1 => self.list_songs.previous(),
                            _ => {}
                        };
                    }
                    KeyCode::Enter => {
                        if self.focus_i == 1 {
                            self.list_songs.toggle_select()
                        }
                    }

                    _ => {}
                }
            }
            AppEvent::Quit => {}
        }
    }
}

impl FinishableComp for PlaylistManager {
    type Res = ();
    fn finish(&mut self) -> Result<Self::Res> {
        self.save_data()
    }
}
