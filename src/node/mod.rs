use crate::{Buffer, Context};

pub trait Node: Send {
    fn process(&mut self, buffer: &mut Buffer, context: &mut Context);
}

mod add;
pub use add::*;
mod sin;
pub use sin::*;

#[cfg(feature = "sampler")]
mod player;
#[cfg(feature = "sampler")]
pub use player::*;
