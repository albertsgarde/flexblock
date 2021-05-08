use midi::Message;

pub trait MidiPlayer {
    fn handle_message(&mut self, message: Message);

    fn next(&mut self) -> (f32, f32);
}
