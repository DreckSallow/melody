use std::{fs::File, io::BufReader, path::PathBuf, thread, time::Duration};

use anyhow::Result;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};
use rodio::{Decoder, OutputStream, Sink};

use crate::{component::Component, event::AppEvent};

use super::state::{PlayerState, PlayerStateAction};

// #[derive(Debug)]
// struct Song {
//     // name: String,
//     path: String,
// }

pub struct AudioPlayer {
    song: Option<PathBuf>,
    sink: Sink,
    _stream: OutputStream,
    pub is_focus: bool,
}

impl AudioPlayer {
    pub fn build() -> Result<Self> {
        let (_stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;
        Ok(Self {
            sink,
            _stream,
            is_focus: false,
            song: None,
        })
    }
}

impl AudioPlayer {
    pub fn on_change(&mut self, action: &PlayerStateAction, state: &PlayerState) {
        if let PlayerStateAction::SetAudio = *action {
            self.song = state
                .audio_selected
                .as_ref()
                .and_then(|name| Some(PathBuf::from(name)));
            if let Some(ref song) = self.song {
                self.sink.stop();
                let file_song = BufReader::new(File::open(song).unwrap());
                self.sink
                    .append(Decoder::new(file_song).expect("PROBLEMAS!"));
                thread::sleep(Duration::from_secs(2));
                self.sink.play();
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
            AppEvent::Key(_key_event) => {}
        }
    }
    fn is_focus(&self) -> bool {
        self.is_focus
    }
}
