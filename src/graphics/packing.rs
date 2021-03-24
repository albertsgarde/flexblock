use super::pack::{get_vp_matrix, RenderState};
use super::{RenderMessage, RenderMessages};
use crate::channels::*;
use std::thread::{self, JoinHandle};

pub fn start_packing_thread(
    logic_rx: LogicToPackingReceiver,
    tx: PackingToWindowSender,
    window_rx : WindowToPackingReceiver
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

            let vp = get_vp_matrix(&data.view);

            let mut messages = RenderMessages::new();

            // What should happen:
            // 1. Clear color and depth buffer
            // 2. Supply commmon uniforms
            // 3. For every new chunk we're interested in (Or dirty chunks)
            //   3a. Fill the chunk into a vertex array or update the existing vertex array
            // 4. For every chunk already filled into a vertex array
            //   4a. Supply specific uniforms
            //   4b. Draw

            messages.add_message(RenderMessage::ChooseShader {
                shader: String::from("s1"),
            });
            messages.add_message(RenderMessage::ClearBuffers {
                color_buffer: true,
                depth_buffer: true,
            });

           // state.pack_next_chunk(data.view.location().chunk, &mut messages, &data.terrain);
            state.repack_chunk(data.view.location().chunk, &mut messages, &data.terrain);
            state.repack_chunk(data.view.location().chunk+glm::vec3(1,0,0), &mut messages, &data.terrain);
            state.repack_chunk(data.view.location().chunk+glm::vec3(-1,0,0), &mut messages, &data.terrain);
            state.repack_chunk(data.view.location().chunk+glm::vec3(0,1,0), &mut messages, &data.terrain);
            state.repack_chunk(data.view.location().chunk+glm::vec3(0,-1,0), &mut messages, &data.terrain);
            state.repack_chunk(data.view.location().chunk+glm::vec3(0,0,1), &mut messages, &data.terrain);
            state.repack_chunk(data.view.location().chunk+glm::vec3(0,0,-1), &mut messages, &data.terrain);

            state.clear_distant_chunks(data.view.location().chunk, &mut messages);

            state.render_packed_chunks(&mut messages, &vp);

            let mut message_mutex = tx.render_pack.lock().unwrap();
            // This check is done to make sure that persistent-state calls are done even if draw misses a call or two.
            // Like packing chunks into buffers
            if let Some(old) = message_mutex.take() {
                messages.merge_old(old);
            }
            *message_mutex = Some(messages);
        }
    })
}
