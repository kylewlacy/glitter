#![feature(raw)]

#[macro_use] extern crate bitflags;
extern crate gl as gl_lib;

#[macro_use] mod context;
mod buffer;
mod shader;
mod program;
mod vertex_data;
mod vertex_buffer;
mod types;

pub use gl_lib as gl;

pub use context::Context;
pub use buffer::{Buffer, BufferBinding, BufferDataUsage,
                 STREAM_DRAW, STATIC_DRAW, DYNAMIC_DRAW,
                 ArrayBufferBinder, ElementArrayBufferBinder,
                 ArrayBufferBinding, ElementArrayBufferBinding};
pub use shader::{Shader, ShaderType, VERTEX_SHADER, FRAGMENT_SHADER};
pub use program::{Program, ProgramAttrib};
pub use vertex_data::{VertexData, VertexDatum, VertexBytes, VertexAttribBinder,
                      DataType, BYTE, UNSIGNED_BYTE, SHORT, UNSIGNED_SHORT,
                      FIXED, FLOAT};
pub use vertex_buffer::VertexBuffer;
pub use types::{Color, GLError, BufferBits,
                COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, STENCIL_BUFFER_BIT};
