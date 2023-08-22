use std::{
    fs::File,
    io::BufReader,
    rc::Rc,
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

use crate::{
    app::AppState,
    component::{Component, FrameType},
    event::AppEvent,
    loaders::player::PlaylistSong,
    tabs::log::LogsState,
};

use super::state::PlayerState;

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
    pub fn set_song(&mut self, song_opt: Option<PlaylistSong>) {
        if let Some(song) = &song_opt {
            let file_song = BufReader::new(File::open(&song.path).expect("ERROR"));
            let decoder = Decoder::new(file_song).expect("Decoder not read");
            self.append(decoder);
        }
        self.song = song_opt;
    }
    pub fn append(&mut self, decoder: Decoder<BufReader<File>>) {
        if !self.sink.empty() {
            self.sink.stop();
        };
        self.sink.append(decoder);
        self.progress = Progress::default();

        if let AudioStatus::Play = self.status {
            self.play();
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
    logger: LogsState,
    indices: Option<(usize, usize)>,
    pub is_focus: bool,
}

impl AudioPlayer {
    pub fn build(
        app_state: &AppState,
        song: Option<PlaylistSong>,
        indices: Option<(usize, usize)>,
    ) -> Result<Self> {
        let mut handler = AudioHandler::try_default()?;
        handler.set_song(song);

        Ok(Self {
            handler,
            is_focus: false,
            indices,
            logger: Rc::clone(&app_state.log),
        })
    }
}

impl Component for AudioPlayer {
    type State = PlayerState;
    fn render(&mut self, frame: &mut FrameType, area: ratatui::prelude::Rect, state: &Self::State) {
        if self.indices != state.indices() {
            self.indices = state.indices();
            self.handler.pause();
            self.handler.set_song(state.selected_audio().cloned());
        }

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
                    .label(format!("{} / {}", seconds, song.duration.as_secs()));
                frame.render_widget(gauge, chunks[1]);
            }
            None => {}
        }
    }
    fn on_event(&mut self, event: &AppEvent, state: &mut Self::State) {
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
