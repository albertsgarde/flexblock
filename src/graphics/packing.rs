use crate::channels::*;
use std::thread::{self, JoinHandle};

pub fn start_packing_thread(_: LogicToPackingReceiver, _: PackingToWindowSender) -> JoinHandle<()> {
    thread::spawn(|| {
        println!("Ready to pack graphics bindings!");
    })
}
