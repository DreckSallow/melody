use std::{cell::RefCell, rc::Rc};

use crate::loaders::player::PlaylistsData;

#[derive(Debug)]
pub struct PlayerState {
    pub library: PlaylistsData,
    pub playlist_selected: Option<usize>,
    pub audio_selected: Option<usize>,
}

impl PlayerState {
    pub fn create(playlist_data: PlaylistsData) -> Self {
        Self {
            library: playlist_data,
            playlist_selected: None,
            audio_selected: None,
        }
    }
}

pub struct PlayerStateReactive {
    state: Rc<RefCell<PlayerState>>,
    observers: Vec<Box<dyn Fn(&PlayerStateAction, &PlayerState)>>,
}

impl From<&Rc<RefCell<PlayerState>>> for PlayerStateReactive {
    fn from(value: &Rc<RefCell<PlayerState>>) -> Self {
        PlayerStateReactive {
            state: Rc::clone(value),
            observers: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum PlayerStateAction {
    SetPlaylist,
    SetAudio,
}

impl PlayerStateReactive {
    pub fn dispatch<F>(&mut self, action: PlayerStateAction, cb: F)
    where
        F: FnOnce(&mut PlayerState),
    {
        {
            let mut state = self.state.borrow_mut();
            cb(&mut state);
        }
        let ref_state = &self.state.borrow();

        self.observers
            .iter_mut()
            .for_each(|f| f(&action, ref_state))
    }
    pub fn subscribe<F>(&mut self, cb: F)
    where
        F: Fn(&PlayerStateAction, &PlayerState) + 'static,
    {
        self.observers.push(Box::new(cb))
    }
}
