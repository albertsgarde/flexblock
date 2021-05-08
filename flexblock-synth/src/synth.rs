use crate::modules::Module;
use cpal::traits::{DeviceTrait, StreamTrait};

pub struct Synth<L: Module, R: Module> {
    left: L,
    right: R,
    sample_num: u64,
}

impl<L: Module + Send, R: Module + Send> Synth<L, R> {
    pub fn new(left: L, right: R) -> Synth<L, R> {
        Synth {
            left,
            right,
            sample_num: std::u64::MAX,
        }
    }

    fn next(&mut self) -> (f32, f32) {
        if self.sample_num == std::u64::MAX {
            self.sample_num = 0;
        } else {
            self.sample_num += 1;
        }
        (
            self.left.next(self.sample_num),
            self.right.next(self.sample_num),
        )
    }
}

pub fn start_stream<L, R, D>(mut synth: Synth<L, R>, device: D) -> impl StreamTrait
where
    L: 'static + Module + Send,
    R: 'static + Module + Send,
    D: DeviceTrait,
{
    let supported_config = device
        .supported_output_configs()
        .expect("error while querying configs")
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let mut right_sample = true;
    let mut left_sample = 0.;

    let stream = device
        .build_output_stream(
            &supported_config.config(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for sample in data {
                    if right_sample {
                        let (next_left_sample, next_right_sample) = synth.next();
                        left_sample = next_left_sample;
                        *sample = next_right_sample;
                        right_sample = false;
                    } else {
                        *sample = left_sample;
                        right_sample = true;
                    }
                }
            },
            move |_| {
                panic!("An error!"); // react to errors here.
            },
        )
        .unwrap();

    stream.play().unwrap();
    stream
}
