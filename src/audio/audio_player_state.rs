use crate::game::{world::Location, View};

pub struct AudioPlayerState {
    view: View,
}

impl AudioPlayerState {
    pub fn new(view: View) -> Self {
        AudioPlayerState { view }
    }

    pub fn location(&self) -> Location {
        self.view.location()
    }

    pub fn view(&self) -> &View {
        &self.view
    }
}

impl Default for AudioPlayerState {
    fn default() -> Self {
        AudioPlayerState {
            view: View::default(),
        }
    }
}
