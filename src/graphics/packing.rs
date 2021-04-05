use super::pack::{RenderState, RenderMessageValidator};
use crate::channels::*;
use std::thread::{self, JoinHandle};

pub fn start_packing_thread(
    logic_rx: LogicToPackingReceiver,
    tx: PackingToWindowSender,
    window_rx: WindowToPackingReceiver,
) -> JoinHandle<()> {
    let mut state = RenderState::new();
    // Only used by debug validation
    // Keeps all state needed for running validation.
    let mut validator = RenderMessageValidator::new();

    thread::spawn(move || {
        println!("Ready to pack graphics state!");

        for _ in logic_rx.channel_receiver.iter() {
            if let Ok(cap) = window_rx.channel_receiver.try_recv() {
                println!("Received a new graphics capabilities object!");
                state.update_capabilities(cap);
            }
            let data = logic_rx.graphics_state_model.lock().unwrap();

            let mut messages = state.create_render_messages(&data);

            /* Framebuffer test code (renders a red triangle to the texture bound to framebuffer f1)
            if messages.size() > 0 {
                messages.add_message(RenderMessage::ChooseShader{shader : String::from("s2")});
                messages.add_message(RenderMessage::ChooseFramebuffer{framebuffer : Some(String::from("f1"))});
                messages.add_message(RenderMessage::ClearBuffers{color_buffer : true, depth_buffer : true});
                messages.add_message(RenderMessage::Pack {
                    buffer : 80,
                    pack : VertexPack::new(
                        vec![
                            Vertex3D {x:-1., y:-1., z:-1., r:1., g:0., b:0., u:0., v:0.},
                            Vertex3D {x:1., y:-1., z:-1., r:0., g:1., b:0., u:0., v:0.},
                            Vertex3D {x:1., y:1., z:-1., r:0., g:0., b:1., u:0., v:0.}
                            ],
                        None
                    )
                });
                messages.add_message(RenderMessage::Draw {buffer : 80});
                messages.add_message(RenderMessage::ClearArray {buffer : 80});
            }*/

            let mut message_mutex = tx.render_pack.lock().unwrap();
            // This check is done to make sure that persistent-state calls are done even if draw misses a call or two.
            // Like packing chunks into buffers
            if let Some(old) = message_mutex.take() {
                messages.merge_old(old);
            } else {
                
            }

            // Validate render messages.
            debug_assert!(validator.validate(&mut state, &messages).unwrap() == ());

            *message_mutex = Some(messages);
        }
    })
}
