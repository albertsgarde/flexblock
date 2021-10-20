use std::thread::JoinHandle;

use crate::{ClientMessage, ServerMessage};
use game::StateInputEvent;
use log::info;
use tokio::select;
use tokio::{io::Result, net::TcpStream, sync::mpsc};

use crate::packet::{Receiver, Transmitter};

async fn handle_server_connection(
    server_stream: std::net::TcpStream,
    tick_events_to_client: mpsc::UnboundedSender<Vec<StateInputEvent>>,
    mut from_client: mpsc::UnboundedReceiver<InternalFromClientMessage>,
) {
    let server_stream = TcpStream::from_std(server_stream).expect("Error converting to Tokio TCP stream.");
    let (receiver, sender) = server_stream.into_split();
    let mut sender = Transmitter::new(sender);
    let mut receiver = Receiver::new(receiver);
    let mut running = true;
    while running {
        select! {
            Some(message) = from_client.recv() => {
                match message {
                    InternalFromClientMessage::Message(message) => {
                        sender.write_object(&message).await.expect("Could not send events to server.");
                    }
                    InternalFromClientMessage::Disconnect => {
                        let message = ClientMessage::Disconnect;
                        sender.write_object(&message).await.expect("Could not send events to server.");
                        running = false;
                    }
                }
            },
            server_message = receiver.next_object::<ServerMessage>() => {
                match server_message.expect("Failed to get tick events from server.") {
                    ServerMessage::TickEvents(events) => tick_events_to_client.send(events).expect("Failed to send tick events to client."),
                    ServerMessage::ShutDown => {
                        info!("Server shut down.");
                        running = false;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum InternalFromClientMessage {
    Message(ClientMessage),
    Disconnect,
}

pub struct Client {
    tick_events: mpsc::UnboundedReceiver<Vec<StateInputEvent>>,
    events_to_server: mpsc::UnboundedSender<InternalFromClientMessage>,
    join_handle: JoinHandle<()>,
}

impl Client {
    pub fn start(ip: &str) -> Result<Client> {
        let (tick_events_to_client, tick_events_from_server) =
            mpsc::unbounded_channel();
        let (events_to_server, events_from_client) =
            mpsc::unbounded_channel();
        let server_stream = std::net::TcpStream::connect(ip)?;
        server_stream.set_nonblocking(true)?;
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let join_handle = std::thread::spawn(move || {
            runtime.block_on(handle_server_connection(
                server_stream,
                tick_events_to_client,
                events_from_client,
            ));
        });
        Ok(Self {
            tick_events: tick_events_from_server,
            events_to_server,
            join_handle,
        })
    }

    pub fn next_tick_events(&mut self) -> Option<Vec<StateInputEvent>> {
        match self.tick_events.try_recv() {
            Ok(events) => Some(events),
            Err(err) => match err {
                mpsc::error::TryRecvError::Disconnected => panic!("Server disconnected!"),
                mpsc::error::TryRecvError::Empty => None,
            },
        }
    }

    pub fn send_events(&mut self, events: Vec<StateInputEvent>) {
        let message = InternalFromClientMessage::Message(ClientMessage::Events(events));
        self.events_to_server
            .send(message)
            .expect("Could not send events to server.");
    }

    pub fn disconnect(self) {
        let message = InternalFromClientMessage::Disconnect;
        self.events_to_server
            .send(message)
            .expect("Could not send events to server.");
        self.join_handle.join().unwrap();
    }
}
