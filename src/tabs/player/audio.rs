use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    prelude::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{block::Title, Block, Borders, Gauge, Paragraph},
};
use rodio::{Decoder, OutputStream, Sink};

use crate::{component::Component, event::AppEvent, loaders::player::PlaylistSong};

use super::state::{PlayerState, PlayerStateAction};

#[derive(Debug)]
struct Progress {
    duration: Duration,
    timer: Option<Instant>,
}

impl Default for Progress {
    fn default() -> Self {
        Self::new(Duration::default(), None)
    }
}

impl Progress {
    pub fn new(duration: Duration, timer: Option<Instant>) -> Self {
        Self { duration, timer }
    }
    pub fn pause(&mut self) {
        if let Some(timer) = self.timer.as_ref() {
            self.duration += timer.elapsed();
            self.timer = None;
        }
    }
    pub fn start(&mut self) {
        self.timer = Some(Instant::now())
    }
    pub fn seconds(&self) -> u64 {
        match self.timer {
            Some(ref timer) => (self.duration + timer.elapsed()).as_secs(),
            None => self.duration.as_secs(),
        }
    }
    pub fn percentage(&self, other: Duration) -> u8 {
        let percentage = (self.seconds() * 100) / other.as_secs();
        if percentage >= 100 {
            100
        } else {
            percentage as u8
        }
    }
}

enum AudioStatus {
    Pause,
    Play,
}

struct AudioHandler {
    song: Option<PlaylistSong>,
    sink: Sink,
    _stream: OutputStream,
    status: AudioStatus,
    progress: Progress,
}

impl AudioHandler {
    pub fn try_default() -> Result<Self> {
        let (_stream, handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&handle)?;
        sink.pause();

        Ok(Self {
            sink,
            _stream,
            song: None,
            status: AudioStatus::Pause,
            progress: Progress::default(),
        })
    }
    pub fn set_song(&mut self, song: Option<PlaylistSong>) {
        self.song = song;
    }
    pub fn pause(&mut self) {
        self.sink.pause();
        self.progress.pause();
        self.status = AudioStatus::Pause;
    }
    pub fn play(&mut self) {
        self.sink.play();
        self.progress.start();
        self.status = AudioStatus::Play;
    }
    pub fn finish(&mut self) {
        self.sink.pause();
        self.sink.stop();
    }
    pub fn percentage_info(&mut self, other: Duration) -> (u64, u8) {
        let info = (self.progress.seconds(), self.progress.percentage(other));
        if info.1 >= 100 {
            self.pause();
        }

        info
    }
    pub fn append(&mut self, decoder: Decoder<BufReader<File>>) {
        self.sink.append(decoder);
        self.progress = Progress::default();
        match self.status {
            AudioStatus::Play => {
                self.play();
            }
            _ => {}
        }
    }
    pub fn toggle_action(&mut self) {
        match self.status {
            AudioStatus::Pause => self.play(),
            AudioStatus::Play => self.pause(),
        }
    }
}

pub struct AudioPlayer {
    handler: AudioHandler,
    pub is_focus: bool,
}

impl AudioPlayer {
    pub fn build() -> Result<Self> {
        Ok(Self {
            handler: AudioHandler::try_default()?,
            is_focus: false,
        })
    }
}

impl AudioPlayer {
    pub fn on_change(&mut self, action: &PlayerStateAction, state: &PlayerState) {
        if let PlayerStateAction::SetAudio = *action {
            if state.playlist_selected.is_none() && state.audio_selected.is_none() {
                return;
            }
            let (playlist_i, audio_i) = (
                state.playlist_selected.unwrap(),
                state.audio_selected.unwrap(),
            );
            if let Some(song) = state
                .library
                .playlists
                .get(playlist_i)
                .and_then(|p| p.songs.get(audio_i))
            {
                self.handler.set_song(Some(song.clone()));
                let file_song = BufReader::new(File::open(&song.path).unwrap());
                let decoder = Decoder::new(file_song).expect("PROBLEMAS!");
                self.handler.append(decoder);
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
        let block_title = if self.handler.song.is_some() {
            "Now Playing"
        } else {
            "Not song"
        };

        let block = Block::default()
            .title(Title::from(block_title).alignment(Alignment::Center))
            .borders(Borders::ALL)
            .style(styled);
        frame.render_widget(block, area);

        match self.handler.song.clone() {
            Some(song) => {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .vertical_margin(2)
                    .horizontal_margin(1)
                    .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                    .split(area);
                let header_block =
                    Paragraph::new(song.file_name.clone().unwrap_or("No name".into()))
                        .style(Style::default())
                        .alignment(Alignment::Center);
                frame.render_widget(header_block, chunks[0]);

                let (seconds, percent) = self.handler.percentage_info(song.duration);

                let gauge = Gauge::default()
                    .gauge_style(Style::default().fg(Color::Red))
                    .percent(percent as u16)
                    .label(format!(
                        "{} / {}",
                        seconds,
                        song.duration.as_secs().to_string()
                    ));
                frame.render_widget(gauge, chunks[1]);
            }
            None => {}
        }
    }
    fn on_event(&mut self, event: &AppEvent) {
        match *event {
            AppEvent::Quit => {
                self.handler.finish();
            }
            AppEvent::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press {
                    return;
                };
                match key_event.code {
                    KeyCode::Char(' ') => self.handler.toggle_action(),

                    _ => {}
                }
            }
        }
    }
    fn is_focus(&self) -> bool {
        self.is_focus
    }
}
