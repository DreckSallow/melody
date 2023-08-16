use anyhow::Result;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Tabs},
};

use crate::{
    component::{Component, FrameType},
    event::AppEvent,
    tabs::player::PlayerTab,
};

type TabsType<'a> = Vec<(&'a str, Box<dyn Component>)>;

pub struct App {
    tabs: TabsType<'static>,
    tab_index: usize,
}

impl App {
    pub fn build() -> Result<Self> {
        let tabs: TabsType<'static> = vec![("player", Box::new(PlayerTab::build()?))];
        Ok(App { tabs, tab_index: 0 })
    }
}

impl Component for App {
    fn render(&mut self, frame: &mut FrameType, area: ratatui::prelude::Rect) {
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
            section.render(frame, chunks[1]);
        }
    }
    fn on_event(&mut self, event: &AppEvent) {
        let tab_info = self.tabs.get_mut(self.tab_index);
        if let Some((_, section)) = tab_info {
            section.on_event(event);
        }
    }
}
