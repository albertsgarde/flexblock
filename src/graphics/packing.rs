use crate::channels::*;
use tokio::task::{self, JoinHandle};

pub fn start_packing_thread(_: LogicToPackingReceiver, _: PackingToWindowSender) -> JoinHandle<()> {
    task::spawn(async {
        println!("Ready to pack graphics bindings!");
    })
}
