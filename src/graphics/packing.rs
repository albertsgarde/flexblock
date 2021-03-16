use super::{RenderMessage, RenderPack, UniformData, VertexPack};
use crate::channels::*;
use crate::utils::Vertex3D;
use std::thread::{self, JoinHandle};

fn create_chunk_pack() -> VertexPack {
    let vertex_pack = VertexPack::new(vec![], None);

    vertex_pack
}

pub fn start_packing_thread(
    rx: LogicToPackingReceiver,
    tx: PackingToWindowSender,
) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("Ready to pack graphics bindings!");

        for _ in rx.channel_receiver.iter() {
            let data = rx.graphics_state_model.lock().unwrap();
            //TODO: DO STUFF WITH TERRAIN

            let direction = data.player.view_direction();
            let position = data.player.location().position;
            let center = position + direction;

            let v: glm::Mat4 = glm::look_at(
                &glm::vec3(position[0], position[1], position[2]),
                &glm::vec3(center[0], center[1], center[2]),
                &glm::vec3(0., 1., 0.),
            );
            let p: glm::Mat4 = glm::perspective_fov(90. / 180. * 3.1415, 600., 400., 0.1, 100.0); //TODO: CORRECT FOV, WIDTH, AND HEIGHT

            let mvp = p * v;

            let mut messages = RenderPack::new();

            //TODO: Mock vertex packing. This shouldn't be done in such a silly way
            let vertex_pack = VertexPack::new(
                vec![
                    Vertex3D {
                        x: -0.5,
                        y: -0.5,
                        z: -40.,
                        r: 0.,
                        g: 0.,
                        b: 0.5,
                    },
                    Vertex3D {
                        x: 0.5,
                        y: -0.5,
                        z: -40.,
                        r: 0.,
                        g: 0.,
                        b: 0.5,
                    },
                    Vertex3D {
                        x: 0.5,
                        y: 0.5,
                        z: -40.,
                        r: 0.,
                        g: 0.,
                        b: 0.5,
                    },
                    Vertex3D {
                        x: 1.,
                        y: -0.5,
                        z: -0.5,
                        r: 0.,
                        g: 0.5,
                        b: 0.,
                    },
                    Vertex3D {
                        x: 1.,
                        y: 0.5,
                        z: -0.5,
                        r: 0.,
                        g: 0.6,
                        b: 0.,
                    },
                    Vertex3D {
                        x: 1.,
                        y: 0.5,
                        z: 0.5,
                        r: 0.,
                        g: 0.7,
                        b: 0.,
                    },
                    Vertex3D {
                        x: 0.5,
                        y: -0.5,
                        z: -1.,
                        r: 0.,
                        g: 0.3,
                        b: 0.,
                    },
                    Vertex3D {
                        x: 0.5,
                        y: 0.5,
                        z: -1.,
                        r: 0.,
                        g: 0.3,
                        b: 0.,
                    },
                    Vertex3D {
                        x: -0.5,
                        y: 0.5,
                        z: -1.,
                        r: 0.,
                        g: 0.3,
                        b: 0.,
                    },
                    Vertex3D {
                        x: 5.5,
                        y: -0.5,
                        z: -1.,
                        r: 0.,
                        g: 0.3,
                        b: 0.,
                    },
                    Vertex3D {
                        x: 5.5,
                        y: 0.5,
                        z: -1.,
                        r: 0.,
                        g: 0.3,
                        b: 0.,
                    },
                    Vertex3D {
                        x: -0.5,
                        y: 0.5,
                        z: -1.,
                        r: 0.,
                        g: 0.3,
                        b: 0.,
                    },
                    Vertex3D {
                        x: 5.5,
                        y: -0.5,
                        z: -6.,
                        r: 0.,
                        g: 0.3,
                        b: 0.,
                    },
                    Vertex3D {
                        x: 5.5,
                        y: 5.5,
                        z: -6.,
                        r: 0.,
                        g: 0.3,
                        b: 0.,
                    },
                    Vertex3D {
                        x: -0.5,
                        y: 0.5,
                        z: -6.,
                        r: 0.,
                        g: 0.3,
                        b: 0.,
                    },
                    Vertex3D {
                        x: 50.,
                        y: -50.,
                        z: 0.0,
                        r: 0.,
                        g: 0.,
                        b: 1.,
                    },
                    Vertex3D {
                        x: 50.,
                        y: 50.,
                        z: 0.0,
                        r: 0.,
                        g: 0.,
                        b: 1.,
                    },
                    Vertex3D {
                        x: -50.,
                        y: 50.,
                        z: 0.0,
                        r: 0.,
                        g: 0.,
                        b: 1.,
                    },
                    Vertex3D {
                        x: -50.0,
                        y: -1.,
                        z: -50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 0.,
                        y: -1.,
                        z: -50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -50.,
                        y: -1.,
                        z: 0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -50.,
                        y: -1.,
                        z: 0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 0.,
                        y: -1.,
                        z: -50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 0.,
                        y: -1.,
                        z: 0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -0.0,
                        y: -1.,
                        z: -50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 50.,
                        y: -1.,
                        z: -50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -0.,
                        y: -1.,
                        z: 0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -0.,
                        y: -1.,
                        z: 0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 50.,
                        y: -1.,
                        z: -50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 50.,
                        y: -1.,
                        z: 00.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -0.0,
                        y: -1.,
                        z: -0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 50.,
                        y: -1.,
                        z: -0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -0.,
                        y: -1.,
                        z: 50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -0.,
                        y: -1.,
                        z: 50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 50.,
                        y: -1.,
                        z: -0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 50.,
                        y: -1.,
                        z: 50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -50.0,
                        y: -1.,
                        z: -0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 0.,
                        y: -1.,
                        z: -0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -50.,
                        y: -1.,
                        z: 50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: -50.,
                        y: -1.,
                        z: 50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 0.,
                        y: -1.,
                        z: -0.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                    Vertex3D {
                        x: 0.,
                        y: -1.,
                        z: 50.0,
                        r: 0.5,
                        g: 0.5,
                        b: 0.2,
                    },
                ],
                None,
            );

            messages.add_message(RenderMessage::ChooseShader {
                shader: String::from("s1"),
            });

            messages.add_message(RenderMessage::Clear { vertex_array: 1 });

            messages.add_message(RenderMessage::Pack {
                vertex_array: 1,
                pack: vertex_pack,
            });

            let uniform_data = UniformData::new(vec![(mvp, String::from("MVP"))], vec![]);
            messages.add_message(RenderMessage::Uniforms {
                uniforms: uniform_data,
            });

            let mut message_mutex = tx.render_pack.lock().unwrap();
            *message_mutex = Some(messages);
        }
    })
}
