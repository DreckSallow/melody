use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::{
    app::AppState,
    component::{Component, FinishableComp},
};

pub type LogsState = Rc<RefCell<Vec<LogMessage>>>;

pub enum LogMessage {
    Info(String),
    Warn(String),
    Error(String),
}

impl LogMessage {
    pub fn text(&self) -> String {
        let s = match self {
            LogMessage::Info(s) => s,
            LogMessage::Warn(s) => s,
            LogMessage::Error(s) => s,
        };
        s.clone()
    }
}

pub struct LogTab;

impl LogTab {
    pub fn build() -> Self {
        Self
    }
}

impl Component for LogTab {
    type State = AppState;
    fn render(
        &mut self,
        frame: &mut crate::component::FrameType,
        area: Rect,
        state: &mut Self::State,
    ) {
        let block = Block::default().borders(Borders::ALL).title(" Logs ");

        let lines: Vec<Line> = state
            .log
            .borrow()
            .iter()
            .map(|log| {
                let styled = match log {
                    LogMessage::Info(_) => Color::Green,
                    LogMessage::Warn(_) => Color::Yellow,
                    LogMessage::Error(_) => Color::Red,
                };
                let mut line = Line::from(log.text().clone());
                line.patch_style(Style::default().fg(styled));
                line
            })
            .collect();

        let paragraph = Paragraph::new(lines).block(block);

        frame.render_widget(paragraph, area);
    }
    fn on_event(&mut self, _event: &crate::event::AppEvent, _state: &mut Self::State) {}
}

impl FinishableComp for LogTab {
    type Res = ();
    fn finish(&mut self) -> Result<Self::Res> {
        Ok(())
    }
}
