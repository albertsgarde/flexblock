use crate::graphics::ExternalEvent;
use glm::Vec3;
use glutin::event::{ElementState, VirtualKeyCode};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::mpsc};

/// Represents the event of something happening outside of state that the state might need to react to.
/// Examples are player actions and game closing.
#[derive(Debug, Serialize, Deserialize)]
pub enum StateInputEvent {
    /// Rotates the view along the great circle in the delta direction by |delta| radians.
    RotateView { delta: (f32, f32) },
    /// Makes the player move in the direction given in view coordinates.
    MovePlayerRelative { delta: Vec3 },
}

/// Handles external events and produces state input events.
pub struct ExternalEventHandler {
    key_state: HashMap<VirtualKeyCode, bool>,
    tick_events: Vec<StateInputEvent>,
}

impl ExternalEventHandler {
    pub fn new() -> ExternalEventHandler {
        ExternalEventHandler {
            key_state: HashMap::new(),
            tick_events: Vec::new(),
        }
    }

    /// Empties the channel of new events and handles them.
    pub fn handle_inputs(
        &mut self,
        input_event_receiver: &mpsc::Receiver<crate::graphics::ExternalEvent>,
    ) {
        loop {
            match input_event_receiver.try_recv() {
                Ok(event) => self.handle_event(event),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => panic!("Event channel disconnected!"),
            }
        }
    }

    fn handle_event(&mut self, event: ExternalEvent) {
        match event {
            ExternalEvent::MouseMotion { delta } => {
                self.tick_events.push(StateInputEvent::RotateView {
                    delta: (0.003 * delta.0 as f32, 0.003 * delta.1 as f32),
                })
            }
            ExternalEvent::KeyboardInput { keycode, state } => {
                self.key_state
                    .insert(keycode, state == ElementState::Pressed);
            }
        }
    }

    /// Returns and clears the current event buffer.
    pub fn tick_events(&mut self) -> Vec<StateInputEvent> {
        let mut result = std::mem::replace(&mut self.tick_events, Vec::new());
        let mut move_vector = Vec3::new(0., 0., 0.);
        if let Some(true) = self.key_state.get(&VirtualKeyCode::W) {
            move_vector += Vec3::new(0., 0., -1.);
        }
        if let Some(true) = self.key_state.get(&VirtualKeyCode::D) {
            move_vector += Vec3::new(1., 0., 0.);
        }
        if let Some(true) = self.key_state.get(&VirtualKeyCode::S) {
            move_vector += Vec3::new(0., 0., 1.);
        }
        if let Some(true) = self.key_state.get(&VirtualKeyCode::A) {
            move_vector += Vec3::new(-1., 0., 0.);
        }
        if let Some(true) = self.key_state.get(&VirtualKeyCode::Space) {
            move_vector += Vec3::new(0., 1., 0.);
        }
        if let Some(true) = self.key_state.get(&VirtualKeyCode::LShift) {
            move_vector += Vec3::new(0., -1., 0.);
        }
        if move_vector != Vec3::new(0., 0., 0.) {
            result.push(StateInputEvent::MovePlayerRelative { delta: move_vector });
        }
        result
    }
}

/// Represents the entire history of input events.
pub struct InputEventHistory {
    input_events: Vec<Vec<StateInputEvent>>,
}

impl InputEventHistory {
    /// Creates a new history with no events stored.
    pub fn new() -> InputEventHistory {
        InputEventHistory {
            input_events: Vec::new(),
        }
    }

    /// Receive the events for the next tick.
    pub fn receive_tick_events(&mut self, events: Vec<StateInputEvent>) {
        self.input_events.push(events)
    }

    /// Gets all events stored for the specific tick.
    /// Returns None if the history hasn't reached the given tick number yet.
    ///
    /// # Arguments
    ///
    /// `tick_num` - The tick to get events for.
    pub fn get_events(&'_ self, tick_num: usize) -> Option<&'_ [StateInputEvent]> {
        self.input_events.get(tick_num).map(|vec| &vec[..])
    }

    /// Returns the events for the latest tick.
    pub fn cur_tick_events(&'_ self) -> Option<&'_ [StateInputEvent]> {
        self.input_events.last().map(|vec| &vec[..])
    }

    /// Returns the current tick number.
    pub fn cur_tick_num(&self) -> usize {
        self.input_events.len()
    }
}
