use std::io::Stdout;

use ratatui::{
    prelude::{CrosstermBackend, Rect},
    Frame,
};

use crate::event::AppEvent;

pub(crate) type FrameType<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait Component {
    fn render(&mut self, frame: &mut FrameType, area: Rect);
    fn on_event(&mut self, event: &AppEvent);
    fn is_focus(&self) -> bool {
        false
    }
}
