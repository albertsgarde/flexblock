use super::wrapper::{EventHandler, Window};
use crate::channels::*;

pub fn start_window(rx: PackingToWindowReceiver, tx: WindowToLogicSender) {
    let window = unsafe { Window::new(rx) };

    let eh: EventHandler = Box::new(move |event| {
        if let Some(event) = super::ExternalEvent::create_from_glut_event(event) {
            tx.channel_sender.send(event).unwrap();
        }
    });

    unsafe { window.run(eh) };
}
