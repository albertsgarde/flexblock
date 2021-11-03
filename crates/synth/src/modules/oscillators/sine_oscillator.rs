use crate::modules::{Module, ModuleTemplate};
use synth_derive::module;

#[module]
pub struct SineOscillator<F: Module> {
    /// A module that supplies the frequency each sample.
    frequency: F,
    /// The inverse sample rate multiplied by tau.
    /// This is to simplify the step size calculation to just multiplying this value with the frequency.
    tau_times_inverse_sample_rate: f32,
    /// The current position round the unit circle.
    current_radians: f32,
}

impl<F: Module> SineOscillator<F> {
    pub fn new(
        frequency: ModuleTemplate<F>,
        sample_rate: u32,
    ) -> ModuleTemplate<SineOscillator<F>> {
        ModuleTemplate {
            module: SineOscillator {
                frequency: frequency.module,
                tau_times_inverse_sample_rate: std::f32::consts::TAU / (sample_rate as f32),
                current_radians: 0.,
            },
        }
    }
}

impl<F: Module> Module for SineOscillator<F> {
    fn next(&mut self, sample_num: u64) -> f32 {
        let frequency = self.frequency.next(sample_num);
        self.current_radians += frequency * self.tau_times_inverse_sample_rate;
        self.current_radians %= std::f32::consts::TAU;
        self.current_radians.sin()
    }
}
