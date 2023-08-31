mod select_list;
pub use select_list::SelectList;

use ratatui::{
    prelude::{Buffer, Rect},
    style::{Style, Styled},
    text::Text,
};

#[derive(Clone, Debug)]
pub struct WCell<'a> {
    content: Text<'a>,
    style: Style,
}

impl<'a> WCell<'a> {
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn draw(&self, buf: &mut Buffer, area: Rect) {
        buf.set_style(area, self.style);
        for (i, line) in self.content.lines.iter().enumerate() {
            if i as u16 >= area.height {
                break;
            }
            buf.set_line(area.x, area.y + i as u16, line, area.width);
        }
    }
}

impl<'a, T> From<T> for WCell<'a>
where
    T: Into<Text<'a>>,
{
    fn from(content: T) -> WCell<'a> {
        WCell {
            content: content.into(),
            style: Style::default(),
        }
    }
}

impl<'a> Styled for WCell<'a> {
    type Item = WCell<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}
#[derive(Clone, Debug)]
pub struct WRow<'a> {
    cells: Vec<WCell<'a>>,
    pub style: Style,
    height: u16,
}

impl<'a> WRow<'a> {
    pub fn new<T, C>(cells: T) -> Self
    where
        T: IntoIterator<Item = C>,
        C: Into<WCell<'a>>,
    {
        WRow {
            cells: cells.into_iter().map(|c| c.into()).collect(),
            style: Style::default(),
            height: 1,
        }
    }
    pub fn with_height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }
    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
    pub fn draw(&self, buf: &mut Buffer, areas: &[Rect]) {
        for (i, cel) in self.cells.iter().enumerate() {
            if let Some(area) = areas.get(i) {
                cel.draw(buf, *area);
            }
        }
    }

    // pub fn total_height(&self) -> u16 {
    //     self.height.saturating_add(self.bottom_margin)
    // }
}
impl<'a> Styled for WRow<'a> {
    type Item = WRow<'a>;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style(self, style: Style) -> Self::Item {
        self.style(style)
    }
}
