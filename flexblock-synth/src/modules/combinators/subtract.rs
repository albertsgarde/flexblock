use crate::modules::{Module, ModuleTemplate};
use flexblock_synth_derive::module;

#[module]
pub struct Subtract<L: Module, R: Module> {
    lhs: L,
    rhs: R,
}

impl<L: Module, R: Module> Subtract<L, R> {
    pub fn new(lhs: ModuleTemplate<L>, rhs: ModuleTemplate<R>) -> ModuleTemplate<Self> {
        ModuleTemplate {
            module: Subtract {
                lhs: lhs.module,
                rhs: rhs.module,
            },
        }
    }
}

impl<L: Module, R: Module> Module for Subtract<L, R> {
    fn next(&mut self, sample_num: u64) -> f32 {
        self.lhs.next(sample_num) - self.rhs.next(sample_num)
    }
}
