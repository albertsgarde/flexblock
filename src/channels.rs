use crate::game::{GraphicsStateModel};
use crate::graphics::RenderPack;
use crate::graphics::ExternalEvent;
use std::sync::{mpsc, Arc, Mutex};

pub struct Update;

pub struct WindowToLogicSender {
    pub channel_sender: mpsc::Sender<ExternalEvent>,
}

pub struct WindowToLogicReceiver {
    pub channel_receiver: mpsc::Receiver<ExternalEvent>,
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
    pub render_pack: Arc<Mutex<Option<RenderPack>>>,
}

pub struct PackingToWindowReceiver {
    pub render_pack: Arc<Mutex<Option<RenderPack>>>,
}
