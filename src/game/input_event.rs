use std::sync::mpsc;

/// Represents the event of something happening outside of state that the state might need to react to.
/// Examples are player actions and game closing.
pub enum InputEvent {
    Nothing, //TODO: THIS IS SJUSK
}

// TODO: THIS IMPL BLOCK IS SJUSK
impl InputEvent {
    pub fn input_event_from_external(_e: crate::graphics::ExternalEvent) -> InputEvent {
        InputEvent::Nothing
    }
}

/// Represents the entire history of input events.
pub struct InputEventHistory {
    input_events: Vec<Vec<InputEvent>>,
}

impl InputEventHistory {
    /// Creates a new history with no events stored.
    pub fn new() -> InputEventHistory {
        InputEventHistory {
            input_events: Vec::new(),
        }
    }

    /// Empties the channel and stores the events as the events for a new tick.
    ///
    /// # Arguments
    ///
    /// `input_event_receiver` - The channel to empty of events.
    /// TODO: THIS SHOULDN'T TAKE EXTERNAL EVENTS, IT SHOULD TAKE INPUT EVENTS
    pub fn handle_inputs(
        &mut self,
        input_event_receiver: &mpsc::Receiver<crate::graphics::ExternalEvent>,
    ) {
        let mut tick_input_events = Vec::new();
        loop {
            match input_event_receiver.try_recv() {
                Ok(input_event) => {
                    tick_input_events.push(InputEvent::input_event_from_external(input_event))
                }
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => panic!("Event channel disconnected!"),
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
    pub fn get_events(&'_ self, tick_num: usize) -> Option<&'_ [InputEvent]> {
        self.input_events.get(tick_num).map(|vec| &vec[..])
    }

    /// Returns the events for the latest tick.
    pub fn cur_tick_events(&'_ self) -> Option<&'_ [InputEvent]> {
        self.input_events.last().map(|vec| &vec[..])
    }

    /// Returns the current tick number.
    pub fn cur_tick_num(&self) -> usize {
        self.input_events.len()
    }
}
