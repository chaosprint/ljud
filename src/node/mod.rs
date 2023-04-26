use crate::{Buffer, Context};

pub trait Node: Send {
    fn process(&mut self, buffer: &mut Buffer, context: &mut Context);
}

mod add;
mod sin;

pub use add::*;
pub use sin::*;
