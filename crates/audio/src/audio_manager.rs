use crate::{AudioHandle, Listener, Sound, SoundTemplate};
use cpal::traits::{DeviceTrait, HostTrait};
use synth::{start_stream, SampleProvider};
use log::debug;
use std::sync::mpsc;
use world::Location;

use super::listener::ListenerInterpolation;

const MONO_SAMPLES_SIZE: usize = 8192;

pub enum AudioMessage {
    StartSound(usize, Option<Location>),
    Listener(Listener),
}

pub struct AudioManager {
    sound_templates: Vec<Box<dyn SoundTemplate>>,
    current_audio: Vec<Box<dyn Sound>>,
    mono_samples: [f32; MONO_SAMPLES_SIZE],
    audio_message_receiver: mpsc::Receiver<AudioMessage>,
    audio_message_sender: mpsc::Sender<AudioMessage>,
    next_listener: Listener,
    listener_interpolation: ListenerInterpolation,
    // The number of samples since last tick.
    tick_sample: u32,
    ticks_per_sample: f32,
}

impl AudioManager {
    pub fn new(tps: u32) -> AudioManager {
        let (sender, receiver) = mpsc::channel();
        AudioManager {
            sound_templates: Vec::new(),
            current_audio: Vec::new(),
            mono_samples: [0.; MONO_SAMPLES_SIZE],
            audio_message_receiver: receiver,
            audio_message_sender: sender,
            next_listener: Listener::default(),
            listener_interpolation: ListenerInterpolation::default(),
            tick_sample: 0,
            ticks_per_sample: tps as f32 / 48000.,
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
            AudioMessage::Listener(listener) => {
                let mut old_listener = listener;
                std::mem::swap(&mut old_listener, &mut self.next_listener);
                self.listener_interpolation = old_listener.interpolate_to(&self.next_listener);
                self.tick_sample = 0;
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
            debug!("Chosen device: {}", device.name().unwrap());

            let mut supported_configs_range = device
                .supported_output_configs()
                .expect("error while querying configs");

            let supported_stream_config_range = supported_configs_range
                .next()
                .expect("no supported config?!");

            debug!(
                "Max sample rate: {:?}",
                supported_stream_config_range.max_sample_rate()
            );
            debug!(
                "Min sample rate: {:?}",
                supported_stream_config_range.min_sample_rate()
            );

            let supported_config = supported_stream_config_range.with_max_sample_rate();

            debug!("Sample format: {:?}", supported_config.sample_format());
            debug!("Number of channels: {:?}", supported_config.channels());

            let _sample_rate = supported_config.sample_rate().0;

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

impl SampleProvider for AudioManager {
    fn next(&mut self, samples: &mut [f32]) {
        reset_samples(samples);

        let mono_samples = &mut self.mono_samples[0..samples.len() / 2];

        for sound in self.current_audio.iter_mut() {
            let mut tick_sample = self.tick_sample;
            sound.next(mono_samples);
            for (i, mono_sample) in mono_samples.iter().enumerate() {
                // Estimate of how much of the current game tick has passed.
                let tick_passed = (tick_sample as f32 * self.ticks_per_sample).min(1.);
                let (left, right) = if let Some(location) = sound.location() {
                    self.listener_interpolation
                        .mono_to_stereo(*mono_sample, location, tick_passed)
                } else {
                    (*mono_sample * 0.5, *mono_sample * 0.5)
                };
                samples[2 * i] += left;
                samples[2 * i + 1] += right;

                tick_sample += 1;
            }
        }
        self.tick_sample += (samples.len() / 2) as u32;
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
