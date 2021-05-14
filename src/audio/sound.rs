pub trait Sound: Send {
    fn next(&mut self, samples: &mut [f32]);

    fn is_finished(&self) -> bool;
}

pub trait SoundTemplate: Send {
    fn create_instance(&self) -> Box<dyn Sound>;
}
