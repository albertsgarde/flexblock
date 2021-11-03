mod oscillators;
pub use oscillators::*;
mod effects;
pub use effects::*;
mod combinators;
pub use combinators::*;
mod envelope;
pub use envelope::*;
mod delay;
pub use delay::*;
mod sampler;
pub use sampler::*;
mod input;
pub use input::*;

#[derive(Clone)]
pub struct ModuleTemplate<M: Module> {
    module: M,
}

impl<M: Module> ModuleTemplate<M> {
    pub fn create_instance(&self) -> M {
        self.module.clone()
    }

    pub fn module(self) -> M {
        self.module
    }
}

impl<L: Module, R: Module> std::ops::Add<ModuleTemplate<R>> for ModuleTemplate<L> {
    type Output = ModuleTemplate<crate::modules::Add<L, R>>;

    fn add(self, rhs: ModuleTemplate<R>) -> Self::Output {
        crate::modules::Add::new(self, rhs)
    }
}

impl<L: Module, R: Module> std::ops::Mul<ModuleTemplate<R>> for ModuleTemplate<L> {
    type Output = ModuleTemplate<crate::modules::Multiply<L, R>>;

    fn mul(self, rhs: ModuleTemplate<R>) -> Self::Output {
        crate::modules::Multiply::new(self, rhs)
    }
}

impl<L: Module> std::ops::Add<f32> for ModuleTemplate<L> {
    type Output = ModuleTemplate<crate::modules::Add<L, f32>>;

    fn add(self, rhs: f32) -> Self::Output {
        crate::modules::Add::new(self, ModuleTemplate { module: rhs })
    }
}

impl<L: Module> std::ops::Mul<f32> for ModuleTemplate<L> {
    type Output = ModuleTemplate<crate::modules::Multiply<L, f32>>;

    fn mul(self, rhs: f32) -> Self::Output {
        crate::modules::Multiply::new(self, ModuleTemplate { module: rhs })
    }
}

impl std::convert::From<f32> for ModuleTemplate<f32> {
    fn from(value: f32) -> Self {
        ModuleTemplate { module: value }
    }
}

pub trait Module: Clone {
    fn next(&mut self, sample_num: u64) -> f32;
}

impl Module for f32 {
    fn next(&mut self, _: u64) -> f32 {
        *self
    }
}
