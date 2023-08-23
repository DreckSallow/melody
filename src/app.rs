use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Tabs},
};

use crate::{
    component::{Component, FrameType},
    event::AppEvent,
    tabs::{
        log::{LogTab, LogsState},
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

type TabComponent<'a> = (&'a str, Box<dyn Component<State = AppState>>);

type TabsType<'a> = Vec<TabComponent<'a>>;

pub struct App {
    state: AppState,
    tabs: TabsType<'static>,
    tab_index: usize,
}

impl App {
    pub fn build() -> Result<Self> {
        let state = AppState::default();
        let player: TabComponent<'static> = ("player", Box::new(PlayerTab::build(&state)?));
        let log: TabComponent<'static> = ("Log", Box::new(LogTab::build()?));
        let tabs: TabsType<'static> = vec![player, log];
        Ok(App {
            tabs,
            tab_index: 0,
            state,
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
            .block(Block::default().borders(Borders::ALL).title("tabs"))
            .select(self.tab_index)
            .highlight_style(Style::default().bg(Color::Red));

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
                if self.tab_index + 1 >= self.tabs.len() {
                    self.tab_index = 0;
                } else {
                    self.tab_index += 1
                }
            }
        }
        let tab_info = self.tabs.get_mut(self.tab_index);
        if let Some((_, section)) = tab_info {
            section.on_event(event, &mut self.state);
        }
    }
}
