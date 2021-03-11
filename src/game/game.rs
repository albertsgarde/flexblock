use crate::channels::*;
use tokio::task::{self, JoinHandle};

pub fn start_game_thread(_: WindowToLogicReceiver, _: LogicToPackingSender) -> JoinHandle<()> {
    task::spawn(async {})
}
