use std::{path::Path, sync::Arc};

#[derive(Debug)]
pub enum AudioLoadError {
    HoundError(hound::Error),
    InvalidSampleFormat,
    Not32BitsPerSample,
}

pub struct Audio {
    audio: Vec<Arc<[f32]>>,
    num_channels: usize,
    sample_rate: u32,
}

impl Audio {
    pub fn load<P: AsRef<Path>>(path: P, sample_rate: u32) -> Result<Audio, AudioLoadError> {
        match hound::WavReader::open(path) {
            Err(error) => Err(AudioLoadError::HoundError(error)),
            Ok(wav_reader) => {
                let spec = wav_reader.spec();
                if spec.sample_format != hound::SampleFormat::Float {
                    Err(AudioLoadError::InvalidSampleFormat)
                } else if spec.bits_per_sample != 32 {
                    Err(AudioLoadError::Not32BitsPerSample)
                } else {
                    let num_channels = spec.channels as usize;
                    let mut audio: Vec<Vec<f32>> =
                        vec![Vec::with_capacity(wav_reader.duration() as usize); num_channels];
                    let mut channel = 0;
                    for sample in wav_reader.into_samples() {
                        audio[channel].push(sample.expect(
                            "This should never fail as the bits per sample are checked above.",
                        ));
                        channel += 1;
                        if channel >= num_channels {
                            channel = 0;
                        }
                    }
                    Ok(Audio {
                        audio: audio.into_iter().map(|vec| (*vec).into()).collect(),
                        num_channels,
                        sample_rate,
                    })
                }
            }
        }
    }

    pub fn num_channels(&self) -> usize {
        self.num_channels
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn channel(&self, channel_num: usize) -> Arc<[f32]> {
        self.audio[channel_num].clone()
    }
}
