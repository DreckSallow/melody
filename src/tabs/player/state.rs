use anyhow::Result;
use rodio::{Decoder, OutputStream, Sink};

use crate::{
    handlers::music::{PlaylistInfo, PlaylistSong},
    select,
    tabs::log::LogMessage,
    utils::{self, Condition},
    view::controllers::{list::ListController, table::TableController},
};

pub struct PlayerState {
    pub(crate) playlists: Vec<PlaylistInfo>,
    pub(crate) list_playlists: ListController,
    pub(crate) table_songs: TableController,
    pub(crate) audio_handler: AudioHandler,
    pub focus_i: u8,
    logger: Rc<RefCell<Vec<LogMessage>>>,
}

impl PlayerState {
    pub fn create(
        playlists: Vec<PlaylistInfo>,
        logger: &Rc<RefCell<Vec<LogMessage>>>,
    ) -> Result<Self> {
        let (list_i, table_i) = match playlists.get(0) {
            Some(play) => {
                let table_i = select!(play.songs.is_empty(), None, Some(0));
                (Some(0), table_i)
            }
            None => (None, None),
        };
        let mut audio_handler = AudioHandler::try_default()?;
        if let Err(e) =
            audio_handler.set_song(playlists.get(0).and_then(|p| p.songs.get(0).cloned()))
        {
            logger.borrow_mut().push(LogMessage::error(e.to_string()))
        }

        Ok(Self {
            list_playlists: ListController::default().with_select(list_i),
            table_songs: TableController::default().with_select(table_i),
            audio_handler,
            playlists,
            focus_i: 0,
            logger: Rc::clone(logger),
        })
    }
    pub fn update_songs(&mut self) {
        if let Some(play) = self.current_playlist() {
            self.table_songs = TableController::default().with_select(select!(
                play.songs.is_empty(),
                None,
                Some(0)
            ));
            self.append_song();
        }
    }
    pub fn current_playlist(&self) -> Option<&PlaylistInfo> {
        self.list_playlists
            .selected()
            .and_then(|i| self.playlists.get(i))
    }
    pub fn append_song(&mut self) {
        if let Some(play) = self.current_playlist() {
            let song_opt = self.table_songs.selected().and_then(|i| play.songs.get(i));
            if let Err(e) = self.audio_handler.set_song(song_opt.cloned()) {
                self.logger
                    .borrow_mut()
                    .push(LogMessage::error(e.to_string()))
            }
        }
    }
}

use std::{
    cell::RefCell,
    fs::File,
    io::BufReader,
    rc::Rc,
    time::{Duration, Instant},
};

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
    pub fn total_duration(&self) -> Duration {
        self.timer
            .and_then(|t| self.duration.checked_add(t.elapsed()))
            .unwrap_or(self.duration)
    }
    pub fn seconds(&self) -> u64 {
        self.total_duration().as_secs()
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
        sink.set_volume(1.0);

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
    pub fn time_format(&self) -> String {
        utils::format_time(self.progress.seconds())
    }
    pub fn volume(&self) -> (f32, u8) {
        let v = self.sink.volume();
        (v, (v * 100.0) as u8)
    }
    pub fn toggle_volume(&self) {
        let v = self.sink.volume();
        if v >= 1.0 {
            self.sink.set_volume(0.0);
        } else if v <= -0.0 {
            self.sink.set_volume(1.0);
        }
    }
    pub fn up_volume(&self) {
        let v = self.sink.volume();
        if v + 0.1 <= 1.0 {
            self.sink.set_volume(v + 0.1);
        }
    }
    pub fn down_volume(&self) {
        let v = self.sink.volume();
        if v - 0.1 >= -0.1 {
            self.sink.set_volume(v - 0.1);
        }
    }
    pub fn percentage(&mut self, other: Duration) -> u8 {
        let percentage = self.progress.percentage(other);
        if percentage >= 100 {
            self.pause();
        }

        percentage
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
