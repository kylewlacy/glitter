//! Exposes the OpenGL [`Renderbuffer`](struct.Renderbuffer.html) object,
//! and related types.

use std::marker::PhantomData;
use gl;
use gl::types::*;
use types::GLObject;

/// An OpenGL renderbuffer object.
///
/// A renderbuffer is essentially a 2D image that's optimized to be
/// a render target. Unlike a [`Texture2d`](../texture/type.Texture2d.html),
/// a renderbuffer is always created with uninitialized data. The only way
/// to 'send' an image to a renderbuffer is to render to it by attaching it
/// to a [`Framebuffer`](../framebuffer/struct.Framebuffer.html). A renderbuffer
/// is most appropriate when rendering a final output image, where no extra
/// postprocessing is required.
///
/// A renderbuffer will automatically be deleted after going out of scope.
///
/// # See also
/// [`gl.build_renderbuffer`](../context/renderbuffer_context/trait.ContextRenderbufferBuilderExt.html#method.build_renderbuffer):
/// Build a new renderbuffer, using the [`RenderbufferBuilder`](../context/renderbuffer_context/struct.RenderbufferBuilder.html)
/// type to setup storage options.
///
/// [`gl.gen_framebuffer`](../context/framebuffer_context/trait.ContextFramebufferExt.html#method.gen_framebuffer):
/// Create a new framebuffer with no attachment points.
///
/// [`gl.bind_renderbuffer`](../context/renderbuffer_context/trait.RenderbufferContext.html#method.bind_renderbuffer):
/// Bind a renderbuffer to a context, returning a [`RenderbufferBinding`]
/// (../context/renderbuffer_context/struct.RenderbufferBinding.html) type
pub struct Renderbuffer {
    gl_id: GLuint,
    _phantom: PhantomData<*mut ()>
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.gl_id as *const GLuint);
        }
    }
}

impl GLObject for Renderbuffer {
    type Id = GLuint;

    unsafe fn from_raw(id: Self::Id) -> Self {
        Renderbuffer {
            gl_id: id,
            _phantom: PhantomData
        }
    }

    fn id(&self) -> Self::Id {
        self.gl_id
    }
}



gl_enum! {
    /// All of the possible OpenGL targets for binding renderbuffer objects.
    pub gl_enum RenderbufferTarget {
        /// The lone renderbuffer target.
        pub const Renderbuffer as RENDERBUFFER = gl::RENDERBUFFER
    }
}
