use std::io::Stdout;

use ratatui::{
    prelude::{CrosstermBackend, Rect},
    Frame,
};

use crate::event::AppEvent;

pub(crate) type FrameType<'a> = Frame<'a, CrosstermBackend<Stdout>>;

pub trait Component {
    type State;
    fn render(&mut self, frame: &mut FrameType, area: Rect, _state: &Self::State);
    fn on_event(&mut self, event: &AppEvent, _state: &mut Self::State);
    fn is_focus(&self) -> bool {
        false
    }
}
