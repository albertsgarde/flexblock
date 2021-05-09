use crate::modules::{Module, ModuleTemplate};
use flexblock_synth_derive::module;

#[module]
pub struct Multiply<L: Module, R: Module> {
    source: L,
    factor: R,
}

impl<L: Module, R: Module> Multiply<L, R> {
    pub fn new(
        source: ModuleTemplate<L>,
        factor: ModuleTemplate<R>,
    ) -> ModuleTemplate<Multiply<L, R>> {
        ModuleTemplate {
            module: Multiply {
                source: source.module,
                factor: factor.module,
            },
        }
    }
}

impl<L: Module, R: Module> Module for Multiply<L, R> {
    fn next(&mut self, sample_num: u64) -> f32 {
        self.source.next(sample_num) * self.factor.next(sample_num)
    }
}
