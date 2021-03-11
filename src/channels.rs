use crate::game::InputEvent;
use crate::game::GraphicsStateModel;
use crate::graphics::Bindings;
use std::sync::Mutex;
use tokio::sync::mpsc;

pub struct Update;

pub struct WindowToLogicSender {
    pub channel_sender: mpsc::UnboundedSender<InputEvent>,
}

pub struct WindowToLogicReceiver {
    pub channel_receiver: mpsc::UnboundedReceiver<InputEvent>,
}

pub struct LogicToPackingSender {
    pub channel_sender: mpsc::Sender<Update>,
    pub graphics_state_model: Mutex<GraphicsStateModel>,
}

pub struct LogicToPackingReceiver {
    pub channel_receiver: mpsc::Receiver<Update>,
    pub graphics_state_model: Mutex<GraphicsStateModel>,
}

pub struct PackingToWindowSender {
    pub bindings: Mutex<Bindings>,
}

pub struct PackingToWindowReceiver {
    pub bindings: Mutex<Bindings>,
}
