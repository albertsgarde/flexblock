use crate::{channels::*, logic::{ExternalEventHandler, LatencyState, LogicEvent, SacredState, client::ClientHandle, controls, server::ServerHandle}};
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

use super::sacred_state;

pub fn start_logic_thread(
    window_to_logic_receiver: WindowToLogicReceiver,
    logic_to_packing_sender: LogicToPackingSender,
    audio_message_handle: AudioMessageSender,
) -> JoinHandle<()> {
    thread::spawn(move || {
        info!("Starting server on ip 'localhost:15926'...");
        let server_handle = ServerHandle::start("localhost:15926");
        info!("Server started!");

        info!("Using chunk size={}", world::chunk::CHUNK_SIZE);
        let gsm_mutex = logic_to_packing_sender.graphics_state_model;
        let gsm_channel = logic_to_packing_sender.channel_sender;

        let control_config_path = utils::ASSETS_PATH.join("../config/controls.toml");
        let control_config = controls::load_control_config(&control_config_path);
        controls::save_control_config(&control_config_path, &control_config);

        let mut external_event_handler = ExternalEventHandler::new(control_config);

        info!("Connecting to local server on ip 'localhost:15926'...");
        let client = ClientHandle::start("localhost:15926");
        info!("Successfully connected to server!");
        
        let mut sacred_state = client.state().expect("Client should be initialized with Some state.");
        let mut latency_state = LatencyState::from_sacred_state(&sacred_state);

        let mut last_tick = Instant::now();
        loop {
            // Handle external events.
            external_event_handler.handle_inputs(&window_to_logic_receiver.channel_receiver);
            // Get tick events.
            let (state_events, logic_events) = external_event_handler.tick_events();
            handle_logic_events(&logic_events, &mut sacred_state);

            latency_state.tick(state_events, &audio_message_handle);

            // Update graphics state model.
            match gsm_mutex.try_lock() {
                Ok(mut gsm) => {
                    latency_state.update_graphics_state_model(&mut gsm);
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

fn handle_logic_events(events: &Vec<LogicEvent>, sacred_state: &mut SacredState) {
    for event in events.iter() {
        match event {
            LogicEvent::Save => sacred_state.save(),
            LogicEvent::LoadLatest => *sacred_state = SacredState::load().unwrap(),
        }
    }
}
