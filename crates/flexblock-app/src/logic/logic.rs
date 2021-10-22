use crate::{
    channels::*,
    logic::{controls, ExternalEventHandler, LogicEvent},
};
use game::{State, InputEventHistory};
use audio::AudioMessageSender;
use flate2::{bufread::DeflateDecoder, write::DeflateEncoder, Compression};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

#[derive(Serialize, Deserialize)]
struct SaveData {
    pub state: State,
    pub event_history: InputEventHistory,
}

impl SaveData {

    fn save(&self) {
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
    
    fn load() -> Result<SaveData, std::io::Error> {
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
        let loaded_save_data: SaveData = bincode::deserialize_from(decoder).unwrap();
        Ok(loaded_save_data)
    }
}

pub fn start_logic_thread(
    window_to_logic_receiver: WindowToLogicReceiver,
    logic_to_packing_sender: LogicToPackingSender,
    audio_message_handle: AudioMessageSender,
) -> JoinHandle<()> {
    thread::spawn(move || {
        info!("Using chunk size={}", world::chunk::CHUNK_SIZE);
        let gsm_mutex = logic_to_packing_sender.graphics_state_model;
        let gsm_channel = logic_to_packing_sender.channel_sender;

        let control_config_path = utils::ASSETS_PATH.join("config/controls.toml");
        let control_config = controls::load_control_config(&control_config_path);
        controls::save_control_config(&control_config_path, &control_config);
        let mut external_event_handler = ExternalEventHandler::new(control_config);
        let mut save_data = SaveData {
            state: State::new(),
            event_history: InputEventHistory::new(),
        };

        let mut last_tick = Instant::now();
        loop {
            // Handle external events.
            external_event_handler.handle_inputs(&window_to_logic_receiver.channel_receiver);
            // Get tick events.
            let (state_events, logic_events) = external_event_handler.tick_events();
            handle_logic_events(&logic_events, &mut save_data);

            let event_history = &mut save_data.event_history;
            let state = &mut save_data.state;

            // Add tick events to history.
            event_history.receive_tick_events(state_events);

            // Run tick.
            state.tick(
                event_history
                    .cur_tick_events()
                    .expect("This should not be possible"),
                &audio_message_handle,
            );

            // Update graphics state model.
            match gsm_mutex.try_lock() {
                Ok(mut gsm) => {
                    state.update_graphics_state_model(&mut gsm);
                    if let Err(error) = gsm_channel.send(Update) {
                        panic!("Packing thread has deallocated the channel. {}", error);
                    }
                }
                Err(std::sync::TryLockError::Poisoned(error)) => {
                    panic!("Graphics state model mutex is poisoned. {}", error)
                }
                Err(std::sync::TryLockError::WouldBlock) => {}
            }

            // Wait for next tick if necessary.
            if last_tick.elapsed().as_secs_f32() < game::SECONDS_PER_TICK {
                thread::sleep(Duration::from_secs_f32(
                    game::SECONDS_PER_TICK - last_tick.elapsed().as_secs_f32(),
                ));
            }
            last_tick = Instant::now();
        }
    })
}

fn handle_logic_events(events: &Vec<LogicEvent>, save_data: &mut SaveData) {
    for event in events.iter() {
        match event {
            LogicEvent::Save => save_data.save(),
            LogicEvent::LoadLatest => *save_data = SaveData::load().unwrap(),
        }
    }
}
