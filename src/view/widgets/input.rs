use ratatui::{
    prelude::Rect,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};

use super::state::input::InputState;

#[derive(Debug, Default)]
pub struct Input<'a> {
    block: Option<Block<'a>>,
    style: Style,
    show_cursor: bool,
    style_cursor: Style,
}

impl<'a> Input<'a> {
    pub fn with_style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
    pub fn cursor_visibility(mut self, b: bool) -> Self {
        self.show_cursor = b;
        self
    }
    pub fn cursor_style(mut self, s: Style) -> Self {
        self.style_cursor = s;
        self
    }
}

impl<'a> StatefulWidget for Input<'a> {
    type State = InputState;
    fn render(
        mut self,
        area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
        state: &mut Self::State,
    ) {
        if area.area() == 0 {
            return;
        }
        buf.set_style(area, self.style);
        let input_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };
        buf.set_stringn(
            input_area.x,
            input_area.y,
            state.text(),
            input_area.width as usize,
            Style::default(),
        );
        if self.show_cursor {
            let cursor = Rect::new(input_area.x + state.index() as u16, input_area.y, 1, 1);
            buf.set_style(cursor, self.style_cursor);
        }
    }
}
