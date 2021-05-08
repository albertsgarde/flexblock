use midi::Message;

pub struct MidiSequence {
    messages: Vec<Message>,
    deltas: Vec<u64>,
    time: u64,
    next_event: usize,
}

impl MidiSequence {
    pub fn next_messages(&mut self, delta: u64) -> &[Message] {
        self.time += delta;
        let result_slice_start = self.next_event;
        while self.deltas[self.next_event] <= self.time {
            self.time -= self.deltas[self.next_event];
            self.next_event += 1;
        }
        &self.messages[result_slice_start..self.next_event]
    }
}
