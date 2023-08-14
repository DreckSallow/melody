use std::io::Stdout;

use crossterm::event::KeyEvent;
use ratatui::{
    prelude::{CrosstermBackend, Rect},
    Frame,
};

pub(crate) type FrameType<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait Component {
    fn render(&mut self, frame: &mut FrameType, area: Rect);
    fn on_event(&mut self, event: &KeyEvent);
    fn is_focus(&self) -> bool {
        false
    }
}
