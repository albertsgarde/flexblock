use crate::{
    game::{LogicEvent, StateInputEvent},
    graphics::ExternalEvent,
};
use glm::Vec3;
use glutin::event::{ElementState, MouseButton, VirtualKeyCode};
use std::{collections::HashMap, sync::mpsc};

/// Handles external events and produces state input events.
pub struct ExternalEventHandler {
    /// The state of each keyboard key.
    key_state: HashMap<VirtualKeyCode, bool>,
    /// The state of each mouse button.
    button_state: HashMap<MouseButton, bool>,
    /// The state events generated this tick.
    tick_state_events: Vec<StateInputEvent>,
    /// The logic events generated this tick.
    tick_logic_events: Vec<LogicEvent>,
}

impl ExternalEventHandler {
    pub fn new() -> ExternalEventHandler {
        ExternalEventHandler {
            key_state: HashMap::new(),
            button_state: HashMap::new(),
            tick_state_events: Vec::new(),
            tick_logic_events: Vec::new(),
        }
    }

    fn key_state(&self, key_code: VirtualKeyCode) -> bool {
        *self.key_state.get(&key_code).unwrap_or(&false)
    }

    fn button_state(&self, mouse_button: MouseButton) -> bool {
        *self.button_state.get(&mouse_button).unwrap_or(&false)
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
                self.tick_state_events.push(StateInputEvent::RotateView {
                    delta: (0.003 * delta.0 as f32, 0.003 * delta.1 as f32),
                })
            }
            ExternalEvent::KeyboardInput { keycode, state } => {
                if !self.key_state(keycode) {
                    // Handling of key presses should happen here, as the if avoids repeated presses from holding down the button.
                    match keycode {
                        VirtualKeyCode::Space => self.tick_state_events.push(StateInputEvent::Jump),
                        VirtualKeyCode::S => {
                            if self.key_state(VirtualKeyCode::LControl)
                                || self.key_state(VirtualKeyCode::RControl)
                            {
                                self.tick_logic_events.push(LogicEvent::Save)
                            }
                        }
                        VirtualKeyCode::L => {
                            if self.key_state(VirtualKeyCode::LControl)
                                || self.key_state(VirtualKeyCode::RControl)
                            {
                                self.tick_logic_events.push(LogicEvent::LoadLatest)
                            }
                        }
                        _ => {}
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
                            self.tick_state_events
                                .push(StateInputEvent::PlayerInteract1);
                        }
                        MouseButton::Right => {
                            self.tick_state_events
                                .push(StateInputEvent::PlayerInteract2);
                        }
                        _ => {}
                    }
                };
            }
        }
    }

    /// Returns and clears the current event buffer.
    pub fn tick_events(&mut self) -> Vec<StateInputEvent> {
        let mut result = std::mem::replace(&mut self.tick_state_events, Vec::new());
        let mut move_vector = Vec3::new(0., 0., 0.);
        if self.key_state(VirtualKeyCode::W) {
            move_vector += Vec3::new(0., 0., -1.);
        }
        if self.key_state(VirtualKeyCode::D) {
            move_vector += Vec3::new(1., 0., 0.);
        }
        if self.key_state(VirtualKeyCode::S) {
            move_vector += Vec3::new(0., 0., 1.);
        }
        if self.key_state(VirtualKeyCode::A) {
            move_vector += Vec3::new(-1., 0., 0.);
        }
        if move_vector != Vec3::new(0., 0., 0.) {
            result.push(StateInputEvent::MovePlayerRelative { delta: move_vector });
        }
        result
    }
}
