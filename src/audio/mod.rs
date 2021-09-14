mod audio_handle;
pub use audio_handle::{AudioHandle, AudioMessageHandle};
mod audio_manager;
pub use audio_manager::{AudioManager, AudioMessage};
mod audio_setup;
pub use audio_setup::setup_audio;
mod sound;
use sound::Sound;
use sound::SoundTemplate;
mod synth;
pub use synth::Synth;
mod listener;
pub use listener::Listener;

/// Number of audio samples per game tick.
/// Will be made a runtime constant when the sample rate is allowed to vary.
pub const SAMPLES_PER_TICK: u32 = crate::game::TPS * 48000;
/// Reciprocal of SAMPLES_PER_TICK.
pub const TICKS_PER_SAMPLE: f32 = 1. / SAMPLES_PER_TICK as f32;
