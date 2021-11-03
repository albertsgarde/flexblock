pub trait Sound: Send {
    fn next(&mut self, samples: &mut [f32]);

    fn is_finished(&self) -> bool;

    fn location(&self) -> Option<world::Location>;
}

pub trait SoundTemplate: Send {
    fn create_instance(&self, location: Option<world::Location>) -> Box<dyn Sound>;
}
