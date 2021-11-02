use std::sync::{Arc, Mutex, TryLockError, mpsc::{self, TryRecvError}};

use multiplayer::Client;

use crate::logic::sacred_state::SacredState;

pub struct ClientHandle {
    stop_sender: mpsc::Sender<()>,
    shared_state: Arc<Mutex<Option<SacredState>>>,
}

impl ClientHandle {
    pub fn start(ip: &str) -> ClientHandle {
        let (stop_sender, stop_receiver) = mpsc::channel();
        let shared_state = Arc::new(Mutex::new(Some(SacredState::new())));
        let mut state = SacredState::new();
        let result = ClientHandle {stop_sender, shared_state: shared_state.clone()};
        let mut client = Client::start(ip).unwrap();

        let mut has_sent_state = true;
        std::thread::spawn(move || {
            loop {
                if let Err(error) = stop_receiver.try_recv() {
                    if let TryRecvError::Disconnected = error {
                        panic!("Client stop channel disconnected unexpectedly.")
                    }
                } else {
                    client.disconnect();
                    break;
                }

                if let Some(tick_events) = client.next_tick_events() {
                    state.tick(tick_events);
                    has_sent_state = false;
                } else if !has_sent_state {
                    match shared_state.try_lock() {
                        Ok(mut mutex) => {*mutex = Some(state.clone()); has_sent_state = true;},
                        Err(TryLockError::Poisoned(error)) => panic!("Client state mutex poisoned. Error: {:?}", error),
                        _ => {},
                    } 
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            }
        });
        result
    }

    pub fn stop(&self) {
        self.stop_sender.send(()).unwrap();
    }

    pub fn state(&self) -> Option<SacredState> {
        let mut mutex = self.shared_state.lock().expect("Client state mutex poisoned");
        std::mem::replace(&mut mutex, None)
    }
}
