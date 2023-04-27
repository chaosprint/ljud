use crate::node::Node;
use crate::{svec, Buffer, Context};
// use gnuplot::*;
use hound::WavReader;
use smallvec::SmallVec;
// use std::fs::File;
// use std::io::{BufReader, Read};
use std::path::Path;

pub struct AudioPlayer {
    data: SmallVec<[SmallVec<[f32; 131072]>; 2]>,
    position: usize,
    looping: bool,
}

impl AudioPlayer {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let mut reader = WavReader::open(path).expect("Failed to open WAV file");
        let spec = reader.spec();
        let num_channels = spec.channels as usize;
        let mut data: SmallVec<[SmallVec<[f32; 131072]>; 2]> = svec![];

        for _ in 0..num_channels {
            data.push(SmallVec::new());
        }

        let mut sample_count = 0;
        for result in reader.samples::<i16>() {
            let sample = result.unwrap() as f32 / i16::MAX as f32;
            let channel = sample_count % num_channels;
            data[channel].push(sample);
            sample_count += 1;
        }

        // let mut x = (0..data[0].len()).collect::<Vec<_>>();

        // let mut fg = Figure::new();
        // fg.axes2d()
        //     .set_title("Glicol output", &[])
        //     .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        //     .lines(&x, &data[0], &[Caption("left")]);
        // fg.show().unwrap();

        Self {
            data,
            position: 0,
            looping: false,
        }
    }
    pub fn looping(mut self, looping: bool) -> Self {
        self.looping = looping;
        self
    }
    pub fn boxed(self) -> Box<dyn Node + Send> {
        Box::new(self)
    }
}

impl Node for AudioPlayer {
    fn process(&mut self, buffer: &mut Buffer, _context: &mut Context) {
        for (channel, buffer_channel) in self.data.iter().zip(buffer.iter_mut()) {
            for (sample_idx, buffer_sample) in buffer_channel.iter_mut().enumerate() {
                let position = self.position + sample_idx;
                if self.looping {
                    *buffer_sample = channel[position % channel.len()];
                } else {
                    if position < channel.len() {
                        *buffer_sample = channel[position];
                    } else {
                        *buffer_sample = 0.0;
                    }
                }
            }
        }
        self.position += buffer[0].len();
        if self.looping && self.position >= self.data[0].len() {
            self.position %= self.data[0].len();
        }
    }

    // fn process(&mut self, buffer: &mut Buffer, _context: &mut Context) {
    // for (channel, buffer_channel) in self.data.iter().zip(buffer.iter_mut()) {
    //     for (sample, buffer_sample) in channel[self.position..]
    //         .iter()
    //         .zip(buffer_channel.iter_mut())
    //     {
    //         *buffer_sample = *sample;
    //     }
    // }
    // self.position += buffer[0].len();
    // if self.position >= self.data[0].len() {
    //     self.position = 0;
    // }

    // }
}

pub fn audio_player<P: AsRef<Path>>(path: P) -> AudioPlayer {
    AudioPlayer::new(path)
}
