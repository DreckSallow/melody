use ratatui::widgets::ListState;

use crate::{select, utils::Condition};

#[derive(Default)]
pub struct ListController {
    state: ListState,
}

impl ListController {
    pub fn with_select(mut self, index: Option<usize>) -> Self {
        self.select(index);
        self
    }
    pub fn select(&mut self, index: Option<usize>) {
        self.state.select(index)
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn state(&mut self) -> &mut ListState {
        &mut self.state
    }

    pub fn next(&mut self, len: usize) {
        if len == 0 {
            return self.state.select(None);
        }

        let i = match self.state.selected() {
            Some(i) => {
                select!(i >= len - 1, 0, i + 1)
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self, len: usize) {
        if len == 0 {
            return self.state.select(None);
        }
        let i = match self.state.selected() {
            Some(i) => {
                select!(i == 0, len - 1, i - 1)
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
