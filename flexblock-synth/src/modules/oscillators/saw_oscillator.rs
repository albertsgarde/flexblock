use crate::modules::{Module, ModuleTemplate};
use flexblock_synth_derive::module;

#[module]
pub struct SawOscillator<F: Module> {
    frequency: F,
    inverse_sample_rate: f32,
    cur_pos: f32,
}

impl<F: Module> SawOscillator<F> {
    pub fn new(frequency: ModuleTemplate<F>, sample_rate: u32) -> ModuleTemplate<SawOscillator<F>> {
        ModuleTemplate {
            module: SawOscillator {
                frequency: frequency.module,
                inverse_sample_rate: 1. / (sample_rate as f32),
                cur_pos: 0.,
            },
        }
    }
}

impl<F: Module> Module for SawOscillator<F> {
    fn next(&mut self, sample_num: u64) -> f32 {
        self.cur_pos += self.frequency.next(sample_num) * self.inverse_sample_rate;
        self.cur_pos %= 1.;
        self.cur_pos - 0.5
    }
}
