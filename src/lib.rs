#![deny(missing_docs)]

//! A type-safe, zero-cost OpenGL abstraction.
//!
//! # Introduction
//! OpenGL is an API designed for 2D and 3D rendering in a high-level,
//! platform-agnostic way. In many situations, it is the *only* viable choice,
//! considering platform, time, and performance requirements.
//!
//! glitter is designed to provide a thin, zero-cost, type-safe wrapper
//! for OpenGL. The API is designed to look and feel familiar to the plain C
//! OpenGL API, so that translating code between glitter and OpenGL can be as
//! painless as possible. Most OpenGL functions have a direct parallel
//! in glitter, such as `glClearColor`, provided as [`gl.clear_color`]
//! (context/trait.ContextExt.html#method.clear_color). Others provide several
//! different wrapper functions to provide better type safety, such as
//! `glDrawElements`, which is wrapped by [`gl.draw_elements`]
//! (context/buffer_context/trait.ContextBufferExt.html#method.draw_elements),
//! [`gl.draw_n_elements`]
//! (context/buffer_context/trait.ContextBufferExt.html#method.draw_n_elements),
//! and [`gl.draw_n_elements_buffered`]
//! (context/buffer_context/trait.ContextBufferExt.html#method.draw_n_elements_buffered).
//!
//! Additionally, some higher-level abstractions are included, usually to
//! reduce boilerplate or to provide a common interface to work across multiple
//! OpenGL versions. A simple example is the [`gl.build_program`]
//! (context/program_context/trait.ContextProgramBuilderExt.html#method.build_program)
//! interface, which makes it quick and easy to create and link a program
//! object, while also correctly checking and reporting any errors.
//!
//! A good starting starting point to dive into glitter is the [`context`]
//! (context/index.html) module. This module is the home of the [`ContextOf`]
//! (context/struct.ContextOf.html) type, which is the main entry point
//! to making OpenGL calls.
//!
//! # OpenGL Version Support
//! Currently, glitter only supports OpenGL ES 2, although the goal is to
//! enable support for targetting any OpenGL version. An example of what this
//! entails is the [`VertexBuffer`](struct.VertexBuffer.html) type. The current
//! implementation of [`VertexBuffer`](struct.VertexBuffer.html) uses OpenGL
//! "vertex buffer objects" under the hood. However, OpenGL also has a
//! complimentary feature, called "vertex array objects", that could replace the
//! current implementation and reduce the number of draw calls. Unfortunately,
//! this API is not available in OpenGL ES 2 (without an extension, that is).
//! In a future version of glitter, the goal is to update [`VertexBuffer`]
//! (struct.VertexBuffer.html) to use vertex array objects, and to fall back
//! to vertex buffer objects when vertex array objects are unavailable.
//!
//! # Thread Safety
//! Eventually, glitter should support proper thread safety using the [`Send`]
//! (https://doc.rust-lang.org/std/marker/trait.Send.html) and [`Sync`]
//! (https://doc.rust-lang.org/std/marker/trait.Sync.html) marker traits.
//! For now, most types have been marked as `!Send` and `!Sync`, meaning that
//! they cannot be sent or shared across threads.
//!
//! # The Future
//! In its current form, glitter should be considered work-in-progress, and
//! the API will likely undergo radical changes before a 1.0 version is
//! establised.

#[macro_use] extern crate bitflags;
extern crate gl;
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

/// Re-exports essential extension traits. Everything exported in this module
/// should be used anywhere that glitter is used.
///
/// Generally, this means that most modules that use glitter will start
/// with the following:
///
/// ```no_run
/// use glitter;
/// use glitter::prelude::*;
/// ```
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
