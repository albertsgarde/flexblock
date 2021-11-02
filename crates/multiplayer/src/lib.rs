mod packet;
mod server_networking;
pub use server_networking::ServerNetworking;
mod client;
pub use client::Client;

use game::StateInputEvent;
use serde::{Deserialize, Serialize};

type ClientId = u32;

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ServerMessage {
    TickEvents(Vec<StateInputEvent>),
    ShutDown,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
enum ClientMessage {
    Events(Vec<StateInputEvent>),
    Disconnect,
}
