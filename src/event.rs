use crossterm::event::KeyEvent;

#[derive(Debug)]
pub enum AppEvent {
    Quit,
    Key(KeyEvent),
}
