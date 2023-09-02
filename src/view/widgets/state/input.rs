#[derive(Default)]
pub struct InputState {
    text: String,
    index: usize,
}

impl InputState {
    pub fn new(text: String, index: usize) -> Self {
        Self { text, index }
    }
    pub fn text(&self) -> &str {
        &self.text
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn next_index(&mut self) {
        if self.index + 1 <= self.text.len() {
            self.index += 1;
        }
    }
    pub fn back_index(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        }
    }
    pub fn insert(&mut self, slice: &str) {
        self.text.insert_str(self.index, slice);
    }
    pub fn remove_ch(&mut self) {
        if self.index > 0 {
            self.text.remove(self.index - 1);
        }
    }
}
