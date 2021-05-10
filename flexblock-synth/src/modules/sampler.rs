use crate::modules::{Module, ModuleTemplate};
use flexblock_synth_derive::module;
use std::sync::Arc;

#[module]
pub struct Sampler {
    audio: Arc<[f32]>,
    repeat: bool,
    position: usize,
    end: usize,
}

impl Sampler {
    pub fn new(audio: Arc<[f32]>, repeat: bool) -> ModuleTemplate<Self> {
        let audio_length = audio.len();
        ModuleTemplate {
            module: Sampler {
                audio,
                repeat,
                position: 0,
                end: audio_length,
            },
        }
    }
}

impl Module for Sampler {
    fn next(&mut self, _: u64) -> f32 {
        if self.position == self.end {
            if self.repeat {
                self.position = 0;
            } else {
                return 0.;
            }
        }
        let result = self.audio[self.position];
        self.position += 1;
        result
    }
}
