use super::pack::{RenderState};
use crate::channels::*;
use std::thread::{self, JoinHandle};

pub fn start_packing_thread(
    logic_rx: LogicToPackingReceiver,
    tx: PackingToWindowSender,
    window_rx: WindowToPackingReceiver,
) -> JoinHandle<()> {
    let mut state = RenderState::new();

    thread::spawn(move || {
        println!("Ready to pack graphics state!");

        for _ in logic_rx.channel_receiver.iter() {
            if let Ok(cap) = window_rx.channel_receiver.try_recv() {
                println!("Received a new graphics capabilities object!");
                state.update_capabilities(cap);
            }
            let data = logic_rx.graphics_state_model.lock().unwrap();


            let mut messages = state.create_render_messages(&data);

            let mut message_mutex = tx.render_pack.lock().unwrap();
            // This check is done to make sure that persistent-state calls are done even if draw misses a call or two.
            // Like packing chunks into buffers
            if let Some(old) = message_mutex.take() {
                messages.merge_old(old);
            } else {
                
            }

            // Validate render messages.
            debug_assert!(state.validate(&messages).unwrap() == ());

            *message_mutex = Some(messages);
        }
    })
}
