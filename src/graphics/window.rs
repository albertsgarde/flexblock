use crate::channels::*;
use tokio::task::{self, JoinHandle};

pub fn start_window_thread(_: PackingToWindowReceiver, _: WindowToLogicSender) -> JoinHandle<()> {
    task::spawn(async {
        
    })
}
