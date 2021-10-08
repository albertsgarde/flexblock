use crate::modules::{Module, ModuleTemplate};
use synth_derive::module;
use std::sync::{Arc, RwLock, TryLockError};

#[module]
pub struct Input {
    value: Arc<RwLock<f32>>,
    prev_value: f32,
}

impl Input {
    pub fn new(value: Arc<RwLock<f32>>) -> ModuleTemplate<Input> {
        ModuleTemplate {
            module: Input {
                value,
                prev_value: 0.,
            },
        }
    }
}

impl Module for Input {
    fn next(&mut self, _: u64) -> f32 {
        match self.value.try_read() {
            Ok(value) => {
                self.prev_value = *value;
                *value
            }
            Err(err) => match err {
                TryLockError::WouldBlock => self.prev_value,
                TryLockError::Poisoned(poison_error) => {
                    panic!("Control mutex RwLock poisoned! {:?}", poison_error)
                }
            },
        }
    }
}
