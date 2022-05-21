use std::{
    sync::{mpsc::{self, TryRecvError}, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::server_networking::ServerNetworking;

use crate::sacred_state::SacredState;

pub struct ServerHandle {
    stop_sender: mpsc::Sender<()>,
}

impl ServerHandle {
    pub fn start(ip: &str) -> Self {
        let (stop_sender, stop_receiver) = mpsc::channel();
        let state = Arc::new(Mutex::new(SacredState::new()));

        let mut server = ServerNetworking::start(ip, Arc::clone(&state)).unwrap();
        std::thread::Builder::new()
            .name("server_handle".to_owned())
            .spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let tick_events = server.new_events();

                    server.send_events(tick_events.to_vec());
                    state.lock().unwrap().tick(tick_events);

                    if let Err(error) = stop_receiver.try_recv() {
                        if let TryRecvError::Disconnected = error {
                            panic!("Server stop channel disconnected unexpectedly.")
                        }
                    } else {
                        server.join_shut_down();
                        break;
                    }

                    // Wait for next tick if necessary.
                    if last_tick.elapsed().as_secs_f32() < game::SECONDS_PER_TICK {
                        thread::sleep(Duration::from_secs_f32(
                            game::SECONDS_PER_TICK - last_tick.elapsed().as_secs_f32(),
                        ));
                    }
                    last_tick = Instant::now();
                }
            })
            .unwrap();
        ServerHandle { stop_sender }
    }

    pub fn stop(self) {
        self.stop_sender
            .send(())
            .expect("Server stop channel disconnected unexpectedly.");
    }
}
