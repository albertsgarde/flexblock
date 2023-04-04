use crate::modules::{Module, ModuleTemplate};
use crate::utils::RotatingArray;
use synth_derive::module;

#[module]
/// Delays the incoming signal by a specified amount of time.
pub struct Delay<S: Module, D: Module> {
    buffer: RotatingArray<f32>,
    sample_rate: u32,
    source: S,
    delay: D,
}

impl<S: Module, D: Module> Delay<S, D> {
    /// Creates a new Delay module that takes the signal from `source` and delays it `delay_time` seconds.
    ///
    /// # Arguments
    ///
    /// * `source` - The source signal to delay.
    /// * `delay_time` - The time in seconds to delay the signal.
    /// * `sample_rate` - The used sample rate.
    pub fn new(
        source: ModuleTemplate<S>,
        delay_time: ModuleTemplate<D>,
        max_delay_time: f32,
        sample_rate: u32,
    ) -> ModuleTemplate<Delay<S, D>> {
        let buffer_size = (max_delay_time * sample_rate as f32) as usize+1;
        ModuleTemplate {
            module: Delay {
                buffer: RotatingArray::new(buffer_size, 0.),
                sample_rate,
                source: source.module,
                delay: delay_time.module,
            },
        }
    }
}

impl<S: Module, D: Module> Module for Delay<S, D> {
    fn next(&mut self, sample_num: u64) -> f32 {
        let cur_delay_samples = (self.delay.next(sample_num) * self.sample_rate as f32) as usize;
        if cur_delay_samples >= self.buffer.len() {
            panic!("Delay must not exceed the maximum delay set at initialization. Max delay samples: {}, cur delay samples: {}", self.buffer.len(), cur_delay_samples);
        }
        let source = self.source.next(sample_num);
        self.buffer.set(cur_delay_samples, |sample| sample + source);
        self.buffer.push_pop(0.)
    }
}
