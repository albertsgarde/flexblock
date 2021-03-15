use crate::channels::*;
use std::thread::{self, JoinHandle};
use super::{RenderPack,RenderMessage, RenderData};
use crate::utils::{vertex::Vertex, Vertex3D};

pub fn start_packing_thread(rx: LogicToPackingReceiver, tx: PackingToWindowSender) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("Ready to pack graphics bindings!");

        for _ in rx.channel_receiver.iter() {
            println!("We've received stuff!");
            let _data = rx.graphics_state_model.lock().unwrap();
            //TODO: DO STUFF WITH DATA


            
            let mut messages = RenderPack::new();
            
            //TODO: Mock vertex packing. This shouldn't be done in such a silly way
            let vertex_pack = RenderData::new(
                vec![
                    Vertex3D {x : -0.5, y : -0.5, z : 0., r : 0., g : 0., b : 0.},
                    Vertex3D {x : 0.5, y : -0.5, z : 0., r : 0., g : 0., b : 0.},
                    Vertex3D {x : 0., y : 0.5, z : 0., r : 0., g : 0., b : 0.},
                ],
                None,
                Vertex3D::attribute_pointers()
            );
            
            messages.add_message(RenderMessage::Clear {
                vertex_array : 1
            });

            messages.add_message(RenderMessage::Pack {
                vertex_array : 1,
                pack : vertex_pack
            });

            let mut message_mutex = tx.render_pack.lock().unwrap();
            *message_mutex = Some(messages);
        }
    })
}
