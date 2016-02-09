//! Exposes the OpenGL [`Buffer`](struct.Buffer.html) object, and related types.

use std::marker::PhantomData;
use gl;
use gl::types::*;
use types::GLObject;

/// An OpenGL buffer object.
///
/// In OpenGL, a buffer is essentially untyped, and what data it can contain
/// only depends on how it is used. A buffer can be used to store vertices,
/// indices, or essentially any other data that the graphics device needs
/// to process.
///
/// A buffer will automatically be deleted after going out of scope.
///
/// # See also
/// [`gl.gen_buffer`](../context/buffer_context/trait.ContextBufferExt.html#method.gen_buffer) -
/// Create a new, empty buffer.
///
/// [`VertexBuffer`](../vertex_buffer/struct.VertexBuffer.html) and [`IndexBuffer`]
/// (vertex_buffer/struct.IndexBuffer.html): Wrapper types over raw buffers
/// that are more suitable for buffering vertex and index data, respectively.
///
/// [`gl.bind_array_buffer`](../context/buffer_context/trait.ArrayBufferContext.html#method.bind_array_buffer):
/// and [`gl.bind_element_array_buffer`](../context/buffer_context/trait.ElementArrayBufferContext.html#tymethod.split_element_array_buffer):
/// Bind a buffer to a target, returning a buffer binding type.
pub struct Buffer {
    gl_id: GLuint,
    _phantom: PhantomData<*mut ()>
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_id as *const GLuint);
        }
    }
}

impl GLObject for Buffer {
    type Id = GLuint;

    unsafe fn from_raw(id: Self::Id) -> Self {
        Buffer {
            gl_id: id,
            _phantom: PhantomData
        }
    }

    fn id(&self) -> Self::Id {
        self.gl_id
    }
}



gl_enum! {
    /// Proivdes a hint to the OpenGL driver for how a buffer will be used.
    pub gl_enum BufferDataUsage {
        /// Indicates that a buffer will be set once and drawn
        /// only a few times.
        pub const StreamDraw as STREAM_DRAW = gl::STREAM_DRAW,

        /// Indicates that a buffer will be set once and drawn many times
        pub const StaticDraw as STATIC_DRAW = gl::STATIC_DRAW,

        /// Indicates that a buffer will be set and modified repeatedly
        /// and drawn many times.
        pub const DynamicDraw as DYNAMIC_DRAW = gl::DYNAMIC_DRAW
    }
}

gl_enum! {
    /// All of the possible OpenGL targets for binding a buffer object.
    pub gl_enum BufferBindingTarget {
        /// The array buffer object binding.
        pub const ArrayBuffer as ARRAY_BUFFER =
            gl::ARRAY_BUFFER,

        /// The element array buffer object binding.
        pub const ElementArrayBuffer as ELEMENT_ARRAY_BUFFER =
            gl::ELEMENT_ARRAY_BUFFER
    }
}
