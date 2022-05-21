use crate::{
    channels::*,
    logic::{
        controls, ExternalEventHandler,
        LogicEvent,
    },
    
};
use multiplayer::{ClientHandle, ServerHandle, LatencyState, SacredState};
use audio::AudioMessageSender;

use game::{InputEventHistory, State, TPS, SECONDS_PER_TICK};
use log::{error, info};
use serde::{Deserialize, Serialize};
use utils::Tick;
use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub fn start_server(ip: String) -> ServerHandle {
    info!("Starting server on ip '{}'...", ip);
    let server_handle = ServerHandle::start(&ip);
    info!("Server started!");
    server_handle
}

pub fn start_logic_thread(
    window_to_logic_receiver: WindowToLogicReceiver,
    logic_to_packing_sender: LogicToPackingSender,
    audio_message_handle: AudioMessageSender,
    ip: String,
) -> JoinHandle<()> {
    thread::Builder::new().name("logic".to_owned()).spawn(move || {

        info!("Using chunk size={}", world::chunk::CHUNK_SIZE);
        let gsm_mutex = logic_to_packing_sender.graphics_state_model;
        let gsm_channel = logic_to_packing_sender.channel_sender;

        let control_config_path = utils::ASSETS_PATH.join("../config/controls.toml");
        let control_config = controls::load_control_config(&control_config_path);
        controls::save_control_config(&control_config_path, &control_config);

        let mut external_event_handler = ExternalEventHandler::new(control_config);

        info!("Connecting to server on ip '{}'...", ip);
        let client = ClientHandle::start(&ip);
        info!("Successfully connected to server!");

        let mut sacred_state = client
            .state()
            .expect("Client should be initialized with Some state.");
        let mut latency_state = LatencyState::from_sacred_state(&sacred_state);

        let mut tick = Tick::start(game::TPS);
        loop {
            // Handle external events.
            external_event_handler.handle_inputs(&window_to_logic_receiver.channel_receiver);
            // Get tick events.
            let (state_events, logic_events) = external_event_handler.tick_events();
            handle_logic_events(&logic_events, &mut sacred_state);

            client.send_events(state_events.clone());

            if let Some(state) = client.state() {
                sacred_state = state;
                latency_state.update_state(&sacred_state);
            }

            // Update latency state with newest events.
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
            tick.sync_next_tick();
        }
    }).unwrap()
}

fn handle_logic_events(events: &[LogicEvent], sacred_state: &mut SacredState) {
    for event in events.iter() {
        match event {
            LogicEvent::Save => sacred_state.save(),
            LogicEvent::LoadLatest => *sacred_state = SacredState::load().unwrap(),
        }
    }
}
