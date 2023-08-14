use ratatui::widgets::TableState;

#[derive(Default)]
pub struct TableController {
    state: TableState,
}

impl TableController {
    pub fn with_select(mut self, index: Option<usize>) -> Self {
        self.select(index);
        self
    }
    pub fn select(&mut self, index: Option<usize>) {
        self.state.select(index)
    }

    pub fn state(&mut self) -> &mut TableState {
        &mut self.state
    }

    pub fn next(&mut self, len: usize) {
        if len == 0 {
            return self.state.select(None);
        }

        let i = match self.state.selected() {
            Some(i) => {
                if i >= len - 1 {
                    0
                } else {
                    i + 1
                }
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
                if i == 0 {
                    len - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
