use crate::game::{GraphicsStateModel, InputEvent};
use crate::graphics::Bindings;
use std::sync::{mpsc, Arc, Mutex};

pub struct Update;

pub struct WindowToLogicSender {
    pub channel_sender: mpsc::Sender<InputEvent>,
}

pub struct WindowToLogicReceiver {
    pub channel_receiver: mpsc::Receiver<InputEvent>,
}

pub struct LogicToPackingSender {
    pub channel_sender: mpsc::Sender<Update>,
    pub graphics_state_model: Arc<Mutex<GraphicsStateModel>>,
}

pub struct LogicToPackingReceiver {
    pub channel_receiver: mpsc::Receiver<Update>,
    pub graphics_state_model: Arc<Mutex<GraphicsStateModel>>,
}

pub struct PackingToWindowSender {
    pub bindings: Arc<Mutex<Bindings>>,
}

pub struct PackingToWindowReceiver {
    pub bindings: Arc<Mutex<Bindings>>,
}
