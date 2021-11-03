use crate::modules::{Module, ModuleTemplate};
use synth_derive::module;

#[module]
pub struct Add<L: Module, R: Module> {
    lhs: L,
    rhs: R,
}

impl<L: Module, R: Module> Add<L, R> {
    pub fn new(lhs: ModuleTemplate<L>, rhs: ModuleTemplate<R>) -> ModuleTemplate<Self> {
        ModuleTemplate {
            module: Add {
                lhs: lhs.module,
                rhs: rhs.module,
            },
        }
    }
}

impl<L: Module, R: Module> Module for Add<L, R> {
    fn next(&mut self, sample_num: u64) -> f32 {
        self.lhs.next(sample_num) + self.rhs.next(sample_num)
    }
}
