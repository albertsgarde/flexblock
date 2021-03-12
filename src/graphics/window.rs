use crate::channels::*;
use std::thread::{self, JoinHandle};

pub fn start_window_thread(_: PackingToWindowReceiver, _: WindowToLogicSender) -> JoinHandle<()> {
    thread::spawn(|| {
        println!("Managing window!");
    })
}
