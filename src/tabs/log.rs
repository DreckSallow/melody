use std::{cell::RefCell, rc::Rc};

use anyhow::Result;
use ratatui::{
    prelude::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, Borders, Paragraph},
};
use time::OffsetDateTime;

use crate::{
    app::AppState,
    component::{Component, FinishableComp},
};

pub type LogsState = Rc<RefCell<Vec<LogMessage>>>;

pub enum LogType {
    Info,
    Warn,
    Error,
}

pub struct LogMessage {
    message: String,
    log_type: LogType,
}

impl LogMessage {
    fn get_time() -> String {
        let now = OffsetDateTime::now_local().unwrap_or(OffsetDateTime::now_utc());
        let (h, m, s) = now.to_hms();
        format!("[{} {h}:{m}:{s}]", now.date())
    }
    fn format_text(m: String, typ: &str) -> String {
        format!("{} [{typ}] {m}", Self::get_time(),)
    }
    pub fn info<T: Into<String>>(message: T) -> Self {
        LogMessage {
            message: Self::format_text(message.into(), "INFO"),
            log_type: LogType::Info,
        }
    }
    pub fn warn<T: Into<String>>(message: T) -> Self {
        LogMessage {
            message: Self::format_text(message.into(), "WARN"),
            log_type: LogType::Warn,
        }
    }
    pub fn error<T: Into<String>>(message: T) -> Self {
        LogMessage {
            message: Self::format_text(message.into(), "ERROR"),
            log_type: LogType::Error,
        }
    }
    pub fn text(&self) -> String {
        self.message.clone()
    }
    pub fn color(&self) -> Color {
        match self.log_type {
            LogType::Info => Color::White,
            LogType::Warn => Color::Yellow,
            LogType::Error => Color::Red,
        }
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
        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Logs ")
            .border_style(Style::default().fg(Color::Cyan));

        let lines: Vec<Line> = state
            .log
            .borrow()
            .iter()
            .map(|log| {
                let mut line = Line::from(log.text());
                line.patch_style(Style::default().fg(log.color()));
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
