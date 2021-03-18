pub enum ExternalEvent {}

impl ExternalEvent {
    pub fn create_from_glut_event<'a, T: 'static>(
        event: glutin::event::Event<'a, T>,
    ) -> Option<ExternalEvent> {
        match event {
            _ => None, //TODO: ADD THE CASES WE CARE ABOUT!
        }
    }
}
