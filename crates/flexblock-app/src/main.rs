#![allow(dead_code)]
#![warn(missing_docs)]
//! Flexiblock aims to be a messy, overengineered, feature-creeped, and generally super cool Minecraft clone.
mod channels;
mod logic;
mod graphics;
mod logging;

use std::sync::{mpsc, Arc, Mutex};

extern crate nalgebra_glm as glm;

use game::GraphicsStateModel;
use crate::graphics::RenderMessages;

fn main() {
    logging::log_init();

    // Create game input event channel.
    let (game_event_sender, game_event_receiver) = mpsc::channel();
    let window_to_logic_sender = channels::WindowToLogicSender {
        channel_sender: game_event_sender,
    };
    let window_to_logic_receiver = channels::WindowToLogicReceiver {
        channel_receiver: game_event_receiver,
    };

    // Create state model for the packer and a channel to tell packer of updates.
    let graphics_state_model = Arc::new(Mutex::new(GraphicsStateModel::new()));
    let (graphics_model_update_sender, graphics_model_update_receiver) = mpsc::channel();
    let logic_to_packing_sender = channels::LogicToPackingSender {
        channel_sender: graphics_model_update_sender,
        graphics_state_model: graphics_state_model.clone(),
    };
    let logic_to_packing_receiver = channels::LogicToPackingReceiver {
        channel_receiver: graphics_model_update_receiver,
        graphics_state_model,
    };

    let (graphics_capabilities_sender, graphics_capabilities_receiver) = mpsc::channel();
    let window_to_packing_sender = channels::WindowToPackingSender {
        channel_sender: graphics_capabilities_sender,
    };
    let window_to_packing_receiver = channels::WindowToPackingReceiver {
        channel_receiver: graphics_capabilities_receiver,
    };

    // Create bindings object to share between packer and window.
    let render_pack: Arc<Mutex<Option<RenderMessages>>> = Arc::new(Mutex::new(None));
    let packing_to_window_sender = channels::PackingToWindowSender {
        render_pack: render_pack.clone(),
    };
    let packing_to_window_receiver = channels::PackingToWindowReceiver { render_pack };

    // Create audio thread.
    let audio_handle = audio::setup_audio(game::TPS);
    let logic_audio_message_handle = audio_handle.audio_message_handle();

    // Start threads.
    let logic_thread = logic::start_logic_thread(
        window_to_logic_receiver,
        logic_to_packing_sender,
        logic_audio_message_handle,
    );
    let packing_thread = graphics::start_packing_thread(
        logic_to_packing_receiver,
        packing_to_window_sender,
        window_to_packing_receiver,
    );

    // We unfortunately cannot catch panics from the window thread :(
    graphics::start_window(
        packing_to_window_receiver,
        window_to_logic_sender,
        window_to_packing_sender,
    );
    audio_handle.stop_audio();

    packing_thread.join().expect("Panic in packing thread");
    logic_thread.join().expect("Panic in logic thread");
}
