//! Contains all of the OpenGL state types related to renderbuffers.

use std::marker::PhantomData;
use std::borrow::BorrowMut;
use gl;
use gl::types::*;
use context::{AContext, BaseContext, ContextOf};
use renderbuffer::{Renderbuffer, RenderbufferTarget};
use image_data::{RenderbufferFormat};
use types::{GLObject, GLError};

/// Provides a safe wrapper for creating renderbuffer objects. A
/// `RenderbufferBuilder` can be created using the [`gl.build_renderbuffer`]
/// (trait.ContextRenderbufferBuilderExt.html#method.build_renderbuffer)
/// method.
pub struct RenderbufferBuilder<C>
    where C: RenderbufferContext
{
    gl: C,
    storage_params: Option<(RenderbufferFormat, u32, u32)>
}

impl<C> RenderbufferBuilder<C>
    where C: RenderbufferContext
{
    fn new(gl: C) -> Self {
        RenderbufferBuilder {
            gl: gl,
            storage_params: None
        }
    }

    /// Set the storage parameters for the renderbuffer.
    pub fn storage(mut self,
                   format: RenderbufferFormat,
                   width: u32,
                   height: u32)
        -> Self
    {
        self.storage_params = Some((format, width, height));
        self
    }

    /// Create and return a renderbuffer with the provided storage options,
    /// or return an error.
    ///
    /// # Failures
    /// An error will be returned if no storage options were provided.
    ///
    /// # Panics
    /// This function will panic if an OpenGL error is generated
    /// and debug assertions are enabled.
    pub fn try_unwrap(self) -> Result<Renderbuffer, GLError> {
        let gl = self.gl;
        let mut rbo = unsafe { gl.gen_renderbuffer() };

        match self.storage_params {
            Some((format, width, height)) => {
                {
                    let (mut gl_rbo, gl) = gl.bind_renderbuffer(&mut rbo);
                    gl.storage(&mut gl_rbo, format, width, height);
                }

                Ok(rbo)
            },
            None => {
                let msg = "Error building renderbuffer: no format or dimensions provided";
                Err(GLError::Message(msg.to_owned()))
            }
        }
    }

    /// Create a renderbuffer with the provided storage options, or panic.
    ///
    /// # Panics
    /// This function will panic if no storage options were provided
    /// or if an OpenGL error was generated and debug assertions are enabled.
    pub fn unwrap(self) -> Renderbuffer {
        self.try_unwrap().unwrap()
    }
}

/// The extension trait for contexts that adds the `build_renderbuffer` method.
/// This trait is only implemented for contexts with a free renderbuffer
/// binding.
pub trait ContextRenderbufferBuilderExt: RenderbufferContext + Sized {
    /// Create a new renderbuffer builder, providing a safe interface
    /// for constructing a renderbuffer object. See the [`RenderbufferBuilder`]
    /// (struct.RenderbufferBuilder.html) docs for more details.
    fn build_renderbuffer(self) -> RenderbufferBuilder<Self> {
        RenderbufferBuilder::new(self)
    }
}

impl<'a, C: 'a> ContextRenderbufferBuilderExt for &'a mut C
    where &'a mut C: RenderbufferContext
{

}

/// An extension trait that includes renderbuffer-related OpenGL methods.
pub trait ContextRenderbufferExt: BaseContext {
    /// Create a new renderbuffer object with no storage allocated.
    ///
    /// # Safety
    /// Many OpenGL function calls expect a renderbuffer to be
    /// [attachment-complete](https://www.opengl.org/wiki/Framebuffer_Object#Attachment_Completeness).
    /// Be sure to properly set up the renderbuffer's storage before passing
    /// the renderbuffer to such functions.
    unsafe fn gen_renderbuffer(&self) -> Renderbuffer {
        let mut id : GLuint = 0;

        gl::GenRenderbuffers(1, &mut id as *mut GLuint);
        dbg_gl_sanity_check! {
            GLError::InvalidValue => "`n` is negative",
            _ => "Unknown error"
        }

        Renderbuffer::from_raw(id)
    }

    /// Initialize a renderbuffer object's storage.
    ///
    /// - `gl_rbo`: The binding of the renderbuffer to set up storage for.
    /// - `format`: The storage format to use for the renderbuffer.
    /// - `width`: The storage width of the renderbuffer, in pixels.
    /// - `height`: The storage height of the renderbuffer, in pixels.
    ///
    /// # See also
    /// [`glRenderbufferStorage`](http://docs.gl/es2/glRenderbufferStorage)
    /// OpenGL docs
    fn storage(&self,
               gl_rbo: &mut RenderbufferBinding,
               format: RenderbufferFormat,
               width: u32,
               height: u32)
    {
        unsafe {
            gl::RenderbufferStorage(gl_rbo.target().gl_enum(),
                                    format.gl_enum(),
                                    width as GLint,
                                    height as GLint);
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_RENDERBUFFER` or `internalformat` is not an accepted format",
                GLError::InvalidValue => "`width` or `height` is less than zero or greater than `GL_MAX_RENDERBUFFER_SIZE`",
                GLError::OutOfMemory => "Unable to allocate enough memory for requested size",
                GLError::InvalidOperation => "Renderbuffer object 0 is bound",
                _ => "Unknown error"
            }
        }
    }
}

impl<C: BaseContext> ContextRenderbufferExt for C {

}



/// An OpenGL context that has a free `GL_RENDERBUFFER` binding.
pub trait RenderbufferContext: AContext {
    /// The type of binder this context contains.
    type Binder: BorrowMut<RenderbufferBinder>;

    /// The OpenGL context that will be returned after binding a renderbuffer.
    type Rest: AContext;

    /// Split the context into a binder and the remaining context.
    fn split_renderbuffer(self) -> (Self::Binder, Self::Rest);

    /// Bind a renderbuffer to this context's renderbuffer,
    /// returning a new context and a binding.
    ///
    /// # See also
    /// [`glBindRenderbuffer`](http://docs.gl/es2/glBindRenderbuffer)
    /// OpenGL docs
    fn bind_renderbuffer<'a>(self, rbo: &'a mut Renderbuffer)
        -> (RenderbufferBinding<'a>, Self::Rest)
        where Self: Sized
    {
        let (mut binder, rest) = self.split_renderbuffer();
        (binder.borrow_mut().bind(rbo), rest)
    }
}

impl<B, F, P, R, T> RenderbufferContext for ContextOf<B, F, P, R, T>
    where R: BorrowMut<RenderbufferBinder>
{
    type Binder = R;
    type Rest = ContextOf<B, F, P, (), T>;

    fn split_renderbuffer(self) -> (Self::Binder, Self::Rest) {
        self.swap_renderbuffer(())
    }
}

impl<'a, B, F, P, R, T> RenderbufferContext
    for &'a mut ContextOf<B, F, P, R, T>
    where R: BorrowMut<RenderbufferBinder>
{
    type Binder = &'a mut RenderbufferBinder;
    type Rest = ContextOf<&'a mut B, &'a mut F, &'a mut P, (), &'a mut T>;

    fn split_renderbuffer(self) -> (Self::Binder, Self::Rest) {
        let gl = self.borrowed_mut();
        gl.swap_renderbuffer(())
    }
}



/// Represents a renderbuffer that has been bound to the `GL_RENDERBUFFER`
/// binding target.
pub struct RenderbufferBinding<'a> {
    _phantom_ref: PhantomData<&'a mut Renderbuffer>,
    _phantom_ptr: PhantomData<*mut ()>
}

impl<'a> RenderbufferBinding<'a> {
    fn target(&self) -> RenderbufferTarget {
        RenderbufferTarget::Renderbuffer
    }
}

/// The OpenGL state representing the `GL_RENDERBUFFER` target.
pub struct RenderbufferBinder {
    _phantom: PhantomData<*mut ()>
}

impl RenderbufferBinder {
    /// Get the current `GL_RENDERBUFFER` binder.
    ///
    /// # Safety
    /// The same rules apply to this function as the
    /// [`ContextOf::current_context`]
    /// (../struct.ContextOf.html#method.current_context) method.
    pub unsafe fn current() -> Self {
        RenderbufferBinder {
            _phantom: PhantomData
        }
    }

    /// Bind a renderbuffer to the `GL_RENDERBUFFER` target, returning
    /// a binding.
    pub fn bind<'a>(&mut self, renderbuffer: &'a mut Renderbuffer)
        -> RenderbufferBinding<'a>
    {
        let binding = RenderbufferBinding {
            _phantom_ref: PhantomData,
            _phantom_ptr: PhantomData
        };
        unsafe {
            gl::BindRenderbuffer(binding.target().gl_enum(),
                                 renderbuffer.id());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_RENDERBUFFER`",
                _ => "Unknown error"
            }
        }
        binding
    }
}
