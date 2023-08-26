use ratatui::{
    style::{Color, Style},
    widgets::{block::Title, Block, Borders},
};

pub fn ui_block<'a, T>(title: T, color: Color) -> Block<'a>
where
    T: Into<Title<'a>>,
{
    Block::default()
        .title(title.into())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(color))
}
