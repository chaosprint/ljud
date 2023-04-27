use crate::node::Node;
use crate::{svec, Buffer, Context, SmallVec};
use hound::WavReader;
use std::path::Path;
use std::vec::Vec;

pub struct Convolution {
    kernels: [SmallVec<[f32; 131072]>; 2],
    position: usize,
    internal_storage: [Vec<Vec<f32>>; 2],
}

impl Convolution {
    pub fn new<P: AsRef<Path>>(paths: [P; 2]) -> Self {
        let mut kernels = [svec![], svec![]];

        for (i, path) in paths.iter().enumerate() {
            let mut reader = WavReader::open(path).expect("Failed to open WAV file");
            let spec = reader.spec();
            assert_eq!(spec.channels, 1, "Convolution kernel must be mono");

            for result in reader.samples::<i16>() {
                let sample = result.unwrap() as f32 / i16::MAX as f32;
                kernels[i].push(sample);
            }
        }

        let kernel_len = kernels[0].len();
        let internal_storage = [
            vec![vec![0.0; kernel_len]; kernel_len],
            vec![vec![0.0; kernel_len]; kernel_len],
        ];

        Self {
            kernels,
            position: 0,
            internal_storage,
        }
    }

    pub fn boxed(self) -> Box<dyn Node + Send> {
        Box::new(self)
    }
}

impl Node for Convolution {
    fn process(&mut self, buffer: &mut Buffer, _context: &mut Context) {
        let kernel_len = self.kernels[0].len();
        let buffer_len = buffer[0].len();

        for (i, buffer_channel) in buffer.iter_mut().enumerate() {
            for buffer_sample_idx in 0..buffer_len {
                let input_sample = buffer_channel[buffer_sample_idx];
                for kernel_idx in 0..kernel_len {
                    self.internal_storage[i][kernel_idx]
                        .push(input_sample * self.kernels[i][kernel_idx]);
                }
            }
        }

        for (i, buffer_channel) in buffer.iter_mut().enumerate() {
            for buffer_sample_idx in 0..buffer_len {
                let mut sum = 0.0;
                for kernel_idx in 0..kernel_len {
                    let position = self.position + buffer_sample_idx + kernel_idx;
                    if position < self.internal_storage[i][kernel_idx].len() {
                        sum += self.internal_storage[i][kernel_idx][position];
                    }
                }
                buffer_channel[buffer_sample_idx] = sum;
            }
        }

        self.position += buffer_len;
    }
}

pub fn convolution<P: AsRef<Path>>(paths: [P; 2]) -> Convolution {
    Convolution::new(paths)
}
