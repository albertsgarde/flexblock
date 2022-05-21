mod client;
mod packet;
mod sacred_state;
mod server_networking;
pub use sacred_state::SacredState;
mod latency_state;
pub use latency_state::LatencyState;
mod client_handle;
pub use client_handle::ClientHandle;
mod server_handle;
pub use server_handle::ServerHandle;

use game::StateInputEvent;
use serde::{Deserialize, Serialize};

type ClientId = u32;

/// Message from the server to clients.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    /// The events for the next tick.
    TickEvents(Vec<StateInputEvent>),
    /// The total current sacred state.
    /// Used when a new client connects.
    SacredState(SacredState),
    /// Sent when server is shutting down.
    ShutDown,
}

/// Message from a client to the server.
#[derive(Serialize, Deserialize, Debug, Clone)]
enum ClientMessage {
    /// The events from this client for the next tick.
    Events(Vec<StateInputEvent>),
    /// Sent when the client disconnects.
    Disconnect,
}

#[cfg(test)]
mod test {
    use std::sync::{Mutex, Arc};

    use super::*;
    use client::Client;
    use server_networking::ServerNetworking;

    fn wait_steps(steps: u64) {
        const STEP_DUR: u64 = 1000;
        std::thread::sleep(std::time::Duration::from_millis(STEP_DUR * steps));
    }

    #[test]
    #[ignore]
    fn test() {
        const IP: &str = "localhost:15926";
        let server = std::thread::spawn(|| {
            //0
            let state = Arc::new(Mutex::new(SacredState::new()));
            let mut server = ServerNetworking::start(IP, state).unwrap();
            assert_eq!(server.new_events().len(), 0);
            wait_steps(2);
            //2
            assert_eq!(server.num_clients(), 1);
            let new_events = server.new_events();
            assert_eq!(new_events.len(), 1);
            server.send_events(new_events);
            wait_steps(2);
            //4
            assert_eq!(server.num_clients(), 0);
        });

        let client = std::thread::spawn(|| {
            //0
            let mut client = Client::start(IP).unwrap();
            wait_steps(1);
            //1
            let event = StateInputEvent::Jump;
            client.send_events(vec![event]);
            assert!(client.next_server_message().is_none());
            wait_steps(2);
            //3
            match client.next_server_message() {
                Some(ServerMessage::TickEvents(events)) => assert_eq!(events.len(), 1),
                _ => unreachable!(),
            }
            client.disconnect();
        });

        server.join().unwrap();
        client.join().unwrap();
    }

    #[test]
    #[ignore]
    fn two_clients() {
        const IP: &str = "localhost:15926";
        let server = std::thread::spawn(|| {
            //0
            let state = Arc::new(Mutex::new(SacredState::new()));
            let mut server = ServerNetworking::start(IP, state).unwrap();
            assert_eq!(server.new_events().len(), 0);
            wait_steps(2);
            //2
            assert_eq!(server.num_clients(), 1);
            wait_steps(2);
            //4
            assert_eq!(server.num_clients(), 2);
            wait_steps(3);
            //7
            let new_events = server.new_events();
            assert_eq!(new_events.len(), 2);
            server.send_events(new_events);
            wait_steps(2);
            //9
            assert_eq!(server.num_clients(), 1);
            wait_steps(2);
            //11
            assert_eq!(server.num_clients(), 0);
        });

        let client1 = std::thread::spawn(|| {
            wait_steps(1);
            //1
            let mut client = Client::start(IP).unwrap();
            wait_steps(4);
            //5
            let event = StateInputEvent::PlayerInteract1;
            client.send_events(vec![event]);
            assert!(client.next_server_message().is_none());
            wait_steps(3);
            //8
            if let  Some(ServerMessage::TickEvents(events)) = client.next_server_message() {
                assert_eq!(events.len(), 2);
                assert_eq!(events[0], StateInputEvent::PlayerInteract1);
                assert_eq!(events[1], StateInputEvent::PlayerInteract2);
            } else {
                unreachable!();
            }
            
            client.disconnect();
        });

        let client2 = std::thread::spawn(|| {
            wait_steps(3);
            //3
            let mut client = Client::start(IP).unwrap();
            wait_steps(3);
            //6
            let event = StateInputEvent::PlayerInteract2;
            client.send_events(vec![event]);
            assert!(client.next_server_message().is_none());
            wait_steps(4);
            //10
            if let  Some(ServerMessage::TickEvents(events)) = client.next_server_message() {
                assert_eq!(events.len(), 2);
                assert_eq!(events[0], StateInputEvent::PlayerInteract1);
                assert_eq!(events[1], StateInputEvent::PlayerInteract2);
            } else {
                unreachable!();
            }
            client.disconnect();
        });

        server.join().unwrap();
        client1.join().unwrap();
        client2.join().unwrap();
    }
}
