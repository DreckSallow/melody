use std::collections::HashSet;

use crate::{select, utils::Condition};

#[derive(Clone, Debug, Default)]
pub struct SelectListState {
    items_len: usize,
    index: Option<usize>,
    selecteds: HashSet<usize>,
}

impl SelectListState {
    pub fn with_len(mut self, len: usize) -> Self {
        self.items_len = len;
        self
    }

    pub fn with_selecteds<T>(mut self, usizes: T) -> Self
    where
        T: IntoIterator<Item = usize>,
    {
        self.selecteds = usizes.into_iter().collect();
        self
    }

    pub fn with_index(mut self, index: Option<usize>) -> Self {
        self.index = index;
        self
    }
    pub fn index(&self) -> Option<usize> {
        self.index
    }

    pub fn selecteds(&self) -> &HashSet<usize> {
        &self.selecteds
    }

    pub fn toggle_select(&mut self) {
        if let Some(index) = self.index {
            if self.selecteds.contains(&index) {
                self.selecteds.remove(&index);
            } else {
                self.selecteds.insert(index);
            }
        }
    }
    pub fn set_index(&mut self, index: Option<usize>) {
        self.index = index;
    }
    pub fn next(&mut self) {
        if let Some(i) = self.index {
            let index = select!(i >= self.items_len - 1, 0, i + 1);
            self.set_index(Some(index));
        } else if self.items_len > 0 {
            self.set_index(Some(0))
        }
    }
    pub fn previous(&mut self) {
        if let Some(i) = self.index {
            let index = select!(i == 0, self.items_len - 1, i - 1);
            self.set_index(Some(index));
        }
    }
}
