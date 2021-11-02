use crate::AudioMessage;
use std::{sync::mpsc::Sender, thread::JoinHandle};

pub trait AudioMessageHandle {
    fn send_message(&self, message: AudioMessage);
}

pub struct AudioMessageSender {
    audio_message_sender: Sender<AudioMessage>,
}

impl AudioMessageHandle for AudioMessageSender {
    fn send_message(&self, message: AudioMessage) {
        self.audio_message_sender.send(message).expect("Cannot send message to audio as it has disconnected.");
    }
}

pub struct AudioMessageIgnorer;

impl AudioMessageHandle for AudioMessageIgnorer {
    fn send_message(&self, _: AudioMessage) {}
}

pub struct AudioHandle {
    audio_message_sender: Sender<AudioMessage>,
    audio_stop_sender: Sender<()>,
    audio_thread: JoinHandle<()>,
}

impl AudioHandle {
    pub(super) fn new(
        audio_message_sender: Sender<AudioMessage>,
        audio_stop_sender: Sender<()>,
        audio_thread: JoinHandle<()>,
    ) -> Self {
        AudioHandle {
            audio_message_sender,
            audio_stop_sender,
            audio_thread,
        }
    }

    pub fn audio_message_handle(&self) -> AudioMessageSender {
        AudioMessageSender {
            audio_message_sender: self.audio_message_sender.clone(),
        }
    }

    pub fn stop_audio(self) {
        self.audio_stop_sender
            .send(())
            .expect("Audio is already closed.");
        self.audio_thread.join().expect("Audio thread panicked.");
    }
}
