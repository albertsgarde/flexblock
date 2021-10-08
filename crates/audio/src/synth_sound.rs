use crate::{Sound, SoundTemplate};
use synth::modules::{Module, ModuleTemplate};
use world::Location;

pub struct SynthSound<M: Module> {
    module: M,
    length: u64,
    cur_time: u64,
    location: Option<Location>,
}

impl<M: Module> SynthSound<M> {
    fn new(module: M, length: u64, location: Option<Location>) -> Self {
        SynthSound {
            module,
            length,
            cur_time: 0,
            location,
        }
    }
}

impl<M: Module + Send> Sound for SynthSound<M> {
    fn next(&mut self, samples: &mut [f32]) {
        for sample in samples {
            *sample = self.module.next(self.cur_time);
            self.cur_time += 1;
            if self.cur_time >= self.length {
                break;
            }
        }
    }

    fn is_finished(&self) -> bool {
        self.cur_time >= self.length
    }

    fn location(&self) -> Option<Location> {
        self.location
    }
}

pub struct SynthTemplate<M: Module + 'static> {
    module_template: ModuleTemplate<M>,
    sound_length: u64,
}

impl<M: Module> SynthTemplate<M> {
    pub fn new(module_template: ModuleTemplate<M>, sound_length: u64) -> Self {
        SynthTemplate {
            module_template,
            sound_length,
        }
    }
}

impl<M: Module + Send> SoundTemplate for SynthTemplate<M> {
    fn create_instance(&self, location: Option<Location>) -> Box<dyn Sound> {
        Box::new(SynthSound::new(
            self.module_template.create_instance(),
            self.sound_length,
            location,
        ))
    }
}
