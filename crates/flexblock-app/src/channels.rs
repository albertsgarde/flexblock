use graphics::ExternalEvent;
use graphics::RenderMessages;
use graphics::GraphicsCapabilities;
use game::GraphicsStateModel;
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
    pub render_pack: Arc<Mutex<Option<RenderMessages>>>,
}

pub struct PackingToWindowReceiver {
    pub render_pack: Arc<Mutex<Option<RenderMessages>>>,
}

///The window lets the packing thread know what graphics capabilities are available.
pub struct WindowToPackingSender {
    pub channel_sender: mpsc::Sender<GraphicsCapabilities>,
}

pub struct WindowToPackingReceiver {
    pub channel_receiver: mpsc::Receiver<GraphicsCapabilities>,
}
