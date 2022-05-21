use std::thread::JoinHandle;

use crate::{ClientMessage, ServerMessage};
use game::StateInputEvent;
use log::info;
use tokio::select;
use tokio::{io::Result, net::TcpStream, sync::mpsc};

use crate::packet::{Receiver, Transmitter};

async fn handle_server_connection(
    server_stream: std::net::TcpStream,
    server_messages_to_client: mpsc::UnboundedSender<ServerMessage>,
    mut from_client: mpsc::UnboundedReceiver<ClientMessage>,
) {
    let server_stream =
        TcpStream::from_std(server_stream).expect("Error converting to Tokio TCP stream.");
    let (receiver, sender) = server_stream.into_split();
    let mut sender = Transmitter::new(sender);
    let mut receiver = Receiver::new(receiver);
    let mut running = true;
    while running {
        select! {
            Some(message) = from_client.recv() => {
                if matches!(message, ClientMessage::Disconnect) {
                    running = false;
                }
                sender.write_object(&message).await.expect("Could not send events to server.");
            },
            server_message = receiver.next_object::<ServerMessage>() => {
                let message = server_message.expect("Failed to get and deserialize message from server.");
                if matches!(message, ServerMessage::ShutDown) {
                    info!("Server shut down.");
                    running = false;
                }
                server_messages_to_client.send(message).expect("Failed to send tick events to client.");
            }
        }
    }
}

pub struct Client {
    server_messages: mpsc::UnboundedReceiver<ServerMessage>,
    events_to_server: mpsc::UnboundedSender<ClientMessage>,
    join_handle: JoinHandle<()>,
}

impl Client {
    pub fn start(ip: &str) -> Result<Client> {
        let (server_messages_to_client, server_messages_from_server) = mpsc::unbounded_channel();
        let (events_to_server, events_from_client) = mpsc::unbounded_channel();
        let server_stream = std::net::TcpStream::connect(ip)?;
        server_stream.set_nonblocking(true)?;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let join_handle = std::thread::Builder::new()
            .name("client".to_owned())
            .spawn(move || {
                runtime.block_on(handle_server_connection(
                    server_stream,
                    server_messages_to_client,
                    events_from_client,
                ));
            })
            .unwrap();
        Ok(Self {
            server_messages: server_messages_from_server,
            events_to_server,
            join_handle,
        })
    }

    pub fn next_server_message(&mut self) -> Option<ServerMessage> {
        match self.server_messages.try_recv() {
            Ok(message) => Some(message),
            Err(err) => match err {
                mpsc::error::TryRecvError::Disconnected => panic!("Server disconnected!"),
                mpsc::error::TryRecvError::Empty => None,
            },
        }
    }

    pub fn send_events(&mut self, events: Vec<StateInputEvent>) {
        let message = ClientMessage::Events(events);
        self.events_to_server
            .send(message)
            .expect("Could not send events to server.");
    }

    pub fn disconnect(self) {
        let message = ClientMessage::Disconnect;
        self.events_to_server
            .send(message)
            .expect("Could not send events to server.");
        self.join_handle.join().unwrap();
    }
}
