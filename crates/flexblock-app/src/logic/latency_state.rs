use audio::AudioMessageSender;
use game::{GraphicsStateModel, InputEventHistory, State, StateInputEvent};

use super::sacred_state::SacredState;


pub struct LatencyState {
    state: State,
    local_event_history: InputEventHistory,
}

impl LatencyState {
    pub fn from_sacred_state(sacred_state: &SacredState) -> LatencyState {
        LatencyState { state: sacred_state.state().clone(), local_event_history: sacred_state.input_event_history().clone() }
    }

    pub fn tick(&mut self, events: Vec<StateInputEvent>, audio_message_handle: &AudioMessageSender) {
        self.local_event_history.receive_tick_events(events);
        let tick_events = self.local_event_history.cur_tick_events().unwrap();
        self.state.tick(tick_events, audio_message_handle);
    }

    /// Updates the graphics model with any changes in the state.
    pub fn update_graphics_state_model(&self, graphics_state_model: &mut GraphicsStateModel) {
        self.state.update_graphics_state_model(graphics_state_model);
    }
}
