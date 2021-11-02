use std::{fs::File, io::{BufReader, BufWriter}, path::Path};

use game::{InputEventHistory, State, StateInputEvent};
use audio::AudioMessageIgnorer;

use serde::{Serialize, Deserialize};
use log::error;

use flate2::{bufread::DeflateDecoder, write::DeflateEncoder, Compression};

#[derive(Clone, Serialize, Deserialize)]
pub struct SacredState {
    state: State,
    input_event_history: InputEventHistory,
}

impl SacredState {
    pub fn new() -> SacredState {
        SacredState {state: State::new(), input_event_history: InputEventHistory::new() }
    }

    pub fn tick(&mut self, events: Vec<StateInputEvent>) {
        self.input_event_history.receive_tick_events(events);
        let tick_events = self.input_event_history.cur_tick_events().unwrap();
        self.state.tick(tick_events, &AudioMessageIgnorer);
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn input_event_history(&self) -> &InputEventHistory {
        &self.input_event_history
    }

    pub fn save(&self) {
        let save_path = Path::new("saves/save.flex");
        if let Err(error) = std::fs::create_dir_all(save_path.parent().unwrap()) {
            error!(
                "Save failed. Could not create directory. Error: {:?}",
                error
            );
            return;
        }
        // Write save data to file in bincode format.
    
        let file = {
            let file = match File::create(save_path) {
                Ok(file) => file,
                Err(error) => {
                    error!("Could not open save file. Error: {:?}", error);
                    return;
                }
            };
            BufWriter::new(file)
        };
        let mut encoder = DeflateEncoder::new(file, Compression::fast());
        if let Err(error) = bincode::serialize_into(&mut encoder, &self) {
            error!("Save failed with error: {:?}", *error)
        }
    }
    
    pub fn load() -> Result<SacredState, std::io::Error> {
        let save_path = Path::new("saves/save.flex");
        let file = {
            let file = match File::open(save_path) {
                Ok(file) => file,
                Err(error) => {
                    error!("Could not open save file. Error: {:?}", error);
                    return Err(error);
                }
            };
            BufReader::new(file)
        };
        let decoder = DeflateDecoder::new(file);
        let loaded_save_data: SacredState = bincode::deserialize_from(decoder).unwrap();
        Ok(loaded_save_data)
    }
}
