use wrapper::FramebufferIdentifier;

use super::pack::*;
use super::wrapper::ShaderIdentifier;
use super::*;
use super::{RenderMessage, RenderMessages};
use crate::channels::*;
use crate::utils::Vertex3D;
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
            while let Ok(cap) = window_rx.channel_receiver.try_recv() {
                state.update_capabilities(cap);
            }
            let data = logic_rx.graphics_state_model.lock().unwrap();

            let mut messages = RenderMessages::new();

            if state.is_render_ready() {
                messages.add_message(RenderMessage::ChooseFramebuffer {
                    framebuffer: Some(FramebufferIdentifier::FirstPassFramebuffer),
                });

                messages.merge_current(state.create_render_messages(&data));

                {
                    let cx = state.render_capabilities().as_ref().unwrap();
                    let mut cp = ComputePipeline::new();
                    cp.add_dispatch(ComputeDispatch::new(
                        ShaderIdentifier::SobelShader,
                        UniformData::new(
                            vec![],
                            vec![],
                            vec![("fpfcolor".to_owned(), "fromTex".to_owned())],
                        ),
                        "sobel_output".to_owned(),
                        (cx.screen_dimensions.0, cx.screen_dimensions.1, 1),
                    ));

                    messages.merge_current(cp.get_messages());
                }

                // Framebuffer test code (Renders the texture sobel_output to screen)
                messages.add_message(RenderMessage::ChooseShader {
                    shader: ShaderIdentifier::SimpleShader,
                });
                messages.add_message(RenderMessage::Uniforms {
                    uniforms: UniformData::new(
                        vec![],
                        vec![],
                        vec![("sobel_output".to_owned(), "tex".to_owned())],
                    ),
                });
                messages.add_message(RenderMessage::ChooseFramebuffer { framebuffer: None });
                messages.add_message(RenderMessage::ClearBuffers {
                    color_buffer: true,
                    depth_buffer: true,
                });
                messages.add_message(RenderMessage::Pack {
                    buffer: 80,
                    pack: VertexPack::new(
                        vec![
                            Vertex3D {
                                x: -1.,
                                y: -1.,
                                z: -1.,
                                r: 1.,
                                g: 1.,
                                b: 1.,
                                u: 0.,
                                v: 0.,
                            },
                            Vertex3D {
                                x: 1.,
                                y: -1.,
                                z: -1.,
                                r: 1.,
                                g: 1.,
                                b: 1.,
                                u: 1.,
                                v: 0.,
                            },
                            Vertex3D {
                                x: 1.,
                                y: 1.,
                                z: -1.,
                                r: 1.,
                                g: 1.,
                                b: 1.,
                                u: 1.,
                                v: 1.,
                            },
                            Vertex3D {
                                x: -1.,
                                y: 1.,
                                z: -1.,
                                r: 1.,
                                g: 1.,
                                b: 1.,
                                u: 0.,
                                v: 1.,
                            },
                        ],
                        Some(vec![0, 1, 2, 0, 2, 3]),
                    ),
                });
                messages.add_message(RenderMessage::Draw { buffer: 80 });
                messages.add_message(RenderMessage::ClearArray { buffer: 80 });
            }

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
