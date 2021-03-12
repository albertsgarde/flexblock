use std::sync::mpsc;

/// Represents the event of something happening outside of state that the state might need to react to.
/// Examples are player actions and game closing.
pub enum InputEvent {}

/// Represents the entire history of input events.
pub struct InputEventHistory {
    input_events: Vec<Vec<InputEvent>>
}

impl InputEventHistory {
    /// Creates a new history with no events stored.
    pub fn new() -> InputEventHistory {
        InputEventHistory { input_events: Vec::new()}
    }

    /// Empties the channel and stores the events as the events for a new tick.
    /// 
    /// # Arguments
    /// 
    /// `input_event_receiver` - The channel to empty of events.
    pub fn handle_inputs(&mut self, input_event_receiver: &mpsc::Receiver<InputEvent>) {
        let mut tick_input_events = Vec::new();
        loop {
            match input_event_receiver.try_recv() {
                Ok(input_event) => tick_input_events.push(input_event),
                Err(Empty) => break,
                Err(Disconnected) => panic!("Event channel disconnected!"),
            }
        }
        self.input_events.push(tick_input_events)
    }

    /// Gets all events stored for the specific tick.
    /// Returns None if the history hasn't reached the given tick number yet.
    /// 
    /// # Arguments
    /// 
    /// `tick_num` - The tick to get events for.
    pub fn get_events<'a>(&'a self, tick_num: usize) -> Option<&'a [InputEvent]> {
        self.input_events.get(tick_num).map(|vec| &vec[..])
    }

    /// Returns the events for the latest tick.
    pub fn cur_tick_events<'a>(&'a self) -> Option<&'a [InputEvent]> {
        self.input_events.last().map(|vec| &vec[..])
    }

    /// Returns the current tick number.
    pub fn cur_tick_num(&self) -> usize {
        self.input_events.len()
    }
}
