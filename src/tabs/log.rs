use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::{app::AppState, component::Component};

pub type LogsState = Rc<RefCell<Vec<LogMessage>>>;

pub enum LogMessage {
    Info(String),
    Warn(String),
    Error(String),
}

pub struct LogTab {}

impl LogTab {
    pub fn build() -> Result<Self> {
        Ok(Self {})
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
        let block = Block::default().borders(Borders::ALL).title("Logs");

        let lines: Vec<Line> = state
            .log
            .borrow()
            .iter()
            .map(|log| {
                let log_text = match log {
                    LogMessage::Info(t) => t.to_string(),
                    LogMessage::Warn(t) => t.to_string(),
                    LogMessage::Error(t) => t.to_string(),
                };
                let styled = match log {
                    LogMessage::Info(_) => Color::Green,
                    LogMessage::Warn(_) => Color::Yellow,
                    LogMessage::Error(_) => Color::Red,
                };
                let mut line = Line::from(log_text);
                line.patch_style(Style::default().fg(styled));
                line
            })
            .collect();

        let paragraph = Paragraph::new(lines).block(block);

        frame.render_widget(paragraph, area);
    }
    fn on_event(&mut self, _event: &crate::event::AppEvent, _state: &mut Self::State) {}
}
