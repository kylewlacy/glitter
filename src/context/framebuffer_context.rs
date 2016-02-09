//! Contains all of the OpenGL state types related to framebuffer objects.

use std::marker::PhantomData;
use std::collections::hash_map::{HashMap, Entry};
use std::borrow::BorrowMut;
use gl;
use gl::types::*;
use context::{AContext, BaseContext, ContextOf};
use framebuffer::Framebuffer;
use renderbuffer::{Renderbuffer, RenderbufferTarget};
use texture::{Texture, TextureType, ImageTargetType,
              Texture2d, Tx2dImageTarget};
use types::{BufferBits, GLError, GLObject, GLFramebufferError};

/// An extension trait that includes framebuffer-related OpenGL methods.
pub trait ContextFramebufferExt: BaseContext {
    /// Create a new framebuffer object with no attachments.
    ///
    /// # Safety
    /// Most OpenGL function calls assume a framebuffer object will be
    /// [framebuffer-complete](https://www.opengl.org/wiki/Framebuffer_Object#Framebuffer_Completeness).
    /// Since the new framebuffer will have no attachments, it will not be
    /// framebuffer-complete. Violating this invariant is considered undefined
    /// behavior in glitter.
    ///
    /// # See also
    /// [`gl.build_framebuffer`](trait.ContextFramebufferBuilderExt.html#method.build_framebuffer):
    /// A safe wrapper for building a framebuffer, which ensures
    /// framebuffer-completeness.
    ///
    /// [`gl.check_framebuffer_status`](trait.ContextFramebufferExt.html#method.check_framebuffer_status):
    /// Checks to see if a framebuffer object is framebuffer-complete.
    ///
    /// [`glGenFramebuffers`](http://docs.gl/es2/glGenFramebuffers) OpenGL docs
    unsafe fn gen_framebuffer(&self) -> Framebuffer {
        let mut id : GLuint = 0;

        gl::GenFramebuffers(1, &mut id as *mut GLuint);
        dbg_gl_sanity_check! {
            GLError::InvalidValue => "`n` is negative",
            _ => "Unknown error"
        }

        Framebuffer::from_raw(id)
    }

    /// Returns the status of any framebuffer-completeness errors of a
    /// currently-bound framebuffer object. Returns `None` if the framebuffer
    /// is framebuffer-complete.
    ///
    /// # See also
    /// [`glCheckFramebufferStatus`](http://docs.gl/es2/glCheckFramebufferStatus) OpenGL docs
    fn check_framebuffer_status(&self, gl_fbo: &FramebufferBinding)
        -> Option<GLFramebufferError>
    {
        unsafe {
            match gl::CheckFramebufferStatus(gl_fbo.target().gl_enum()) {
                gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                    Some(GLFramebufferError::IncompleteAttachment)
                },
                // gl::FRAMEBUFFER_INCOMPLETE_DIMENSIONS => {
                //     Some(GLFramebufferError::IncompleteDimensions)
                // },
                gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                    Some(GLFramebufferError::IncompleteMissingAttachment)
                },
                gl::FRAMEBUFFER_UNSUPPORTED => {
                    Some(GLFramebufferError::Unsupported)
                },
                _ => { None }
            }
        }
    }

    /// Attach a renderbuffer object to a framebuffer object's attachment
    /// point.
    ///
    /// - `gl_fbo`: The binding of the framebuffer to attach to.
    /// - `attachment`: Which attachment point of the framebuffer to attach to.
    /// - `renderbuffer`: The renderbuffer to attach.
    ///
    /// # See also
    /// [`glFramebufferRenderbuffer`](http://docs.gl/gl4/glFramebufferRenderbuffer) OpenGL docs
    fn framebuffer_renderbuffer(&self,
                                gl_fbo: &mut FramebufferBinding,
                                attachment: FramebufferAttachment,
                                renderbuffer: &mut Renderbuffer)
    {
        // TODO: Should `renderbuffer_target` be an argument?
        let renderbuffer_target = RenderbufferTarget::Renderbuffer;

        unsafe {
            gl::FramebufferRenderbuffer(gl_fbo.target().gl_enum(),
                                        attachment.gl_enum(),
                                        renderbuffer_target.gl_enum(),
                                        renderbuffer.id());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_FRAMEBUFFER`, `attachment` is not a valid attachment point, or `renderbuffer` is not `GL_RENDERBUFFER` and `renderbuffer` is not 0",
                GLError::InvalidOperation => "Framebuffer 0 is bound, or `renderbuffer` is neither 0 nor the name of an existing renderbuffer object",
                _ => "Unknown error"
            }
        }
    }

    /// Attach a texture to a framebuffer object's attachment point.
    ///
    /// - `gl_fbo`: The binding of the framebuffer to attach to.
    /// - `attachment`: Which attachment point of the framebuffer to attach to.
    /// - `tex_target`: The 2D 'face' of the texture to attach.
    /// - `texture`: The texture to attach.
    /// - `level`: The mipmap level of the texture to attach. **Note that this
    ///            value must be 0**.
    ///
    /// # Panics
    /// This function will panic with a debug assertion if `level` is not 0.
    ///
    /// # See also
    /// [`glFramebufferTexture2D`](http://docs.gl/es2/glFramebufferTexture2D) OpenGL docs
    fn framebuffer_texture_2d<I, T>(&self,
                                    gl_fbo: &mut FramebufferBinding,
                                    attachment: FramebufferAttachment,
                                    tex_target: I,
                                    texture: &mut Texture<T>,
                                    level: i32)
        where I: Into<T::ImageTargetType>,
              T: TextureType,
    {
        debug_assert!(level == 0);

        unsafe {
            gl::FramebufferTexture2D(gl_fbo.target().gl_enum(),
                                     attachment.gl_enum(),
                                     tex_target.into().gl_enum(),
                                     texture.id(),
                                     level as GLint);
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_FRAMEBUFFER`, `attachment` is not an accepted attachment point, or `textarget` is not an accepted texture target and texture is not 0",
                GLError::InvalidValue => "`level` is not 0 and `texture` is not 0",
                GLError::InvalidOperation => "Framebuffer object 0 is bound, `texture` is neither 0 nor the name of an existing texture object, or `textarget` is not a valid target for `texture`",
                _ => "Unknown error"
            }
        }
    }

    // TODO: Think about this function signature harder (and all draw calls).
    // Should this require a &mut FramebufferBinding, to prevent a
    // no-op glClear(), and for (future) multi-threaded safety?
    //
    /// Clear the currently-bound drawing buffers that are specified by
    /// the `buffers` argument.
    ///
    /// # See also
    /// [`glClear`](http://docs.gl/es2/glClear) OpenGL docs
    fn clear(&self, buffers: BufferBits) {
        unsafe {
            gl::Clear(buffers.bits());
            dbg_gl_sanity_check! {
                GLError::InvalidValue => "`mask` includes a bit other than an allowed value",
                _ => "Unkown error"
            }
        }
    }
}

impl<C: BaseContext> ContextFramebufferExt for C {

}



enum BuilderAttachment<'a> {
    Texture2d(&'a mut Texture2d, i32),
    Renderbuffer(&'a mut Renderbuffer)
}

/// Provides a safe interface for building a framebuffer object that
/// is checked to be framebuffer-complete. A `FramebufferBuilder` can
/// be created using the [`gl.build_framebuffer`]
/// (trait.ContextFramebufferBuilderExt.html#method.build_framebuffer) method.
///
/// # Note
/// The current implementation of `FramebufferBuilder` uses heap-allocation
/// during construction. This should be fixed in a future release, but
/// consider falling back to the more direct [`gl.gen_framebuffer`]
/// (trait.ContextFramebufferExt.html#method.gen_framebuffer) if this ends
/// up being problematic.
pub struct FramebufferBuilder<'a, C>
    where C: FramebufferContext
{
    gl: C,
    attachments: HashMap<FramebufferAttachment, BuilderAttachment<'a>>
}

impl<'a, C> FramebufferBuilder<'a, C>
    where C: FramebufferContext
{
    fn new(gl: C) -> Self {
        FramebufferBuilder {
            gl: gl,
            attachments: HashMap::new()
        }
    }

    /// Add a 2D texture (at the mipmap level specified by `level`) to the
    /// framebuffer's attachment point.
    ///
    /// # Failures
    /// `level` should be 0, or unwrapping the framebuffer will fail.
    ///
    /// # Note
    /// Currently, only [`Texture2d`](../../texture/type.Texture2d.html)
    /// textures are supported using a `FramebufferBuilder`. To bind a different
    /// type of texture, use [`gl.framebuffer_texture_2d`](trait.ContextFramebufferExt.html#method.framebuffer_texture_2d)
    /// on an existing framebuffer object instead (generated either with
    /// a `FramebufferBuilder` or with [`gl.gen_framebuffer`](trait.ContextFramebufferExt.html#method.gen_framebuffer)).
    pub fn texture_2d(mut self,
                      attachment: FramebufferAttachment,
                      texture: &'a mut Texture2d,
                      level: i32)
        -> Self
    {
        let attached = BuilderAttachment::Texture2d(texture, level);
        match self.attachments.entry(attachment) {
            Entry::Occupied(mut e) => { e.insert(attached); },
            Entry::Vacant(e) => { e.insert(attached); }
        };

        self
    }

    /// Add a renderbuffer to the framebuffer's attachment point.
    pub fn renderbuffer(mut self,
                        attachment: FramebufferAttachment,
                        renderbuffer: &'a mut Renderbuffer)
        -> Self
    {
        let attached = BuilderAttachment::Renderbuffer(renderbuffer);
        match self.attachments.entry(attachment) {
            Entry::Occupied(mut e) => { e.insert(attached); },
            Entry::Vacant(e) => { e.insert(attached); }
        };

        self
    }

    /// Create and return a framebuffer with the specified options, or
    /// return an error.
    ///
    /// # Failures
    /// If the resulting framebuffer is not framebuffer-complete, an error
    /// will be returned.
    ///
    /// # Panics
    /// This function will panic if an OpenGL error was generated with
    /// debug assertions enabled.
    pub fn try_unwrap(self) -> Result<Framebuffer, GLError> {
        let gl = self.gl;
        let mut fbo = unsafe { gl.gen_framebuffer() };
        let fbo_status = {
            let (mut gl_fbo, gl) = gl.bind_framebuffer(&mut fbo);

            for (attachment, attached) in self.attachments.into_iter() {
                match attached {
                    BuilderAttachment::Texture2d(texture, level) => {
                        gl.framebuffer_texture_2d(&mut gl_fbo,
                                                  attachment,
                                                  Tx2dImageTarget::Texture2d,
                                                  texture,
                                                  level);
                    },
                    BuilderAttachment::Renderbuffer(renderbuffer) => {
                        gl.framebuffer_renderbuffer(&mut gl_fbo,
                                                    attachment,
                                                    renderbuffer);
                    }
                }
            }

            gl.check_framebuffer_status(&mut gl_fbo)
        };

        match fbo_status {
            Some(err) => { Err(err.into()) },
            None => { Ok(fbo) }
        }
    }

    /// Create and return a framebuffer with the specified options, or panic.
    ///
    /// # Panics
    /// This function will panic if the resulting framebuffer is not
    /// framebuffer-complete or an OpenGL error was generated with debug
    /// assertions enabled.
    pub fn unwrap(self) -> Framebuffer {
        self.try_unwrap().unwrap()
    }
}

/// The extension trait for contexts that adds the `build_framebuffer` method.
/// This trait is only implemented for contexts that have a free framebuffer
/// binding.
pub trait ContextFramebufferBuilderExt: FramebufferContext + Sized {
    /// Create a new framebuffer builder, providing a safe interface
    /// for constructing a framebuffer object. See the [`FramebufferBuilder`]
    /// (struct.FramebufferBuilder.html) docs for more details.
    fn build_framebuffer<'a>(self) -> FramebufferBuilder<'a, Self> {
        FramebufferBuilder::new(self)
    }
}

impl<'b, C: 'b> ContextFramebufferBuilderExt for &'b mut C
    where &'b mut C: FramebufferContext
{

}



gl_enum! {
    /// All of the possible OpenGL targets for binding
    /// framebuffer objects.
    pub gl_enum FramebufferTarget {
        /// The lone framebuffer target.
        pub const Framebuffer as FRAMEBUFFER = gl::FRAMEBUFFER
    }
}

gl_enum! {
    /// The various attachment points of a framebuffer object.
    pub gl_enum FramebufferAttachment {
        /// The color buffer attachment point.
        pub const ColorAttachment0 as COLOR_ATTACHMENT0 =
            gl::COLOR_ATTACHMENT0,

        /// The depth buffer attachment point.
        pub const DepthAttachment as DEPTH_ATTACHMENT =
            gl::DEPTH_ATTACHMENT,

        /// The stencil buffer attachment point.
        pub const StencilAttachment as STENCIL_ATTACHMENT =
            gl::STENCIL_ATTACHMENT
    }
}

/// An OpenGL context that has a free `GL_FRAMEBUFFER` binding.
pub trait FramebufferContext: AContext {
    /// The type of binder this context contains.
    type Binder: BorrowMut<FramebufferBinder>;

    /// The OpenGL context that will be returned after binding a framebuffer.
    type Rest: AContext;

    /// Split the context into a binder and the remaining context.
    fn split_framebuffer(self) -> (Self::Binder, Self::Rest);

    /// Bind a buffer to this context's framebuffer, returning a new context
    /// and a binding.
    ///
    /// # See also
    /// [`glBindFramebuffer`](http://docs.gl/es2/glBindFramebuffer) OpenGL docs
    fn bind_framebuffer<'a>(self, fbo: &'a mut Framebuffer)
        -> (FramebufferBinding<'a>, Self::Rest)
        where Self: Sized
    {
        let (mut binder, rest) = self.split_framebuffer();
        (binder.borrow_mut().bind(fbo), rest)
    }
}

impl<B, F, P, R, T> FramebufferContext for ContextOf<B, F, P, R, T>
    where F: BorrowMut<FramebufferBinder>
{
    type Binder = F;
    type Rest = ContextOf<B, (), P, R, T>;

    fn split_framebuffer(self) -> (Self::Binder, Self::Rest) {
        self.swap_framebuffer(())
    }
}

impl<'a, B, F, P, R, T> FramebufferContext for &'a mut ContextOf<B, F, P, R, T>
    where F: BorrowMut<FramebufferBinder>
{
    type Binder = &'a mut FramebufferBinder;
    type Rest = ContextOf<&'a mut B, (), &'a mut P, &'a mut R, &'a mut T>;

    fn split_framebuffer(self) -> (Self::Binder, Self::Rest) {
        let gl = self.borrowed_mut();
        gl.swap_framebuffer(())
    }
}



/// Represents a framebuffer that has been bound to the `GL_FRAMEBUFFER`
/// binding target.
pub struct FramebufferBinding<'a> {
    _phantom_ref: PhantomData<&'a mut Framebuffer>,
    _phantom_ptr: PhantomData<*mut ()>
}

impl<'a> FramebufferBinding<'a> {
    fn target(&self) -> FramebufferTarget {
        FramebufferTarget::Framebuffer
    }
}

/// The OpenGL state representing the `GL_FRAMEBUFFER` target.
pub struct FramebufferBinder {
    _phantom: PhantomData<*mut ()>
}

impl FramebufferBinder {
    /// Get the current `GL_FRAMEBUFFER` binder.
    ///
    /// # Safety
    /// The same rules apply to this method as the
    /// [`ContextOf::current_context()`]
    /// (../struct.ContextOf.html#method.current_context) method.
    pub unsafe fn current() -> Self {
        FramebufferBinder {
            _phantom: PhantomData
        }
    }

    /// Get the current `GL_FRAMEBUFFER` binding.
    ///
    /// # Safety
    /// This function should not be used to create an aliasing framebuffer
    /// binding.
    pub unsafe fn current_binding(&mut self) -> FramebufferBinding {
        FramebufferBinding {
            _phantom_ref: PhantomData,
            _phantom_ptr: PhantomData
        }
    }

    /// Bind a framebuffer to the `GL_FRAMEBUFFER` target, returning a binding.
    pub fn bind<'a>(&mut self, fbo: &'a mut Framebuffer)
        -> FramebufferBinding<'a>
    {
        let binding = FramebufferBinding {
            _phantom_ref: PhantomData,
            _phantom_ptr: PhantomData
        };
        unsafe {
            gl::BindFramebuffer(binding.target().gl_enum(), fbo.id());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_FRAMEBUFFER`",
                _ => "Unknown error"
            }
        }
        binding
    }
}
