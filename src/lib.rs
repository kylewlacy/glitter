#[macro_use] extern crate bitflags;
extern crate gl as gl_lib;

mod context;
mod types;

pub mod gl {
    pub use super::gl_lib::*;
}

pub use context::Context;
pub use types::{Color, BufferBits,
                COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, STENCIL_BUFFER_BIT};
