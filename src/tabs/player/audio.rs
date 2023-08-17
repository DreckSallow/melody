use std::{fs::File, io::BufReader, path::PathBuf};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};
use rodio::{Decoder, OutputStream, Sink};

use crate::{component::Component, event::AppEvent};

use super::state::{PlayerState, PlayerStateAction};

pub struct AudioPlayer {
    sink: Sink,
    _stream: OutputStream,
    pub is_focus: bool,
}

impl AudioPlayer {
    pub fn build() -> Result<Self> {
        let (_stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;
        sink.pause();
        Ok(Self {
            sink,
            _stream,
            is_focus: false,
        })
    }
}

impl AudioPlayer {
    pub fn on_change(&mut self, action: &PlayerStateAction, state: &PlayerState) {
        if let PlayerStateAction::SetAudio = *action {
            let song_opt = state
                .audio_selected
                .as_ref()
                .map(|name| PathBuf::from(name));
            if let Some(ref song) = song_opt {
                let file_song = BufReader::new(File::open(song).unwrap());
                let decoder = Decoder::new(file_song).expect("PROBLEMAS!");
                if !self.sink.empty() {
                    self.sink.stop();
                }
                self.sink.append(decoder);
                if !self.sink.is_paused() {
                    self.sink.play();
                }
                // {
                //     let probe = Probe::open(song).unwrap().read().unwrap();
                //     println!(
                //         "metadata: {:?}, {:?}",
                //         probe.properties(),
                //         probe.primary_tag().unwrap().title()
                //     );
                // }
            }
        }
    }
}

impl Component for AudioPlayer {
    fn render(&mut self, frame: &mut crate::component::FrameType, area: ratatui::prelude::Rect) {
        let styled = if self.is_focus {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default()
        };
        let block = Block::default()
            .title("audio")
            .borders(Borders::ALL)
            .style(styled);
        frame.render_widget(block, area)
    }
    fn on_event(&mut self, event: &AppEvent) {
        match *event {
            AppEvent::Quit => {
                self.sink.pause();
            }
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                };
                match key_event.code {
                    KeyCode::Char(' ') => {
                        if self.sink.is_paused() {
                            self.sink.play();
                        } else {
                            self.sink.pause();
                        }
                    }

                    _ => {}
                }
            }
        }
    }
    fn is_focus(&self) -> bool {
        self.is_focus
    }
}
