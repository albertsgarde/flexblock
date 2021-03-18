use glutin::event::*;

pub enum ExternalEvent {
    MouseMotion {
        delta: (f64, f64),
    },
    KeyboardInput {
        keycode: VirtualKeyCode,
        state: ElementState,
    },
}

impl ExternalEvent {
    pub fn create_from_glut_event<'a, T: 'static>(event: Event<'a, T>) -> Option<ExternalEvent> {
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
                    } else if let Some(keycode) = input.virtual_keycode {
                        Some(ExternalEvent::KeyboardInput {
                            keycode,
                            state: input.state,
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        }
    }
}
