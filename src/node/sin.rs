#[cfg(feature = "no_std")]
use core::f32::consts::PI;

#[cfg(feature = "no_std")]
extern crate libm;

#[cfg(not(feature = "no_std"))]
use std::f32::consts::PI;

#[cfg(feature = "no_std")]
use crate::Box;

use crate::*;

pub struct SinOsc {
    freq: ParamType,
    phase: f32,
    amp: f32,
}

impl SinOsc {
    pub fn new() -> Self {
        Self {
            freq: ParamType::Float(440.0),
            phase: 0.0,
            amp: 1.0,
        }
    }

    pub fn freq(mut self, freq: impl Param + Send) -> Self {
        self.freq = freq.as_param();
        self
    }

    pub fn amp(mut self, amp: f32) -> Self {
        self.amp = amp;
        self
    }

    pub fn phase(mut self, phase: f32) -> Self {
        self.phase = phase;
        self
    }

    pub fn boxed(self) -> Box<dyn Node + Send> {
        Box::new(self)
    }

    fn process_chain(&mut self, buffer: &mut Buffer, freq: f32, sr: u32) {
        let chan = buffer.len();
        let frames = buffer[0].len();
        for frame in 0..frames {
            // let sample = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.amp;
            let sample = {
                #[cfg(feature = "no_std")]
                {
                    libm::sinf(2.0 * PI * self.phase) * self.amp
                }
                #[cfg(not(feature = "no_std"))]
                {
                    (2.0 * PI * self.phase as f32).sin() * self.amp
                }
            };
            for channel in 0..chan {
                buffer[channel][frame] = sample;
            }
            self.phase += freq / sr as f32;
        }
    }

    fn process_sidechain(&mut self, buffer: &mut Buffer, sidechain: &mut Buffer, sr: u32) {
        let chan = buffer.len();
        let frames = buffer[0].len();
        for frame in 0..frames {
            let sample = {
                #[cfg(feature = "no_std")]
                {
                    libm::sinf(2.0 * PI * self.phase) * self.amp
                }
                #[cfg(not(feature = "no_std"))]
                {
                    (self.phase * 2.0 * PI).sin() * self.amp
                }
            };
            // let sample = (self.phase * 2.0 * PI).sin() * self.amp;
            for channel in 0..chan {
                buffer[channel][frame] = sample;
            }
            self.phase += sidechain[0][frame] / sr as f32;
        }
    }
}

impl Node for SinOsc {
    fn process(&mut self, buffer: &mut Buffer, context: &mut Context) {
        let sr = context.sample_rate;
        match self.freq {
            ParamType::Float(val) => {
                self.process_chain(buffer, val, sr);
            }
            ParamType::Str(s) => {
                let sidechain_buffer = context.buffers.get_mut(s).unwrap();
                self.process_sidechain(buffer, sidechain_buffer, sr);
            }
        }
    }
}

pub fn sin_osc() -> SinOsc {
    SinOsc::new()
}
