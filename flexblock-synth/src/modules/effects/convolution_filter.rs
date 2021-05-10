use crate::modules::{Module, ModuleTemplate};
use crate::utils::RotatingArray;
use flexblock_synth_derive::module;
use std::f32::consts::*;

/// A convolution filter convoles the input signal with a kernel.
#[module]
pub struct ConvolutionFilter<S: Module> {
    source: S,
    kernel: Vec<f32>,
    prev_inputs: RotatingArray<f32>,
}

impl<S: Module> ConvolutionFilter<S> {
    /// Creates a new ConvolutionFilter with given source module and kernel.
    ///
    /// # Arguments
    ///
    /// * `source` - The module to get inputs from.
    /// * `kernel` - The kernel to convolve the signal with.
    pub fn new(
        source: ModuleTemplate<S>,
        kernel: Vec<f32>,
    ) -> ModuleTemplate<ConvolutionFilter<S>> {
        let kernel_length = kernel.len();
        ModuleTemplate {
            module: ConvolutionFilter {
                source: source.module,
                kernel,
                prev_inputs: RotatingArray::new(kernel_length, 0.),
            },
        }
    }
}

impl<S: Module> Module for ConvolutionFilter<S> {
    fn next(&mut self, sample_num: u64) -> f32 {
        self.prev_inputs.push(self.source.next(sample_num));
        self.kernel
            .iter()
            .zip(self.prev_inputs.iter())
            .map(|(k, s)| k * s)
            .sum()
    }
}

/// Creates a low-pass filter. Frequencies below the cutoff are preserved when
/// samples are convolved with this filter.
/// Shamelessly stolen from gyng/synthrs.
pub fn lowpass_filter(cutoff: f32, size: usize) -> Vec<f32> {
    let size = if size % 2 == 1 { size + 1 } else { size };

    let sinc = |x: f32| -> f32 {
        if x == 0. {
            1.
        } else {
            (x * PI).sin() / (x * PI)
        }
    };

    let sinc_wave: Vec<f32> = (0..size)
        .map(|i| sinc(2.0 * cutoff * (i as f32 - (size as f32 - 1.0) / 2.0)))
        .collect();

    let blackman_window = blackman_window(size);

    let filter: Vec<f32> = sinc_wave
        .iter()
        .zip(blackman_window.iter())
        .map(|tup| *tup.0 * *tup.1)
        .collect();

    // Normalize
    let sum = filter.iter().fold(0.0, |acc, &el| acc + el);

    filter.iter().map(|&el| el / sum).collect()
}

/// Creates a Blackman window of a given size.
/// Shamelessly stolen from gyng/synthrs.
fn blackman_window(size: usize) -> Vec<f32> {
    (0..size)
        .map(|i| {
            0.42 - 0.5 * (2.0 * PI * i as f32 / (size as f32 - 1.0)).cos()
                + 0.08 * (4.0 * PI * i as f32 / (size as f32 - 1.0)).cos()
        })
        .collect()
}
