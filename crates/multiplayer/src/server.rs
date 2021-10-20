use std::collections::HashMap;

use game::StateInputEvent;
use log::{error, info};
use tokio::{
    io::Result,
    net::{TcpListener, TcpStream},
    runtime::Runtime,
    select,
    sync::mpsc,
};

use crate::{
    packet::{Receiver, Transmitter},
    ClientId, ClientMessage, ServerMessage,
};

async fn handle_client_connection(
    socket: TcpStream,
    to_server: std::sync::mpsc::Sender<InternalToServerMessage>,
    mut from_server: mpsc::UnboundedReceiver<InternalFromServerMessage>,
    client_id: ClientId,
) {
    let (receiver, sender) = socket.into_split();
    let mut sender = Transmitter::new(sender);
    let mut receiver = Receiver::new(receiver);
    let mut running = true;
    while running {
        select! {
            Some(server_event) = from_server.recv() => {
                match server_event {
                    InternalFromServerMessage::Disconnect => {running = false;},
                    InternalFromServerMessage::ServerMessage(message) => {
                        if let Err(error) = sender.write_object(&message).await {
                            running = false;
                            to_server.send(InternalToServerMessage::ClientDisconnected(client_id, Box::new(error))).expect("Client event channel unexpectedly closed.");
                        }
                    }
                }
            },
            client_event = receiver.next_object::<ClientMessage>() => {
                match client_event {
                    Ok(message) => {to_server.send(InternalToServerMessage::ClientMessage(client_id, message)).expect("Client event channel unexpectedly closed.");}
                    Err(error) => {
                        running = false;
                        to_server.send(InternalToServerMessage::ClientDisconnected(client_id, Box::new(error))).expect("Client event channel unexpectedly closed.");
                    }
                }
            }
        }
    }
}

async fn listen(
    client_listener: std::net::TcpListener,
    to_server: std::sync::mpsc::Sender<InternalToServerMessage>,
) {
    let client_listener = TcpListener::from_std(client_listener).expect("Error converting to Tokio tcp listener.");
    let mut next_client_id = 0u32;
    loop {
        let (socket, _address) = client_listener
            .accept()
            .await
            .expect("Error accepting new client.");
        let (to_client, from_server) = mpsc::unbounded_channel();
        tokio::spawn(handle_client_connection(
            socket,
            to_server.clone(),
            from_server,
            next_client_id,
        ));
        let client_connection = ClientConnection {
            client_id: next_client_id,
            to_client,
        };
        next_client_id += 1;
        to_server
            .send(InternalToServerMessage::NewClient(client_connection))
            .expect("New client channel unexpectedly closed.");
    }
}

#[derive(Debug)]
enum InternalToServerMessage {
    NewClient(ClientConnection),
    ClientMessage(ClientId, ClientMessage),
    ClientDisconnected(ClientId, Box<dyn std::error::Error + Send>),
}

#[derive(Clone, Debug)]
enum InternalFromServerMessage {
    Disconnect,
    ServerMessage(ServerMessage),
}

#[derive(Debug)]
struct ClientConnection {
    client_id: ClientId,
    to_client: mpsc::UnboundedSender<InternalFromServerMessage>,
}

pub struct Server {
    runtime: Runtime,
    client_connections: HashMap<ClientId, ClientConnection>,
    messages: std::sync::mpsc::Receiver<InternalToServerMessage>,

    new_client_events: Vec<StateInputEvent>,

    shutting_down: bool,
}

impl Server {
    fn new(
        runtime: Runtime,
        messages: std::sync::mpsc::Receiver<InternalToServerMessage>,
    ) -> Self {
        Server {
            runtime,
            client_connections: HashMap::new(),
            messages,
            new_client_events: Vec::new(),
            shutting_down: false,
        }
    }

    pub fn start(ip: &str) -> Result<Self> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let client_listener = std::net::TcpListener::bind(ip)?;
        client_listener.set_nonblocking(true)?;
        let (to_server, from_clients) = std::sync::mpsc::channel();
        runtime.spawn(listen(client_listener, to_server));
        Ok(Self::new(runtime, from_clients))
    }

    fn send_message(&self, message: ServerMessage) {
        let message = InternalFromServerMessage::ServerMessage(message);
        for client in self.client_connections.values() {
            client
                .to_client
                .send(message.clone())
                .expect("Client connection channel closed unexpectedly.");
        }
    }

    pub fn send_events(&mut self, events: Vec<StateInputEvent>) {
        let message = ServerMessage::TickEvents(events);
        self.send_message(message);
    }

    fn handle_messages(&mut self) {
        for message in self.messages.try_iter() {
            match message {
                InternalToServerMessage::NewClient(client_connection) => {
                    if self
                        .client_connections
                        .contains_key(&client_connection.client_id)
                    {
                        panic!("Already have client connection for that ID.");
                    }
                    info!("New client with ID {}.", client_connection.client_id);
                    self.client_connections
                        .insert(client_connection.client_id, client_connection);
                }
                InternalToServerMessage::ClientDisconnected(client_id, error) => {
                    self.client_connections
                        .remove(&client_id)
                        .expect("Have no client connection for disconnected client.");
                    error!("Client disconnected unexpectedly. Error: {:?}", error);
                }
                InternalToServerMessage::ClientMessage(client_id, message) => match message {
                    ClientMessage::Events(mut events) => self.new_client_events.append(&mut events),
                    ClientMessage::Disconnect => {
                        println!("Client {} disconnected.", client_id);
                        info!("Client {} disconnected.", client_id);
                        let client_connection = self
                            .client_connections
                            .remove(&client_id)
                            .expect("No client connection for sender client ID.");
                        client_connection
                            .to_client
                            .send(InternalFromServerMessage::Disconnect)
                            .unwrap();
                    }
                },
            }
        }
    }

    /// Gives a list of the newest events from clients in the order received and clears the internal list.
    pub fn new_events(&mut self) -> Vec<StateInputEvent> {
        self.handle_messages();
        std::mem::replace(&mut self.new_client_events, Vec::new())
    }

    pub fn start_shut_down(&mut self) {
        self.shutting_down = true;
        info!("Sending shut down message.");
        self.send_message(ServerMessage::ShutDown);
    }

    pub fn join_shut_down(mut self) {
        if !self.shutting_down {
            self.start_shut_down();
        }
        info!("Waiting for clients to disconnect...");
        drop(self.runtime);
    }

    pub fn num_clients(&mut self) -> usize {
        self.handle_messages();
        self.client_connections.len()
    }
}
