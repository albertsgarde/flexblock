use crate::modules::{Module, ModuleTemplate};
use flexblock_synth_derive::module;
use rand::Rng;
use rand_distr::StandardNormal;

#[module]
pub struct NoiseOscillator<R: Rng + Clone> {
    rng: R,
}

impl<R: Rng + Clone> NoiseOscillator<R> {
    pub fn new(rng: R) -> ModuleTemplate<NoiseOscillator<R>> {
        ModuleTemplate {
            module: NoiseOscillator { rng },
        }
    }
}

impl<R: Rng + Clone> Module for NoiseOscillator<R> {
    fn next(&mut self, _: u64) -> f32 {
        self.rng.sample(StandardNormal)
    }
}
