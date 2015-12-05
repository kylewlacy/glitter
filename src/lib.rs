#[macro_use] extern crate bitflags;
extern crate gl as gl_lib;
#[cfg(feature = "cgmath")] extern crate cgmath as cgmath_lib;
#[cfg(feature = "image")] extern crate image as image_lib;

mod ref_into;

#[macro_use] mod macros;
mod context;
mod buffer;
mod shader;
mod program;
mod framebuffer;
mod renderbuffer;
mod texture;
mod texture_units;
mod image_data;
mod vertex_data;
mod vertex_buffer;
mod index_data;
mod uniform_data;
mod types;

#[cfg(feature = "cgmath")] mod cgmath_features;
#[cfg(feature = "image")] mod image_features;



pub use gl_lib as gl;
#[cfg(feature = "cgmath")] pub use cgmath_lib as cgmath;
#[cfg(feature = "image")] pub use image_lib as image;

pub use context::*;
pub use buffer::*;
pub use shader::*;
pub use program::*;
pub use framebuffer::*;
pub use renderbuffer::*;
pub use texture::*;
pub use texture_units::*;
pub use image_data::*;
pub use vertex_data::*;
pub use vertex_buffer::*;
pub use index_data::*;
pub use uniform_data::*;
pub use types::*;

pub mod prelude {
    pub use buffer::BufferBinding;
    pub use texture_units::TextureUnit;
    pub use texture::TextureBinding;
}
