use crate::channels::*;
use std::thread::{self, JoinHandle};

pub fn start_logic_thread(_: WindowToLogicReceiver, _: LogicToPackingSender) -> JoinHandle<()> {
    thread::spawn(|| {
        println!("Running game logic!");
    })
}
