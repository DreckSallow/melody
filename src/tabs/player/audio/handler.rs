use std::{
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};

use crate::{loaders::player::PlaylistSong, select, utils::Condition};

#[derive(Debug)]
pub struct Progress {
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
        select!(percentage >= 100, 100, percentage as u8)
    }
}

pub enum AudioStatus {
    Pause,
    Play,
}

pub struct AudioHandler {
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
    pub fn is_end_song(&self) -> bool {
        if let Some(ref song) = self.song {
            let p = self.progress.percentage(song.duration);
            p >= 100
        } else {
            true
        }
    }

    pub fn set_song(&mut self, song_opt: Option<PlaylistSong>) -> Result<()> {
        if let Some(song) = &song_opt {
            let file_song = BufReader::new(File::open(&song.path)?);
            let decoder = Decoder::new(file_song)?;
            self.append(decoder);
        }
        self.song = song_opt;
        Ok(())
    }
    pub fn song(&self) -> Option<&PlaylistSong> {
        self.song.as_ref()
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
