use crate::{
    audio::AudioMessageHandle,
    channels::*,
    game::{state::State, ExternalEventHandler, InputEventHistory},
};
use log::info;
use std::{
    thread::{self, JoinHandle},
    time::{Duration, Instant},
};

pub const TPS: u32 = 24;
pub const SECONDS_PER_TICK: f32 = 1. / (TPS as f32);

pub fn start_logic_thread(
    window_to_logic_receiver: WindowToLogicReceiver,
    logic_to_packing_sender: LogicToPackingSender,
    audio_message_handle: AudioMessageHandle,
) -> JoinHandle<()> {
    thread::spawn(move || {
        info!("Using chunk size={}", crate::game::world::chunk::CHUNK_SIZE);
        let gsm_mutex = logic_to_packing_sender.graphics_state_model;
        let gsm_channel = logic_to_packing_sender.channel_sender;

        let mut external_event_handler = ExternalEventHandler::new();
        let mut event_history = InputEventHistory::new();
        let mut state = State::new();

        let mut last_tick = Instant::now();
        loop {
            // Handle external events.
            external_event_handler.handle_inputs(&window_to_logic_receiver.channel_receiver);
            // Add tick events to history.
            event_history.receive_tick_events(external_event_handler.tick_events());

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
            if last_tick.elapsed().as_secs_f32() < SECONDS_PER_TICK {
                thread::sleep(Duration::from_secs_f32(
                    SECONDS_PER_TICK - last_tick.elapsed().as_secs_f32(),
                ));
            }
            last_tick = Instant::now();
        }
    })
}
