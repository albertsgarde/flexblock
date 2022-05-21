use std::sync::{
    mpsc::{self, TryRecvError},
    Arc, Mutex, TryLockError,
};

use crate::{client::Client, sacred_state::SacredState, ServerMessage};
use game::StateInputEvent;

enum ClientHandleMessage {
    Stop,
    ClientEvents(Vec<StateInputEvent>),
}

pub struct ClientHandle {
    sender: mpsc::Sender<ClientHandleMessage>,
    shared_state: Arc<Mutex<Option<SacredState>>>,
}

impl ClientHandle {
    pub fn start(ip: &str) -> ClientHandle {
        let (sender, receiver) = mpsc::channel();
        let shared_state = Arc::new(Mutex::new(Some(SacredState::new())));
        let mut state = SacredState::new();
        let result = ClientHandle {
            sender,
            shared_state: shared_state.clone(),
        };
        let mut client = Client::start(ip).unwrap();



        let mut has_sent_state = true;
        std::thread::Builder::new()
            .name("client_handle".to_owned())
            .spawn(move || loop {
                match receiver.try_recv() {
                    Ok(ClientHandleMessage::ClientEvents(events)) => client.send_events(events),
                    Ok(ClientHandleMessage::Stop) => {
                        client.disconnect();
                        break;
                    }
                    Err(error) => {
                        if let TryRecvError::Disconnected = error {
                            panic!("Client stop channel disconnected unexpectedly.")
                        }
                        // Otherwise just try again.
                    }
                }

                if let Some(server_message) = client.next_server_message() {
                    match server_message {
                        ServerMessage::TickEvents(events) => state.tick(events),
                        ServerMessage::SacredState(sacred_state) => state = sacred_state,
                        ServerMessage::ShutDown => todo!(),
                    }
                    has_sent_state = false;
                } else if !has_sent_state {
                    match shared_state.try_lock() {
                        Ok(mut mutex) => {
                            *mutex = Some(state.clone());
                            has_sent_state = true;
                        }
                        Err(TryLockError::Poisoned(error)) => {
                            panic!("Client state mutex poisoned. Error: {:?}", error)
                        }
                        _ => {}
                    }
                } else {
                    std::thread::sleep(std::time::Duration::from_millis(1));
                }
            })
            .unwrap();
        result
    }

    pub fn stop(self) {
        self.sender.send(ClientHandleMessage::Stop).unwrap();
    }

    pub fn send_events(&self, events: Vec<StateInputEvent>) {
        self.sender
            .send(ClientHandleMessage::ClientEvents(events))
            .unwrap();
    }

    pub fn state(&self) -> Option<SacredState> {
        let mut mutex = self
            .shared_state
            .lock()
            .expect("Client state mutex poisoned");
        mutex.take()
    }
}
