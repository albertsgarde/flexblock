use wrapper::FramebufferIdentifier;

use super::pack::*;
use super::wrapper::ShaderIdentifier;
use super::*;
use super::{RenderMessage, RenderMessages};
use crate::channels::*;
use crate::graphics::wrapper::BufferTarget;
use std::thread::{self, JoinHandle};
use utils::Vertex3D;

fn get_reticle_pack() -> VertexPack {
    VertexPack::new(
        vec![
            Vertex3D {
                x: -0.02,
                y: -0.01,
                z: -1.,
                r: 1.0,
                g: 1.,
                b: 1.,
                u: 0.,
                v: 0.,
            },
            Vertex3D {
                x: 0.02,
                y: -0.01,
                z: -1.,
                r: 1.,
                g: 1.,
                b: 1.,
                u: 1.,
                v: 0.,
            },
            Vertex3D {
                x: 0.02,
                y: 0.01,
                z: -1.,
                r: 1.,
                g: 1.,
                b: 1.,
                u: 1.,
                v: 1.,
            },
            Vertex3D {
                x: -0.02,
                y: 0.01,
                z: -1.,
                r: 1.,
                g: 1.,
                b: 1.,
                u: 0.,
                v: 1.,
            },
            Vertex3D {
                x: -0.01,
                y: -0.02,
                z: -1.,
                r: 1.0,
                g: 1.,
                b: 1.,
                u: 0.,
                v: 0.,
            },
            Vertex3D {
                x: 0.01,
                y: -0.02,
                z: -1.,
                r: 1.,
                g: 1.,
                b: 1.,
                u: 1.,
                v: 0.,
            },
            Vertex3D {
                x: 0.01,
                y: 0.02,
                z: -1.,
                r: 1.,
                g: 1.,
                b: 1.,
                u: 1.,
                v: 1.,
            },
            Vertex3D {
                x: -0.01,
                y: 0.02,
                z: -1.,
                r: 1.,
                g: 1.,
                b: 1.,
                u: 0.,
                v: 1.,
            },
        ],
        Some(vec![0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7]),
    )
}

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
        for _ in logic_rx.channel_receiver.iter() {
            while let Ok(cap) = window_rx.channel_receiver.try_recv() {
                state.update_capabilities(cap);
            }
            let data = logic_rx.graphics_state_model.lock().unwrap();

            let mut messages = RenderMessages::new();

            if state.is_render_ready() {
                messages.add_message(RenderMessage::ChooseFramebuffer {
                    framebuffer: Some(FramebufferIdentifier::FirstPass),
                });

                messages.merge_current(state.create_render_messages(&data));

                {
                    let cx = state.render_capabilities().as_ref().unwrap();
                    let mut cp = ComputePipeline::new();

                    // Sobel
                    let mut uniforms = UniformData::new();
                    uniforms.texture("fpf_color".to_owned(), "from_tex");

                    cp.add_dispatch(ComputeDispatch::new(
                        ShaderIdentifier::Sobel,
                        uniforms,
                        "sobel_output",
                        (cx.screen_dimensions.0, cx.screen_dimensions.1, 1),
                    ));

                    // Artsyfartsy
                    let mut uniforms = UniformData::new();
                    uniforms.texture("sobel_output".to_owned(), "sobel_tex");
                    uniforms.texture("fpf_depth".to_owned(), "depth_tex");
                    uniforms.texture("fpf_color".to_owned(), "color_tex");

                    cp.add_dispatch(ComputeDispatch::new(
                        ShaderIdentifier::Artsyfartsy,
                        uniforms,
                        "artsy_output",
                        (cx.screen_dimensions.0, cx.screen_dimensions.1, 1),
                    ));

                    messages.merge_current(cp.get_messages());
                }

                // Framebuffer test code (Renders the texture sobel_output to screen)
                messages.add_message(RenderMessage::ChooseShader {
                    shader: ShaderIdentifier::Simple,
                });
                let mut ud = UniformData::new();
                ud.texture("artsy_output".to_owned(), "tex");
                messages.add_message(RenderMessage::Uniforms {
                    uniforms: Box::new(ud),
                });
                messages.add_message(RenderMessage::ChooseFramebuffer { framebuffer: None });
                messages.add_message(RenderMessage::ClearBuffers {
                    color_buffer: true,
                    depth_buffer: true,
                });
                messages.add_message(RenderMessage::Pack {
                    buffer: BufferTarget::NormalBuffer(80),
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
                messages.add_message(RenderMessage::Draw {
                    buffer: BufferTarget::NormalBuffer(80),
                });
                messages.add_message(RenderMessage::ClearArray {
                    buffer: BufferTarget::NormalBuffer(80),
                });
                messages.add_message(RenderMessage::ChooseShader {
                    shader: ShaderIdentifier::Color,
                });
                messages.add_message(RenderMessage::ClearBuffers {
                    color_buffer: false,
                    depth_buffer: true,
                });
                messages.add_message(RenderMessage::Pack {
                    buffer: BufferTarget::NormalBuffer(80),
                    pack: get_reticle_pack(),
                });
                messages.add_message(RenderMessage::Draw {
                    buffer: BufferTarget::NormalBuffer(80),
                });
                messages.add_message(RenderMessage::ClearArray {
                    buffer: BufferTarget::NormalBuffer(80),
                });
            }

            let mut message_mutex = tx.render_pack.lock().unwrap();
            // This check is done to make sure that persistent-state calls are done even if draw misses a call or two.
            // Like packing chunks into buffers
            if let Some(old) = message_mutex.take() {
                messages.merge_old(old);
            } else {
            }

            // Validate render messages.
            debug_assert!(validator.validate(&state, &messages).is_ok());

            *message_mutex = Some(messages);
        }
    })
}
