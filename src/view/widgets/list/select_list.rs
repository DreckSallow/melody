use ratatui::{
    prelude::*,
    style::Style,
    widgets::{Block, StatefulWidget, Widget},
};

use crate::{select, utils::Condition, view::widgets::state::SelectListState};

use super::WRow;

#[derive(Debug, Clone)]
pub struct SelectList<'a> {
    block: Option<Block<'a>>,
    style: Style,
    widths: &'a [Constraint],
    column_spacing: u16,
    highlight_style: Style,
    highlight_symbol: &'a str,
    header: Option<WRow<'a>>,
    rows: Vec<WRow<'a>>,
}

impl<'a> SelectList<'a> {
    pub fn new<R>(rows: R) -> Self
    where
        R: IntoIterator<Item = WRow<'a>>,
    {
        Self {
            block: None,
            style: Style::default(),
            widths: &[],
            column_spacing: 1,
            highlight_style: Style::default(),
            highlight_symbol: "> ",
            header: None,
            rows: rows.into_iter().collect(),
        }
    }

    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    pub fn header(mut self, header: WRow<'a>) -> Self {
        self.header = Some(header);
        self
    }

    pub fn widths(mut self, widths: &'a [Constraint]) -> Self {
        let between_0_and_100 = |&w| match w {
            Constraint::Percentage(p) => p <= 100,
            _ => true,
        };
        assert!(
            widths.iter().all(between_0_and_100),
            "Percentages should be between 0 and 100 inclusively."
        );
        self.widths = widths;
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    pub fn highlight_symbol(mut self, highlight_symbol: &'a str) -> Self {
        self.highlight_symbol = highlight_symbol;
        self
    }

    pub fn highlight_style(mut self, highlight_style: Style) -> Self {
        self.highlight_style = highlight_style;
        self
    }

    pub fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    fn get_columns_widths(&self, max_width: u16, has_selection: bool) -> Vec<u16> {
        let mut constraints = Vec::with_capacity(self.widths.len() * 2 + 1);
        if has_selection {
            //TODO: use .width()
            let highlight_symbol_width = self.highlight_symbol.chars().count() as u16;
            constraints.push(Constraint::Length(highlight_symbol_width));
        }
        for constraint in self.widths {
            constraints.push(*constraint);
            constraints.push(Constraint::Length(self.column_spacing));
        }
        if !self.widths.is_empty() {
            constraints.pop();
        }
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            // .expand_to_fill(false)
            .split(Rect {
                x: 0,
                y: 0,
                width: max_width,
                height: 1,
            });
        let chunks = if !has_selection {
            &chunks[..]
        } else {
            &chunks[1..]
        };
        chunks.iter().step_by(2).map(|c| c.width).collect()
    }
    fn range_rows(&self, index: Option<usize>, max_height: u16) -> (usize, usize) {
        // Check for the first rows
        let (mut start, mut end) = (0, 0);
        let mut body_height = 0;
        for row in &self.rows {
            if row.height() + body_height >= max_height {
                break;
            }
            end += 1;
            body_height += row.height();
        }
        if let Some(i) = index {
            if i <= end {
                // If the index is between in the previous range, return
                return (start, end);
            }
            start = end;
            let mut body_height = 0;
            for w_row in &self.rows[end..] {
                if w_row.height() + body_height >= max_height {
                    start = end;
                    if end <= i {
                        break;
                    }
                }
                body_height += w_row.height();
                end += 1;
            }
        }
        (start, end)
    }
}

impl<'a> StatefulWidget for SelectList<'a> {
    type State = SelectListState;
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
        let mut table_area = match self.block.take() {
            Some(b) => {
                let inner_area = b.inner(area);
                b.render(area, buf);
                inner_area
            }
            None => area,
        };

        let has_selection = state.index().is_some();
        let columns_widths = self.get_columns_widths(table_area.width, has_selection);
        let blank_symbol = " ".repeat(self.highlight_symbol.chars().count());

        // Draw the header
        if let Some(ref header) = self.header {
            buf.set_style(
                Rect {
                    height: table_area.height.min(header.height()),
                    ..table_area
                },
                header.style,
            );
            let mut col = table_area.left();
            if has_selection {
                col += (self.highlight_symbol.chars().count() as u16).min(table_area.width);
            }

            let max_header_height = table_area.height.min(header.height());
            let rects: Vec<Rect> = columns_widths
                .iter()
                .map(|w| {
                    let copy_col = col;
                    col += *w + self.column_spacing;
                    Rect::new(copy_col, table_area.top(), *w, max_header_height)
                })
                .collect();
            header.draw(buf, &rects);
            table_area.y += max_header_height;
        }

        // Draw the rows
        if self.rows.is_empty() {
            return;
        }

        let (mut start, end) =
            self.range_rows(state.index(), table_area.bottom() - table_area.top());

        for w_row in &self.rows[start..end] {
            let row_area = Rect {
                height: w_row.height(),
                ..table_area
            };
            buf.set_style(row_area, w_row.style);

            let is_index = state.index().map_or(false, |s| s == start);
            let table_row_start_col = if has_selection {
                // println!("select");
                let symbol = select!(is_index, self.highlight_symbol, &blank_symbol);
                buf.set_stringn(
                    table_area.left(),
                    table_area.top(),
                    symbol,
                    table_area.width as usize,
                    w_row.style,
                )
                .0 // Get the current col, added the string length
            } else {
                table_area.left()
            };

            let mut col = table_row_start_col;
            let rects: Vec<Rect> = columns_widths
                .iter()
                .map(|w| {
                    let copy_col = col;
                    col += *w + self.column_spacing;
                    Rect::new(copy_col, table_area.top(), *w, w_row.height())
                })
                .collect();
            w_row.draw(buf, &rects);

            if is_index {
                buf.set_style(row_area, self.highlight_style);
            }
            table_area.y += w_row.height();
            start += 1;
        }
    }
}
