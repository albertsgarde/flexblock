use crate::modules::{Module, ModuleTemplate};
use synth_derive::module;

#[module]
pub struct Envelope {
    /// Seconds of 0 at the beginning before the attack starts.
    delta: f32,
    /// Time to linearly rise from 0 to 1 after delta.
    attack: f32,
    /// Time to linearly fall from 1 to sustain value after attack.
    decay: f32,
    /// Sustain value.
    sustain: f32,
    /// Inverse Sample rate.
    inverse_sample_rate: f32,
}

impl Envelope {
    /// Creates a new Envelope module from the specifications given.
    ///
    /// # Arguments
    ///
    /// * `delta` - Seconds of 0 at the beginning before the attack starts.
    /// * `attack` - Time to linearly rise from 0 to 1 after delta.
    /// * `decay` - Time to linearly fall from 1 to sustain value after attack.
    /// * `sustain` - Sustain value to hold indefinitely.
    /// * `sample_rate` - The sample rate used.
    pub fn new(
        delta: f32,
        attack: f32,
        decay: f32,
        sustain: f32,
        sample_rate: u32,
    ) -> ModuleTemplate<Envelope> {
        if delta < 0. {
            panic!("Delta value must be non-negative. Delta: {}", delta);
        }
        if attack < 0. {
            panic!("Attack value must be non-negative. Attack: {}", attack);
        }
        if decay < 0. {
            panic!("Decay value must be non-negative. Decay: {}", decay);
        }
        if sustain < 0. {
            panic!("Sustain value must be non-negative. Sustain: {}", sustain);
        }
        ModuleTemplate {
            module: Envelope {
                delta,
                attack,
                decay,
                sustain,
                inverse_sample_rate: 1. / (sample_rate as f32),
            },
        }
    }
}

impl Module for Envelope {
    fn next(&mut self, sample_num: u64) -> f32 {
        let mut sample_time = (sample_num as f32) * self.inverse_sample_rate;
        if sample_time <= self.delta {
            return 0.;
        } else {
            sample_time -= self.delta;
        }

        if sample_time <= self.attack {
            return sample_time / self.attack;
        } else {
            sample_time -= self.attack;
        }

        if sample_time <= self.decay {
            1. - (sample_time / self.decay) * (1. - self.sustain)
        } else {
            self.sustain
        }
    }
}
