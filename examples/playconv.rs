use anyhow;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, SizedSample,
};

use ljud::*;

// see the `full` folder for the params
const ELEVATION: &str = "-20";
const AZIMUTH: &str = "270";
// const AZIMUTH: &str = "090";
// const AZIMUTH: &str = "000";
fn main() -> anyhow::Result<()> {
    let host = cpal::default_host();

    let device = host
        .default_output_device()
        .expect("failed to find output device");
    println!("Output device: {}", device.name()?);

    let config = device.default_output_config().unwrap();
    println!("Default output config: {:?}", config);

    match config.sample_format() {
        cpal::SampleFormat::I8 => run::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        // cpal::SampleFormat::I24 => run::<I24>(&device, &config.into()),
        cpal::SampleFormat::I32 => run::<i32>(&device, &config.into()),
        // cpal::SampleFormat::I48 => run::<I48>(&device, &config.into()),
        cpal::SampleFormat::I64 => run::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => run::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
        // cpal::SampleFormat::U24 => run::<U24>(&device, &config.into()),
        cpal::SampleFormat::U32 => run::<u32>(&device, &config.into()),
        // cpal::SampleFormat::U48 => run::<U48>(&device, &config.into()),
        cpal::SampleFormat::U64 => run::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => run::<f64>(&device, &config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

const BLOCK_SIZE: usize = 128;

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate.0;
    let channels = config.channels as usize;
    // let azimuth = "270"; // see the `full` folder
    let mut ctx = Context::new()
        .channels(channels as u8)
        .sample_rate(sample_rate)
        .buffer_size(BLOCK_SIZE as u32)
        .set_graph(svec![(
            "output",
            svec![
                audio_player("sin440.wav").looping(true).boxed(),
                // audio_player("assets/sounds/done.wav").looping(true).boxed(),
                convolution(20, 0).boxed(),
                // convolution([
                //     &format!("assets/mit-hrtf/full/elev{ELEVATION}/L{ELEVATION}e{AZIMUTH}a.wav"),
                //     &format!("assets/mit-hrtf/full/elev{ELEVATION}/R{ELEVATION}e{AZIMUTH}a.wav")
                // ])
                // .boxed()
            ]
        )])
        .init();
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let blocks_needed = data.len() / 2 / BLOCK_SIZE;
            let block_step = channels * BLOCK_SIZE;
            for current_block in 0..blocks_needed {
                let block = ctx.next_block();

                for i in 0..BLOCK_SIZE {
                    for chan in 0..channels {
                        let value: T = T::from_sample(block[chan][i]);
                        data[(i * channels + chan) + (current_block) * block_step] = value;
                    }
                }
            }
        },
        err_fn,
        None,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(30000));

    Ok(())
}
