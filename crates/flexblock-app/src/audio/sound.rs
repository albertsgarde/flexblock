pub trait Sound: Send {
    fn next(&mut self, samples: &mut [f32]);

    fn is_finished(&self) -> bool;

    fn location(&self) -> Option<crate::game::world::Location>;
}

pub trait SoundTemplate: Send {
    fn create_instance(&self, location: Option<crate::game::world::Location>) -> Box<dyn Sound>;
}
