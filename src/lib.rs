#![cfg_attr(feature = "no_std", no_std)]

#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
pub use alloc::boxed::Box;

use hashbrown::HashMap;
pub use smallvec::smallvec as svec;
use smallvec::{smallvec, SmallVec};

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
}

impl Context {
    pub fn new() -> Self {
        Self {
            sample_rate: 44100,
            channels: 2,
            buffer_size: 128,
            signals: HashMap::new(),
            buffers: HashMap::new(),
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
