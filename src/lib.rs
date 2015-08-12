#[macro_use] extern crate bitflags;
extern crate gl as gl_lib;

#[macro_use] mod context;
mod buffer;
mod shader;
mod types;

pub use gl_lib as gl;

pub use context::Context;
pub use buffer::{Buffer, BufferDataUsage, BufferBinding,
                 STREAM_DRAW, STATIC_DRAW, DYNAMIC_DRAW,
                 ArrayBufferBinder, ElementArrayBufferBinder,
                 ArrayBufferBinding, ElementArrayBufferBinding};
pub use shader::Shader;
pub use types::{Color, BufferBits,
                COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, STENCIL_BUFFER_BIT};
