use hound;
use ljud::*;

const BLOCK_SIZE: usize = 128;
const SAMPLE_RATE: usize = 44100;
const FILENAME: &str = "sin440.wav";
const DURATION_SECS: usize = 5;

fn main() {
    let mut ctx = Context::new()
        .channels(2)
        .sample_rate(SAMPLE_RATE as u32)
        .buffer_size(BLOCK_SIZE as u32)
        .set_graph(svec![("output", svec![sin_osc().freq(440.).boxed()]),]);
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(FILENAME, spec).unwrap();
    for _ in 0..SAMPLE_RATE * DURATION_SECS / BLOCK_SIZE {
        let buf = ctx.next_block();
        for sample_idx in 0..BLOCK_SIZE {
            for channel_idx in 0..2 {
                let sample = buf[channel_idx][sample_idx].clamp(-1.0, 1.0) * (i16::MAX as f32);
                writer
                    .write_sample(sample.round().clamp(i16::MIN as f32, i16::MAX as f32) as i16)
                    .unwrap();
            }
        }
    }
    writer.finalize().unwrap();
}
