use crate::{synth_sound::SynthTemplate, AudioHandle, AudioManager};
use synth::modules;
use rand::{thread_rng, Rng, SeedableRng};
use rand_chacha::{self, ChaCha20Rng};

pub fn setup_audio(tps: u32) -> AudioHandle {
    let rng: ChaCha20Rng = rand_chacha::ChaCha20Rng::seed_from_u64(thread_rng().gen());

    let mut audio_manager = AudioManager::new(tps);
    let module = modules::SineOscillator::new((130.).into(), 48000);
    let module = module + modules::NoiseOscillator::new(rng) * 0.2;
    let sound = Box::new(SynthTemplate::new(module * 0.6, (48000. * 0.15) as u64));
    audio_manager.add_sound(sound);

    audio_manager.start()
}
