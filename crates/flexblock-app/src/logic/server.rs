use std::sync::mpsc::{self, TryRecvError};

use game::{InputEventHistory};
use multiplayer::ServerNetworking;

use crate::logic::sacred_state::SacredState;

pub struct ServerHandle {
    stop_sender: mpsc::Sender<()>,
}

impl ServerHandle {
    pub fn start(ip: &str) -> Self {
        let (stop_sender, stop_receiver) = mpsc::channel();
        let mut server = ServerNetworking::start(ip).unwrap();
        let mut state = SacredState::new();
        std::thread::spawn(move || {
            loop {
                let tick_events = server.new_events();
                
                server.send_events(tick_events.to_vec());
                state.tick(tick_events);
        
                if let Err(error) = stop_receiver.try_recv() {
                    if let TryRecvError::Disconnected = error {
                        panic!("Server stop channel disconnected unexpectedly.")
                    }
                } else {
                    server.join_shut_down();
                    break;
                }
            }
        });
        ServerHandle { stop_sender }
    }

    pub fn stop(self) {
        self.stop_sender.send(()).expect("Server stop channel disconnected unexpectedly.");
    } 
}
