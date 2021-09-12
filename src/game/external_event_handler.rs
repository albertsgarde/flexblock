use crate::{game::StateInputEvent, graphics::ExternalEvent};
use glm::Vec3;
use glutin::event::{ElementState, MouseButton, VirtualKeyCode};
use std::{collections::HashMap, sync::mpsc};

/// Handles external events and produces state input events.
pub struct ExternalEventHandler {
    /// The state of each keyboard key.
    key_state: HashMap<VirtualKeyCode, bool>,
    /// The state of each mouse button.
    button_state: HashMap<MouseButton, bool>,
    /// The events generated this tick.
    tick_events: Vec<StateInputEvent>,
}

impl ExternalEventHandler {
    pub fn new() -> ExternalEventHandler {
        ExternalEventHandler {
            key_state: HashMap::new(),
            button_state: HashMap::new(),
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

    /// Handles the ExternalEvent by turning it into the right StateInputEvents.
    fn handle_event(&mut self, event: ExternalEvent) {
        match event {
            ExternalEvent::MouseMotion { delta } => {
                self.tick_events.push(StateInputEvent::RotateView {
                    delta: (0.003 * delta.0 as f32, 0.003 * delta.1 as f32),
                })
            }
            ExternalEvent::KeyboardInput { keycode, state } => {
                if !self.key_state.get(&keycode).unwrap_or(&false) {
                    // Handling of key presses should happen here, as the if avoids repeated presses from holding down the button.
                    if keycode == VirtualKeyCode::Space {
                        self.tick_events.push(StateInputEvent::Jump);
                    }
                }
                self.key_state
                    .insert(keycode, state == ElementState::Pressed);
            }
            ExternalEvent::MouseInput { button, state } => {
                self.button_state
                    .insert(button, state == ElementState::Pressed);
                if state == ElementState::Pressed {
                    match button {
                        MouseButton::Left => {
                            self.tick_events.push(StateInputEvent::PlayerInteract1);
                        }
                        MouseButton::Right => {
                            self.tick_events.push(StateInputEvent::PlayerInteract2);
                        }
                        _ => {}
                    }
                };
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
        if move_vector != Vec3::new(0., 0., 0.) {
            result.push(StateInputEvent::MovePlayerRelative { delta: move_vector });
        }
        result
    }
}
