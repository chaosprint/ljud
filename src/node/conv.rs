use crate::node::Node;
use crate::{convolve, crossfade, plot_mono, svec, Buffer, Context, SmallVec};
use arrayvec::ArrayVec;
use hound::WavReader;
use std::path::Path;

pub struct Convolution {
    previous_kernel_left: SmallVec<[f32; 1024]>,
    previous_kernel_right: SmallVec<[f32; 1024]>,
    elevation: i32,
    azimuth: u32,
    all_left_convoluted: SmallVec<[SmallVec<[f32; 1024]>; 8]>,
    all_right_convoluted: SmallVec<[SmallVec<[f32; 1024]>; 8]>,
    to_remove: SmallVec<[usize; 8]>,
    need_crossfade: usize,
    crossfade_step: usize,
    step: u32,
    count: u32,
}

impl Convolution {
    // pub fn new<P: AsRef<Path>>(paths: [P; 2]) -> Self {
    pub fn new(elevation: i32, azimuth: u32) -> Self {
        // let mut kernel_left: ArrayVec<f32, 512> = ArrayVec::new();
        // let mut kernel_right: ArrayVec<f32, 512> = ArrayVec::new();

        // plot_mono(&kernel_left);

        // for (i, path) in paths.iter().enumerate() {
        //     let mut reader = WavReader::open(path).expect("Failed to open WAV file");
        //     let spec = reader.spec();
        //     assert_eq!(spec.channels, 1, "Convolution kernel must be mono");

        //     for result in reader.samples::<i16>() {
        //         let sample = result.unwrap() as f32 / i16::MAX as f32;
        //         match i {
        //             0 => kernel_left.push(sample),
        //             1 => kernel_right.push(sample),
        //             _ => panic!("We need two kernels!"),
        //         }
        //     }
        // }

        // let kernel_len = kernel_right.len();
        // println!("Kernel length: {}", kernel_len);
        Self {
            previous_kernel_left: svec![],
            previous_kernel_right: svec![],
            elevation,
            azimuth,
            all_left_convoluted: svec![],
            all_right_convoluted: svec![],
            to_remove: svec![],
            need_crossfade: 0,
            crossfade_step: 0,
            step: 0,
            count: 0,
        }
    }

    pub fn boxed(self) -> Box<dyn Node + Send> {
        Box::new(self)
    }
}

impl Node for Convolution {
    fn process(&mut self, buffer: &mut Buffer, context: &mut Context) {
        let buffer_len = buffer[0].len();

        // let mut need_crossfade = 0;

        if self.azimuth != (self.step / 5) * 5 {
            self.need_crossfade = 10;
        }

        if self.need_crossfade == 0 {
            let left_convoluted = convolve(
                &buffer[0],
                context
                    .ir_lib
                    .get(&(0, self.elevation, self.azimuth))
                    .unwrap(),
            );
            let right_convoluted = convolve(
                &buffer[1],
                context
                    .ir_lib
                    .get(&(1, self.elevation, self.azimuth))
                    .unwrap(),
            );
            self.all_left_convoluted.push(left_convoluted);
            self.all_right_convoluted.push(right_convoluted);
        } else {
            println!(
                "doing crossfade {} {} {}!",
                self.need_crossfade,
                self.azimuth,
                (self.step / 5) * 5
            );
            if self.need_crossfade == 10 {
                println!(
                    "write kernel to self {} {}!",
                    self.azimuth,
                    (self.step / 5) * 5
                );
                self.previous_kernel_left = SmallVec::from_vec(
                    context
                        .ir_lib
                        .get(&(0, self.elevation, self.azimuth))
                        .unwrap()
                        .clone(),
                );
                self.previous_kernel_right = SmallVec::from_vec(
                    context
                        .ir_lib
                        .get(&(1, self.elevation, self.azimuth))
                        .unwrap()
                        .clone(),
                );
            }
            let new_kernel_left = context
                .ir_lib
                .get(&(0, self.elevation, (self.step / 5) * 5))
                .unwrap();
            let new_kernel_right = context
                .ir_lib
                .get(&(1, self.elevation, (self.step / 5) * 5))
                .unwrap();

            self.azimuth = (self.step / 5) * 5;

            let crossfade_kernel_left = crossfade(
                &self.previous_kernel_left,
                &new_kernel_left,
                self.need_crossfade,
            );
            let crossfade_kernel_right = crossfade(
                &self.previous_kernel_right,
                &new_kernel_right,
                self.need_crossfade,
            );

            let left_convoluted = convolve(&buffer[0], &crossfade_kernel_left);
            let right_convoluted = convolve(&buffer[1], &crossfade_kernel_right);

            self.all_left_convoluted.push(left_convoluted);
            self.all_right_convoluted.push(right_convoluted);
            self.need_crossfade -= 1;
        }

        for n in 0..buffer_len {
            buffer[0][n] = 0.0;
            buffer[1][n] = 0.0;

            for (i, convoluted) in self.all_left_convoluted.iter_mut().enumerate() {
                if convoluted.is_empty() {
                    self.to_remove.push(i);
                    continue;
                } else {
                    buffer[0][n] += convoluted.remove(0);
                    buffer[1][n] += self.all_right_convoluted[i].remove(0);
                }
            }
            for i in self.to_remove.iter().rev() {
                self.all_left_convoluted.remove(*i);
                self.all_right_convoluted.remove(*i);
            }
            self.to_remove.clear();
        }
        self.count += 1;
        if self.count % 100 == 0 {
            eprintln!("New step: {}", self.step);
            self.step = (self.step + 28) % 360;
            self.count = 0;
        }
    }
}

// pub fn convolution<P: AsRef<Path>>(paths: [P; 2]) -> Convolution {
//     Convolution::new(paths)
// }

pub fn convolution(elevation: i32, azimuth: u32) -> Convolution {
    Convolution::new(elevation, azimuth)
}
