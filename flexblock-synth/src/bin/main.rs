use cpal::traits::{DeviceTrait, HostTrait};
use flexblock_synth::modules::*;
use flexblock_synth::{start_stream};
use text_io::read;

fn _sfx_fractal(value: (f32, f32), initial_value: (f32, f32)) -> (f32, f32) {
    let initial_value_squared = (
        initial_value.0 * initial_value.0,
        initial_value.1 * initial_value.1,
    );
    let squared_norm = value.0 * value.0 + value.1 * value.1;
    let new_left =
        value.0 * (squared_norm - initial_value_squared.0) + value.1 * initial_value_squared.1;
    let new_right =
        value.1 * (squared_norm - initial_value_squared.0) - value.0 * initial_value_squared.1;
    (new_left, new_right)
}

fn _burning_ship_fractal(value: (f32, f32), initial_value: (f32, f32)) -> (f32, f32) {
    (
        value.0 * value.0 - value.1 * value.1 + initial_value.0,
        2. * (value.0 * value.1).abs() + initial_value.1,
    )
}

fn main() {
    let host = cpal::default_host();
    for device in host.devices().unwrap() {
        println!("{}", device.name().unwrap());
    }
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
    let frequency: f32 = 100.;
    //let mut envelope = Envelope::new(0., 0., 0.8, 0., sample_rate);
    let synth = SineOscillator::new(
        Envelope::new(0., 0., 4., 0., sample_rate) * 4.1235 + frequency,
        sample_rate,
    ) * 4.
        + SawOscillator::new(frequency.into(), sample_rate);
    //let synth = synth * envelope;
    let filter_kernel = lowpass_filter(8000., 128);
    let synth = ConvolutionFilter::new(synth, filter_kernel);
    /*let synth = OnePoleFilter::new(
        synth,
        Envelope::new(0.4, 1., 1.5, 0.0, sample_rate) * 0.65 + 0.3,
    ); */
    let scale = 1.;
    let synth = Synth::new((synth * scale).module(), 0.);

    let stream = start_stream(synth, device);

    let _: String = read!();
    drop(stream);
}
