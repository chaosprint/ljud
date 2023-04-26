use crate::node::Node;
use crate::param::*;
use crate::{Buffer, Context};

#[cfg(feature = "no_std")]
use crate::Box;

pub struct Add {
    val: ParamType,
}

impl Add {
    pub fn new(val: impl Param + Send) -> Self {
        Self {
            val: val.as_param(),
        }
    }

    fn process_chain(&mut self, buffer: &mut Buffer, val: f32) {
        for channel in buffer.iter_mut() {
            for sample in channel.iter_mut() {
                *sample += val;
            }
        }
    }

    fn process_sidechain(&mut self, buffer: &mut Buffer, sidechain: &mut Buffer) {
        for (channel, sidechain_channel) in buffer.iter_mut().zip(sidechain.iter()) {
            for (sample, sidechain_sample) in channel.iter_mut().zip(sidechain_channel.iter()) {
                *sample += *sidechain_sample;
            }
        }
    }
}

impl Node for Add {
    fn process(&mut self, buffer: &mut Buffer, context: &mut Context) {
        match self.val {
            ParamType::Float(val) => {
                self.process_chain(buffer, val);
            }
            ParamType::Str(s) => {
                let sidechain_buffer = context.buffers.get_mut(s).unwrap();
                self.process_sidechain(buffer, sidechain_buffer);
            }
        }
    }
}

pub fn add(val: impl Param) -> Box<dyn Node + Send> {
    Box::new(Add::new(val))
}
