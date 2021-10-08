use crate::modules::{Module, ModuleTemplate};
use flexblock_synth_derive::module;

#[module]
pub struct OnePoleFilter<S: Module, C: Module> {
    source: S,
    coefficient: C,
    prev_sample: f32,
}

impl<S: Module, C: Module> OnePoleFilter<S, C> {
    pub fn new(
        source: ModuleTemplate<S>,
        coefficient: ModuleTemplate<C>,
    ) -> ModuleTemplate<OnePoleFilter<S, C>> {
        ModuleTemplate {
            module: OnePoleFilter {
                source: source.module,
                coefficient: coefficient.module,
                prev_sample: 0.,
            },
        }
    }
}

impl<S: Module, C: Module> Module for OnePoleFilter<S, C> {
    fn next(&mut self, sample_num: u64) -> f32 {
        self.prev_sample =
            self.source.next(sample_num) + self.prev_sample * self.coefficient.next(sample_num);
        self.prev_sample
    }
}
