#[macro_use] extern crate bitflags;
extern crate gl as gl_lib;
#[cfg(feature = "cgmath")] extern crate cgmath;
#[cfg(feature = "image")] extern crate image;

mod to_ref;

#[macro_use] mod macros;
pub mod context;
pub mod buffer;
pub mod shader;
pub mod program;
pub mod framebuffer;
pub mod renderbuffer;
pub mod texture;
pub mod image_data;
pub mod vertex_data;
pub mod vertex_buffer;
pub mod index_data;
pub mod uniform_data;
pub mod types;

#[cfg(feature = "cgmath")] mod cgmath_features;
#[cfg(feature = "image")] mod image_features;



pub use gl_lib as gl;

pub use context::*;
pub use buffer::*;
pub use shader::*;
pub use program::*;
pub use framebuffer::*;
pub use renderbuffer::*;
pub use texture::*;
pub use image_data::*;
pub use vertex_data::*;
pub use vertex_buffer::*;
pub use index_data::*;
pub use uniform_data::*;
pub use types::*;

pub mod prelude {
    pub use context::{AContext, BufferContext,
                      ArrayBufferContext, ElementArrayBufferContext,
                      FramebufferContext, ContextFramebufferBuilderExt,
                      ProgramContext, ContextProgramBuilderExt,
                      RenderbufferContext, ContextRenderbufferBuilderExt,
                      TextureBinding, ContextTextureBuilderExt,
                      TextureUnit, TextureUnitBinding, ATextureUnitBinding,
                      TextureUnitBinding2d, TextureUnitBindingCubeMap,
                      TextureUnit0Context, TextureUnit1Context,
                      TextureUnit2Context, TextureUnit3Context,
                      TextureUnit4Context, TextureUnit5Context,
                      TextureUnit6Context, TextureUnit7Context};
    pub use context::ext::*;
    pub use shader::ContextShaderBuilderExt;
    pub use vertex_buffer::{VertexBufferContext, IndexBufferContext,
                            ContextVertexBufferExt};
    pub use types::GLObject;
}
