use midi::{Channel, Message, U7};
use std::{
    convert::TryInto,
    sync::{Arc, RwLock},
};

#[derive(Debug)]
struct MidiState {
    control_change_values: [Arc<RwLock<f32>>; 128],
    pitch_bend: Arc<RwLock<f32>>,
}

impl MidiState {
    fn new() -> MidiState {
        MidiState {
            control_change_values: (0..128)
                .map(|_| Arc::new(RwLock::new(0.)))
                .collect::<Vec<Arc<RwLock<f32>>>>()
                .try_into()
                .unwrap(),
            pitch_bend: Arc::new(RwLock::new(0.)),
        }
    }
}

pub struct MidiHandler {
    channel_states: [MidiState; 16],
}

impl MidiHandler {
    pub fn new() -> MidiHandler {
        MidiHandler {
            channel_states: (0..16)
                .map(|_| MidiState::new())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    pub fn handle_message(&mut self, message: Message) {
        match message {
            Message::ControlChange(channel, control, value) => {
                *self.channel_states[channel as usize].control_change_values[control as usize]
                    .write()
                    .unwrap() = value as f32
            }
            Message::PitchBend(channel, value) => {
                *self.channel_states[channel as usize]
                    .pitch_bend
                    .write()
                    .unwrap() = value as f32
            }
            _ => {}
        }
    }

    pub fn control_change(&self, channel: Channel, key: U7) -> Arc<RwLock<f32>> {
        self.channel_states[channel as usize].control_change_values[key as usize].clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn control_change_handling() {
        let mut midi_handler = MidiHandler::new();
        let control = midi_handler.control_change(Channel::Ch5, 33);
        assert_eq!(*control.read().unwrap(), 0.);
        midi_handler.handle_message(Message::ControlChange(Channel::Ch5, 33, 52));
        assert_eq!(*control.read().unwrap(), 52.);
        midi_handler.handle_message(Message::ControlChange(Channel::Ch3, 33, 63));
        assert_eq!(*control.read().unwrap(), 52.);
        midi_handler.handle_message(Message::ControlChange(Channel::Ch3, 34, 63));
        assert_eq!(*control.read().unwrap(), 52.);
    }
}
