use glutin::event::*;

pub enum ExternalEvent {
    MouseMotion {
        delta: (f64, f64),
    },
    KeyboardInput {
        key_code: VirtualKeyCode,
        state: ElementState,
    },
    MouseInput {
        button: MouseButton,
        state: ElementState,
    },
}

impl ExternalEvent {
    pub fn create_from_glut_event<T: 'static>(event: Event<'_, T>) -> Option<ExternalEvent> {
        match event {
            // TODO: Should different devices be handled differently?
            Event::DeviceEvent {
                device_id: _,
                event,
            } => match event {
                DeviceEvent::MouseMotion { delta } => Some(ExternalEvent::MouseMotion { delta }),
                _ => None,
            },
            Event::WindowEvent {
                window_id: _,
                event,
            } => match event {
                WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                    is_synthetic,
                } => {
                    if is_synthetic {
                        // TODO: Should synthetic events be discarded? They only occur on Windows and "X11",
                        // so to be consistent across platforms they are for now.
                        None
                    } else {
                        input
                            .virtual_keycode
                            .map(|key_code| ExternalEvent::KeyboardInput {
                                key_code: key_code,
                                state: input.state,
                            })
                    }
                }
                // To avoid annoying warning on the modifiers field which isn't used anyway.
                #[allow(deprecated)]
                WindowEvent::MouseInput {
                    device_id: _,
                    state,
                    button,
                    modifiers: _,
                } => Some(ExternalEvent::MouseInput { button, state }),
                _ => None,
            },
            _ => None,
        }
    }
}
