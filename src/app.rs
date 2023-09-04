use std::{cell::RefCell, path::PathBuf, rc::Rc};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Tabs},
};

use crate::{
    component::{Component, FinishableComp, FrameType},
    data::config::ConfigData,
    event::AppEvent,
    tabs::{
        log::{LogMessage, LogTab, LogsState},
        manager::PlaylistManager,
        player::PlayerTab,
    },
};

pub struct AppState {
    pub log: LogsState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            log: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

type TabComponent<'a> = (&'a str, Box<dyn FinishableComp<Res = (), State = AppState>>);

type TabsType<'a> = Vec<TabComponent<'a>>;

pub struct App {
    state: AppState,
    tabs: TabsType<'static>,
    tab_index: usize,
    music_path: PathBuf,
}

impl App {
    pub fn build() -> Result<Self> {
        let state = AppState::default();
        let player: TabComponent<'static> = (" Player ", Box::new(PlayerTab::build(&state)?));
        let log: TabComponent<'static> = (" Log ", Box::new(LogTab::build()));

        let config = ConfigData::load()?;

        let manager: TabComponent<'static> = (
            " Manager ",
            Box::new(PlaylistManager::build(&config.music_path)?),
        );
        let tabs: TabsType<'static> = vec![player, manager, log];
        Ok(App {
            tabs,
            tab_index: 0,
            state,
            music_path: config.music_path,
        })
    }
}

impl Component for App {
    type State = Option<()>;
    fn render(
        &mut self,
        frame: &mut FrameType,
        area: ratatui::prelude::Rect,
        _state: &mut Self::State,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(area);

        let tab_titles = self.tabs.iter().map(|(tab, _)| Line::from(*tab)).collect();
        let tabs = Tabs::new(tab_titles)
            .block(Block::default().borders(Borders::ALL))
            .select(self.tab_index)
            .highlight_style(Style::default().bg(Color::Blue));

        frame.render_widget(tabs, chunks[0]);

        let tab_info = self.tabs.get_mut(self.tab_index);
        if let Some((_, section)) = tab_info {
            section.render(frame, chunks[1], &mut self.state);
        }
    }
    fn on_event(&mut self, event: &AppEvent, _state: &mut Self::State) {
        if let AppEvent::Key(key_event) = event {
            if key_event.kind != KeyEventKind::Press {
                return;
            }

            if let KeyCode::Tab = key_event.code {
                let finish_res = self.tabs[self.tab_index].1.finish();
                // Change the tab index
                if self.tab_index + 1 >= self.tabs.len() {
                    self.tab_index = 0;
                } else {
                    self.tab_index += 1
                }

                let finish_res = match finish_res {
                    Ok(()) => {
                        // Create new Tab
                        match self.tab_index {
                            0 => PlayerTab::build(&self.state).map(|p| {
                                let tb: TabComponent<'static> = (" Player ", Box::new(p));
                                tb
                            }),
                            1 => PlaylistManager::build(&self.music_path).map(|p| {
                                let tb: TabComponent<'static> = (" Manager ", Box::new(p));
                                tb
                            }),
                            2 => {
                                let tab: TabComponent<'static> =
                                    (" Log ", Box::new(LogTab::build()));
                                Ok(tab)
                            }
                            _ => {
                                unreachable!()
                            }
                        }
                    }
                    Err(e) => Err(e),
                };
                match finish_res {
                    Ok(tab) => self.tabs[self.tab_index] = tab,
                    Err(e) => self
                        .state
                        .log
                        .borrow_mut()
                        .push(LogMessage::Error(e.to_string())),
                }
            }
        }
        let tab_info = self.tabs.get_mut(self.tab_index);
        if let Some((_, section)) = tab_info {
            section.on_event(event, &mut self.state);
        }
    }
}
