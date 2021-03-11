use crate::game::InputEvent;
use crate::game::GraphicsStateModel;
use crate::graphics::Bindings;
use std::sync::Mutex;
use tokio::sync::mpsc;

pub struct Update;

pub struct WindowToLogicSender {
    channel_sender: mpsc::Sender<InputEvent>,
}

pub struct WindowToLogicReceiver {
    channel_receiver: mpsc::Receiver<InputEvent>,
}

pub struct LogicToPackingSender {
    channel_sender: mpsc::Sender<Update>,
    graphics_state_model: Mutex<GraphicsStateModel>,
}

pub struct LogicToPackingReceiver {
    channel_receiver: mpsc::Receiver<Update>,
    graphics_state_model: Mutex<GraphicsStateModel>,
}

pub struct PackingToWindowSender {
    bindings: Mutex<Bindings>,
}

pub struct PackingToWindowReceiver {
    bindings: Mutex<Bindings>,
}
