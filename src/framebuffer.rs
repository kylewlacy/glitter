//! Exposes the OpenGL [`Framebuffer`](struct.Framebuffer.html) object,
//! and related types.

use std::marker::PhantomData;
use gl;
use gl::types::*;
use types::GLObject;

/// An OpenGL framebuffer object.
///
/// A framebuffer can best be thought of as a 'surface' that can be rendered
/// to. A framebuffer will contain one or more attachment points, which
/// point to the actual container where data will really be written when
/// drawing. Generally, a framebuffer will use a [`Renderbuffer`]
/// (../renderbuffer/struct.Renderbuffer.html) to draw a final image to
/// the screen, and a [`Texture2d`](../texture/type.Texture2d.html) when
/// some post-processing effects need to be applied before displaying the
/// final image.
///
/// A framebuffer will automatically be deleted after going out of scope.
///
/// # See also
/// [`gl.build_framebuffer`](../context/framebuffer_context/trait.ContextFramebufferBuilderExt.html#method.build_framebuffer):
/// Build a new framebuffer, using the [`FramebufferBuilder`](../context/framebuffer_context/struct.FramebufferBuilder.html)
/// type to set attachment points.
///
/// [`gl.gen_framebuffer`](../context/framebuffer_context/trait.ContextFramebufferExt.html#method.gen_framebuffer):
/// Create a new framebuffer with no attachment points.
///
/// [`gl.bind_framebuffer`](../context/framebuffer_context/trait.FramebufferContext.html#method.bind_framebuffer):
/// Bind a framebuffer to a context, returning a [`FramebufferBinding`]
/// (../context/framebuffer_context/struct.FramebufferBinding.html) type.
pub struct Framebuffer {
    gl_id: GLuint,
    _phantom: PhantomData<*mut ()>
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.gl_id as *const GLuint);
        }
    }
}

impl GLObject for Framebuffer {
    type Id = GLuint;

    unsafe fn from_raw(id: Self::Id) -> Self {
        Framebuffer {
            gl_id: id,
            _phantom: PhantomData
        }
    }

    fn id(&self) -> Self::Id {
        self.gl_id
    }
}
