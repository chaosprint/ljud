#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
pub use alloc::boxed::Box;

use hashbrown::HashMap;
use hound::WavReader;
pub use smallvec::smallvec as svec;
use smallvec::{smallvec, SmallVec};
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use walkdir::{DirEntry, WalkDir};

pub mod node;
pub use node::*;

pub mod param;
pub use param::*;

pub mod utils;
pub use utils::*;

pub type Signal = SmallVec<[Box<dyn Node + Send>; 8]>; //  SmallVec<[Box<dyn Node + Send>; 8]>;
pub type Buffer = SmallVec<[SmallVec<[f32; 1024]>; 2]>; // limit to 2 channels

pub struct Context {
    pub sample_rate: u32,
    pub channels: u8,
    pub buffer_size: u32,
    pub signals: HashMap<&'static str, Signal>,
    pub buffers: HashMap<&'static str, Buffer>,
    pub ir_lib: HashMap<(u8, i32, u32), Vec<f32>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer_size: 128,
            signals: HashMap::new(),
            buffers: HashMap::new(),
            ir_lib: HashMap::new(),
        }
    }
    pub fn channels(mut self, channels: u8) -> Self {
        self.channels = channels;
        self
    }
    pub fn sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate;
        self
    }
    pub fn buffer_size(mut self, buffer_size: u32) -> Self {
        self.buffer_size = buffer_size;
        self
    }

    pub fn set_graph(
        mut self,
        graph: SmallVec<[(&'static str, SmallVec<[Box<dyn Node + Send>; 8]>); 8]>,
    ) -> Self {
        for (name, nodes) in graph {
            self.signals.insert(name, nodes);
            self.buffers.insert(
                name,
                smallvec![smallvec![0.0; self.buffer_size as usize]; self.channels as usize],
            );
        }
        self
    }

    pub fn init(mut self) -> Self {
        for entry in WalkDir::new("./assets/mit-hrtf/")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                !e.file_type().is_dir()
                    && e.path().extension().unwrap_or_default() == "wav"
                    && !e.path().to_str().unwrap().contains("headphones")
            })
        {
            let mut kernel = vec![];
            let path = entry.path().to_owned();
            // let mut file = File::open(&path).await.unwrap();
            // println!("loading {:?}", path);

            let mut reader = WavReader::open(path).expect("Failed to open WAV file");
            let spec = reader.spec();
            assert_eq!(spec.channels, 1, "Convolution kernel must be mono");

            for result in reader.samples::<i16>() {
                let sample = result.unwrap() as f32 / i16::MAX as f32;
                kernel.push(sample);
            }
            let mut name = entry.file_name().to_str().unwrap().split('e');
            let channel_and_elevation = name.next().unwrap();
            let azimuth = name.next().unwrap().split("a").next().unwrap();

            // println!("channel_and_elevation {}", channel_and_elevation);
            // println!("azimuth {}", azimuth);

            let elevation = channel_and_elevation[1..].parse::<i32>().unwrap();

            let key = match &channel_and_elevation[0..1] {
                "L" => (0_u8, elevation, azimuth.parse::<u32>().unwrap()),
                "R" => (1_u8, elevation, azimuth.parse::<u32>().unwrap()),
                _ => panic!("invalid channel"),
            };
            // println!("key {:?}", &key);
            self.ir_lib.insert(key, kernel);

            // let mut buffer = Vec::new();
            // file.read_to_end(&mut buffer).await.unwrap();
        }
        self
    }

    pub fn next_block(&mut self) -> &mut Buffer {
        let ctx_ptr = self as *mut Context;

        for (name, signal) in self.signals.iter_mut() {
            let buffer = self.buffers.get_mut(name).unwrap();
            for node in signal.iter_mut() {
                unsafe { node.process(buffer, &mut *ctx_ptr) };
            }
        }
        let buffer = self.buffers.get_mut("output").unwrap();
        buffer
    }
}
