use crate::{
    audio::{AudioHandle, AudioPlayerState, Sound, SoundTemplate},
    game::world::Location,
};
use cpal::traits::{DeviceTrait, HostTrait};
use flexblock_synth::{start_stream, SampleProvider};
use std::sync::mpsc;

const MONO_SAMPLES_SIZE: usize = 8192;

pub enum AudioMessage {
    StartSound(usize, Option<Location>),
    PlayerState(AudioPlayerState),
}

pub struct AudioManager {
    sound_templates: Vec<Box<dyn SoundTemplate>>,
    current_audio: Vec<Box<dyn Sound>>,
    mono_samples: [f32; MONO_SAMPLES_SIZE],
    audio_message_receiver: mpsc::Receiver<AudioMessage>,
    audio_message_sender: mpsc::Sender<AudioMessage>,
    player_state: AudioPlayerState,
}

impl AudioManager {
    pub fn new() -> AudioManager {
        let (sender, receiver) = mpsc::channel();
        AudioManager {
            sound_templates: Vec::new(),
            current_audio: Vec::new(),
            mono_samples: [0.; MONO_SAMPLES_SIZE],
            audio_message_receiver: receiver,
            audio_message_sender: sender,
            player_state: AudioPlayerState::default(),
        }
    }

    fn handle_message(&mut self, message: AudioMessage) {
        match message {
            AudioMessage::StartSound(sound_index, location) => {
                if sound_index >= self.sound_templates.len() {
                    panic!("No such sound. Sound index: {}", sound_index)
                }
                self.current_audio
                    .push(self.sound_templates[sound_index].create_instance(location))
            }
            AudioMessage::PlayerState(player_state) => {
                self.player_state = player_state;
            }
        };
    }

    pub fn add_sound(&mut self, sound: Box<dyn SoundTemplate>) {
        self.sound_templates.push(sound);
    }

    pub fn start(self) -> AudioHandle {
        let (sender, receiver) = mpsc::channel();
        let audio_message_sender = self.audio_message_sender.clone();

        let audio_thread = std::thread::spawn(move || {
            let host = cpal::default_host();
            let device = host.default_output_device().unwrap();
            println!("Chosen device: {}", device.name().unwrap());

            let mut supported_configs_range = device
                .supported_output_configs()
                .expect("error while querying configs");

            let supported_stream_config_range = supported_configs_range
                .next()
                .expect("no supported config?!");

            println!(
                "Max sample rate: {:?}",
                supported_stream_config_range.max_sample_rate()
            );
            println!(
                "Min sample rate: {:?}",
                supported_stream_config_range.min_sample_rate()
            );

            let supported_config = supported_stream_config_range.with_max_sample_rate();

            println!("Sample format: {:?}", supported_config.sample_format());
            println!("Number of channels: {:?}", supported_config.channels());

            let sample_rate = supported_config.sample_rate().0;

            let stream = start_stream(self, device);

            receiver
                .recv()
                .expect("Dropped audio handle without first stopping audio.");
            drop(stream);
        });

        AudioHandle::new(audio_message_sender, sender, audio_thread)
    }
}

fn reset_samples(samples: &mut [f32]) {
    for sample in samples {
        *sample = 0.;
    }
}

fn pan_sample(mono_sample: f32, pan: f32) -> (f32, f32) {
    let (sin, cos) = (std::f32::consts::FRAC_PI_2 * pan).sin_cos();
    (mono_sample * cos, mono_sample * sin)
}

/// Takes a mono sample and transforms it into the stereo samples the player hears using the samples
/// generation location and information about the player's state.
fn mono_to_stereo(
    mut mono_sample: f32,
    location: Location,
    player_state: &AudioPlayerState,
) -> (f32, f32) {
    let vector = player_state.view().view_vector_to_loc(location);
    let distance = vector.norm();
    if distance == 0. {
        return (0., 0.);
    }
    if distance < 0.25 {
        // If the audio is too close, limit the amplification.
        mono_sample *= 16.;
    } else {
        mono_sample *= f32::powi(distance, -2);
    }
    let pan = (vector.x / distance + 1.) * 0.5;
    pan_sample(mono_sample, pan)
}

impl SampleProvider for AudioManager {
    fn next(&mut self, samples: &mut [f32]) {
        reset_samples(samples);

        let mono_samples = &mut self.mono_samples[0..samples.len() / 2];

        for sound in self.current_audio.iter_mut() {
            sound.next(mono_samples);
            for (i, mono_sample) in mono_samples.iter().enumerate() {
                let (left, right) = if let Some(location) = sound.location() {
                    mono_to_stereo(*mono_sample, location, &self.player_state)
                } else {
                    (*mono_sample * 0.5, *mono_sample * 0.5)
                };
                samples[2 * i] += left;
                samples[2 * i + 1] += right;
            }
        }
        self.current_audio.retain(|sound| !sound.is_finished());
        loop {
            match self.audio_message_receiver.try_recv() {
                Ok(event) => self.handle_message(event),
                Err(mpsc::TryRecvError::Empty) => break,
                Err(mpsc::TryRecvError::Disconnected) => panic!("Event channel disconnected!"),
            }
        }
    }
}
