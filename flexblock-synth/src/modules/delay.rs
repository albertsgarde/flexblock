use crate::modules::{Module, ModuleTemplate};
use crate::utils::RotatingArray;
use flexblock_synth_derive::module;

#[module]
/// Delays the incoming signal by a specified amount of time.
pub struct Delay<S: Module> {
    buffer: RotatingArray<f32>,
    source: S,
}

impl<S: Module> Delay<S> {
    /// Creates a new Delay module that takes the signal from `source` and delays it `delay_time` seconds.
    ///
    /// # Arguments
    ///
    /// * `source` - The source signal to delay.
    /// * `delay_time` - The time in seconds to delay the signal.
    /// * `sample_rate` - The used sample rate.
    pub fn new(
        source: ModuleTemplate<S>,
        delay_time: f32,
        sample_rate: u32,
    ) -> ModuleTemplate<Delay<S>> {
        let buffer_size = (delay_time * sample_rate as f32) as usize;
        ModuleTemplate {
            module: Delay {
                buffer: RotatingArray::new(buffer_size, 0.),
                source: source.module,
            },
        }
    }
}

impl<S: Module> Module for Delay<S> {
    fn next(&mut self, sample_num: u64) -> f32 {
        self.buffer.push_pop(self.source.next(sample_num))
    }
}
