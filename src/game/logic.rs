use crate::channels::*;
use tokio::task::{self, JoinHandle};

pub fn start_logic_thread(_: WindowToLogicReceiver, _: LogicToPackingSender) -> JoinHandle<()> {
    task::spawn(async {
        println!("Running game logic!");
    })
}
