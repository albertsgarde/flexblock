#![allow(dead_code)]
#![warn(missing_docs)]
//! Flexiblock aims to be a messy, overengineered, feature-creeped, and generally super cool Minecraft clone.
mod channels;
mod game;
mod graphics;
mod utils;

use crate::game::GraphicsStateModel;
use crate::graphics::Bindings;
use std::sync::{Arc, Mutex, mpsc};

fn main() {
    // Create game input event channel.
    let (game_event_sender, game_event_receiver) = mpsc::channel();
    let window_to_logic_sender = channels::WindowToLogicSender {
        channel_sender: game_event_sender,
    };
    let window_to_logic_receiver = channels::WindowToLogicReceiver {
        channel_receiver: game_event_receiver,
    };

    // Create state model for the packer and a channel to tell packer of updates.
    let graphics_state_model = Arc::new(Mutex::new(GraphicsStateModel {}));
    let (graphics_model_update_sender, graphics_model_update_receiver) =
        mpsc::channel();
    let logic_to_packing_sender = channels::LogicToPackingSender {
        channel_sender: graphics_model_update_sender,
        graphics_state_model: graphics_state_model.clone(),
    };
    let logic_to_packing_receiver = channels::LogicToPackingReceiver {
        channel_receiver: graphics_model_update_receiver,
        graphics_state_model,
    };

    // Create bindings object to share between packer and window.
    let bindings = Arc::new(Mutex::new(Bindings {}));
    let packing_to_window_sender = channels::PackingToWindowSender {
        bindings: bindings.clone(),
    };
    let packing_to_window_receiver = channels::PackingToWindowReceiver { bindings };

    // Start threads.
    let logic_thread = game::start_logic_thread(window_to_logic_receiver, logic_to_packing_sender);
    let packing_thread =
        graphics::start_packing_thread(logic_to_packing_receiver, packing_to_window_sender);
    let window_thread =
        graphics::start_window_thread(packing_to_window_receiver, window_to_logic_sender);
    window_thread.join().expect("Panic in window thread");
    packing_thread.join().expect("Panic in packing thread");
    logic_thread.join().expect("Panic in logic thread");
}
