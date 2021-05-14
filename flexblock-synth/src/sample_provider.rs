use cpal::traits::{DeviceTrait, StreamTrait};

pub trait SampleProvider {
    fn next(&mut self, samples: &mut[f32]);
}

pub trait MonoSampleProvider {
    fn next(&mut self, samples: &mut[f32]);
}

pub fn start_stream<S, D>(mut sample_provider: S, device: D) -> impl StreamTrait
where
    S: SampleProvider + Send + 'static,
    D: DeviceTrait,
{
    let supported_config = device
        .supported_output_configs()
        .expect("error while querying configs")
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let stream = device
        .build_output_stream(
            &supported_config.config(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                sample_provider.next(data);
            },
            move |_| {
                panic!("An error!"); // React to errors here.
            },
        )
        .unwrap();
    stream.play().unwrap();
    stream
}
