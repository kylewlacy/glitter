//! Home of [`ContextOf`](struct.ContextOf.html), which is the type that
//! represents "the OpenGL state machine", and the type you use to make
//! OpenGL calls.

use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use gl;
use gl::types::*;
use types::{Color, Viewport, Capability, GLError};
use program::ProgramAttrib;
use shader::ContextShaderExt;
use to_ref::{ToRef, ToMut};

pub mod buffer_context;
pub mod framebuffer_context;
pub mod program_context;
pub mod renderbuffer_context;
pub mod texture_context;
pub mod texture_units;

pub use self::buffer_context::*;
pub use self::framebuffer_context::*;
pub use self::program_context::*;
pub use self::renderbuffer_context::*;
pub use self::texture_context::*;
pub use self::texture_units::*;

/// A "fresh" OpenGL context: one that essentially has no active bindings.
/// See the [`ContextOf`](struct.ContextOf.html) docs for more details.
pub type Context = ContextOf<BufferBinder,
                             FramebufferBinder,
                             ProgramBinder,
                             RenderbufferBinder,
                             TextureUnits>;

/// The type that represents the whole "OpenGL state machine". This is the core
/// of glitter's design, and what enables the notion of safety.
///
/// To understand how it works, let's look at some code snippets:
///
/// This code will compile without errors:
///
/// ```no_run
/// #[macro_use] extern crate glitter;
/// use glitter::prelude::*;
///
/// # fn main() {
/// let gl = unsafe { glitter::Context::current_context() };
/// let mut buffer = gl.gen_buffer();
/// let (mut gl_buffer, gl) = gl.bind_array_buffer(&mut buffer);
/// gl.buffer_bytes(&mut gl_buffer, &[1, 2, 3], glitter::STATIC_DRAW);
/// # }
/// ```
///
/// ...and this code won't:
///
/// ```ignore
/// let gl: glitter::ContextOf<BufferBinder, _, _, _, _> = unsafe {
///     glitter::Context::current_context()
/// };
/// let mut buffer_1 = unsafe { gl.gen_buffer() };
/// let mut buffer_2 = unsafe { gl.gen_buffer() };
/// let (mut gl_buffer_1, gl): (_, ContextOf<BufferBinderOf<(), _>, _, _, _, _>) = gl.bind_array_buffer(&mut buffer_1);
/// unsafe { gl.buffer_byte(&mut gl_buffer_1); }
/// let (mut gl_buffer_2, gl) = gl.bind_array_buffer(&mut buffer_2);
/// //                             ^~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
/// // error: no method named `bind_array_buffer` found for type `...` found in
/// //        the current scope
/// // note: the method `bind_array_buffer` exists but the following trait
/// //       bounds were not satisfied: `() : BorrowMut<ArrayBufferBinder>`,
/// //       `() : BorrowMut<ArrayBufferBinder>`
/// ```
///
/// The magic of glitter lies in the two return values of [`bind_array_buffer`]
/// (buffer_context/trait.ArrayBufferContext.html#method.bind_array_buffer):
/// it returns both a "buffer binding" (something that represents that we
/// have a buffer currently bound, so we can send data to it), as well as a new
/// context, which doesn't have the `bind_array_buffer` method. How can we
/// achieve this type-level magic? Well, by using generic type parameters,
/// of course!
///
/// # Generic Type Parameters
///
/// Each of the generic type parameters represents a distinct 'piece' of OpenGL
/// state. Here's the state that each type parameter encapsulates:
///
/// - `B`: Buffer state (`GL_ARRAY_BUFFER`, `GL_ELEMENT_ARRAY_BUFFER`)
/// - `F`: Framebuffer state (`GL_FRAMEBUFFER`)
/// - `P`: Program state (the binding set by `glUseProgram`)
/// - `R`: Renderbuffer state (`GL_RENDERBUFFER`)
/// - `T`: Texture unit state (the texture number set by `glActiveTexture`)
///
/// As we saw with the above snippet, each type parameter can take on one of two
/// types: either a `Binding` type (meaning that it has an unbound target
/// that can be bound to), or the `()` type (meaning that the target has
/// already been used in a binding).
///
/// When a generic parameter is present (i.e., when it isn't just `()`), that
/// means that that 'part' of OpenGL state is free to be bound.
///
/// # Generic Code
/// In some circumstances, taking a concrete `Context` by value will be too
/// strict. Additionally, being generic over all of the type parameters adds
/// a great deal of complexity, and still isn't sufficient for all
/// circumstances. For these cases, there are a number of traits that exist,
/// which allow for much more flexibility than using a `ContextOf` instance
/// directly. The traits in question are:
///
/// - [`AContext`](trait.AContext.html)
/// - [`ArrayBufferContext`](buffer_context/trait.BufferContext.html),
/// [`ElementArrayBufferContext`](trait.ElementArrayBufferContext.html), and
/// [`BufferContext`](buffer_context/trait.BufferContext.html)
/// - [`FramebufferContext`](framebuffer_context/trait.FramebufferContext.html)
/// - [`RenderbufferContext`](renderbuffer_context/trait.RenderbufferContext.html)
/// - [`TextureUnit0Context`](texture_units/trait.TextureUnit0Context.html)
/// through [`TextureUnit7Context`](texture_units/trait.TextureUnit7Context.html)
pub struct ContextOf<B, F, P, R, T> {
    buffers: B,
    framebuffer: F,
    program: P,
    renderbuffer: R,
    tex_units: T,
    _phantom: PhantomData<*mut ()>
}

impl<B, F, P, R, T> ContextOf<B, F, P, R, T> {
    /// Use a function to load OpenGL function pointers. This function must
    /// be called before calling [`ContextOf::current_context`]
    /// (struct.ContextOf.html#method.current_context).
    ///
    /// # Safety
    /// `load_fn` takes an OpenGL function name, and must return a function
    /// pointer that can be used as this OpenGL function.
    pub unsafe fn load_with<L>(load_fn: L)
        where L: FnMut(&str) -> *const GLvoid
    {
        gl::load_with(load_fn);
    }

    /// Get the current OpenGL context.
    ///
    /// # Safety
    /// Before calling this function, **a context must be created and
    /// set** within the current thread, and **an OpenGL library needs
    /// to be loaded** by calling the [`ContextOf::load_with`]
    /// (struct.ContextOf.html#method.load_with) function.
    ///
    /// Additionally, special care needs to be taken with this function to
    /// maintain the invariants about bindings and targets. Here's an
    /// example of how this function can be abused:
    ///
    /// ```ignore
    /// // Get a fresh version the current context
    /// let gl = unsafe { glitter::Context::current_context() };
    /// // Create a buffer
    /// let mut buffer_1 = gl.gen_buffer();
    /// // Bind the buffer to the `GL_ARRAY_BUFFER` taret
    /// let (gl_buffer_1, gl) = gl.bind_array_buffer(&mut buffer_1);
    ///
    /// // Get a fresh version of the current context again
    /// let gl_2 = unsafe { Context::current_context() };
    /// // Create a new buffer
    /// let mut buffer_2 = gl_2.gen_buffer();
    /// // Bind the new buffer to `GL_ARRAY_BUFFER` (*replacing the
    /// // old binding*)
    /// let (gl_buffer_2, gl_2) = gl_2.bind_array_buffer(&mut buffer_2);
    ///
    /// // Send some data to the buffers:
    /// gl_buffer_2.buffer_bytes(&[1, 2, 3], glitter::STATIC_DRAW);
    ///
    /// // Current data:
    /// // buffer_1: {uninitialized}
    /// // buffer_2: [1, 2, 3]
    ///
    /// gl_buffer_1.buffer_bytes(&[4, 5, 6], glitter::STATIC_DRAW);
    /// ^~~~~~~~~~~~~~~~~~~~~~~~
    ///  UNSOUNDNESS: gl_buffer_1 refers to GL_ARRAY_BUFFER, which was
    ///               invalidated by gl_buffer_2. This call overwrites buffer_2
    ///               and leaves buffer_1 uninitialized.
    ///
    /// // Current data:
    /// // buffer_1: {uninitialized}
    /// // buffer_2: [4, 5, 6]
    /// ```
    pub unsafe fn current_context() -> Context {
        ContextOf {
            buffers: BufferBinder::current(),
            framebuffer: FramebufferBinder::current(),
            program: ProgramBinder::current(),
            renderbuffer: RenderbufferBinder::current(),
            tex_units: TextureUnits::current(),
            _phantom: PhantomData
        }
    }

    /// Get an OpenGL error that was generated since the last call to
    /// `ContextOf::get_error()`, or `None` is none occurred.
    ///
    /// # Note
    /// When the `debug_assertions` configuration option is set,
    /// `ContextOf::get_error` is automatically called after most OpenGL
    /// function calls (and the program will often panic if an error
    /// was generated).
    pub fn get_error() -> Option<GLError> {
        unsafe {
            match gl::GetError() {
                gl::INVALID_ENUM =>
                    Some(GLError::InvalidEnum),
                gl::INVALID_VALUE =>
                    Some(GLError::InvalidValue),
                gl::INVALID_OPERATION =>
                    Some(GLError::InvalidOperation),
                gl::INVALID_FRAMEBUFFER_OPERATION =>
                    Some(GLError::InvalidFramebufferOperation),
                gl::OUT_OF_MEMORY =>
                    Some(GLError::OutOfMemory),
                _ =>
                    None
            }
        }
    }

    /// Return a new `ContextOf`, where the type parameters of the new context
    /// are borrows of the current context. This function shouldn't be
    /// necessary in most circumstances, and will likely be removed from
    /// the public API in a future release.
    pub fn borrowed<'a, BB, BF, BP, BR, BT>(&'a self)
        -> ContextOf<&'a BB, &'a BF, &'a BP, &'a BR, &'a BT>
        where B: Borrow<BB>,
              F: Borrow<BF>,
              P: Borrow<BP>,
              R: Borrow<BR>,
              T: Borrow<BT>
    {
        ContextOf {
            buffers: self.buffers.borrow(),
            framebuffer: self.framebuffer.borrow(),
            program: self.program.borrow(),
            renderbuffer: self.renderbuffer.borrow(),
            tex_units: self.tex_units.borrow(),
            _phantom: PhantomData
        }
    }

    /// Return a new `ContextOf`, where the type parameters of the new context
    /// are mutable borrows of the current context. This function shouldn't
    /// be necessary in most circumstances, and will likely be removed from
    /// the public API in a future release.
    pub fn borrowed_mut<'a, BB, BF, BP, BR, BT>(&'a mut self)
        -> ContextOf<&'a mut BB,
                     &'a mut BF,
                     &'a mut BP,
                     &'a mut BR,
                     &'a mut BT>
        where B: BorrowMut<BB>,
              F: BorrowMut<BF>,
              P: BorrowMut<BP>,
              R: BorrowMut<BR>,
              T: BorrowMut<BT>
    {
        ContextOf {
            buffers: self.buffers.borrow_mut(),
            framebuffer: self.framebuffer.borrow_mut(),
            program: self.program.borrow_mut(),
            renderbuffer: self.renderbuffer.borrow_mut(),
            tex_units: self.tex_units.borrow_mut(),
            _phantom: PhantomData
        }
    }

    /// Replace the current context's internal `buffers` field (of type `B`)
    /// with a new value, returning the old value and a new context. This
    /// function will likely be removed from the public API in the future.
    ///
    /// # Example
    /// ```no_run
    /// // Get the current context
    /// let gl = unsafe { glitter::Context::current_context() };
    /// // Replace the context's buffer binder with `()`
    /// let (gl_buffer_binder, gl) = gl.swap_buffers(());
    /// ```
    pub fn swap_buffers<NB>(self, new_buffer: NB)
        -> (B, ContextOf<NB, F, P, R, T>)
    {
        (
            self.buffers,
            ContextOf {
                buffers: new_buffer,
                framebuffer: self.framebuffer,
                program: self.program,
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units,
                _phantom: PhantomData
            }
        )
    }

    /// Replace the current context's internal `framebuffer` field (of type `F`)
    /// with a new value, returning the old value and a new context. This
    /// function will likely be removed from the public API in the future.
    ///
    /// # Example
    /// ```no_run
    /// // Get the current context
    /// let gl = unsafe { glitter::Context::current_context() };
    /// // Replace the context's framebuffer binder with `()`
    /// let (gl_framebuffer_binder, gl) = gl.swap_framebuffer(());
    /// ```
    pub fn swap_framebuffer<NF>(self, new_framebuffer: NF)
        -> (F, ContextOf<B, NF, P, R, T>)
    {
        (
            self.framebuffer,
            ContextOf {
                buffers: self.buffers,
                framebuffer: new_framebuffer,
                program: self.program,
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units,
                _phantom: PhantomData
            }
        )
    }

    /// Replace the current context's internal `program` field (of type `P`)
    /// with a new value, returning the old value and a new context. This
    /// function will likely be removed from the public API in the future.
    ///
    /// # Example
    /// ```no_run
    /// // Get the current context
    /// let gl = unsafe { glitter::Context::current_context() };
    /// // Replace the context's program binder with `()`
    /// let (gl_program_binder, gl) = gl.swap_program(());
    /// ```
    pub fn swap_program<NP>(self, new_program: NP)
        -> (P, ContextOf<B, F, NP, R, T>)
    {
        (
            self.program,
            ContextOf {
                buffers: self.buffers,
                framebuffer: self.framebuffer,
                program: new_program,
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units,
                _phantom: PhantomData
            }
        )
    }

    /// Replace the current context's internal `renderbuffer` field (of
    /// type `B`) with a new value, returning a new context and the old value.
    /// This function will likely be removed from the public API in the future.
    ///
    /// # Example
    /// ```no_run
    /// // Get the current context
    /// let gl = unsafe { glitter::Context::current_context() };
    /// // Replace the context's renderbuffer binder with `()`
    /// let (gl_renderbuffer_binder, gl) = gl.swap_renderbuffer(());
    /// ```
    pub fn swap_renderbuffer<NR>(self, new_renderbuffer: NR)
        -> (R, ContextOf<B, F, P, NR, T>)
    {
        (
            self.renderbuffer,
            ContextOf {
                buffers: self.buffers,
                framebuffer: self.framebuffer,
                program: self.program,
                renderbuffer: new_renderbuffer,
                tex_units: self.tex_units,
                _phantom: PhantomData
            }
        )
    }

    /// Replace the current context's internal `tex_units` field (of type `T`)
    /// with a new value, returning the old value and a new context. This
    /// function will likely be removed from the public API in the future.
    ///
    /// # Example
    /// ```no_run
    /// // Get the current context
    /// let gl = unsafe { glitter::Context::current_context() };
    /// // Replace the context's texture unit binder with `()`
    /// let (gl_tex_unit_binder, gl) = gl.swap_tex_units(());
    /// ```
    pub fn swap_tex_units<NT>(self, new_tex_units: NT)
        -> (T, ContextOf<B, F, P, R, NT>)
    {
        (
            self.tex_units,
            ContextOf {
                buffers: self.buffers,
                framebuffer: self.framebuffer,
                program: self.program,
                renderbuffer: self.renderbuffer,
                tex_units: new_tex_units,
                _phantom: PhantomData
            }
        )
    }
}

impl<'a, B, F, P, R, T> ToRef<'a> for ContextOf<B, F, P, R, T>
    where B: 'a + ToRef<'a>,
          F: 'a + ToRef<'a>,
          P: 'a + ToRef<'a>,
          R: 'a + ToRef<'a>,
          T: 'a + ToRef<'a>
{
    type Ref = ContextOf<B::Ref, F::Ref, P::Ref, R::Ref, T::Ref>;

    fn to_ref(&'a self) -> Self::Ref {
        ContextOf {
            buffers: self.buffers.to_ref(),
            framebuffer: self.framebuffer.to_ref(),
            program: self.program.to_ref(),
            renderbuffer: self.renderbuffer.to_ref(),
            tex_units: self.tex_units.to_ref(),
            _phantom: PhantomData
        }
    }
}

impl<'a, B, F, P, R, T> ToMut<'a> for ContextOf<B, F, P, R, T>
    where B: 'a + ToMut<'a>,
          F: 'a + ToMut<'a>,
          P: 'a + ToMut<'a>,
          R: 'a + ToMut<'a>,
          T: 'a + ToMut<'a>
{
    type Mut = ContextOf<B::Mut, F::Mut, P::Mut, R::Mut, T::Mut>;

    fn to_mut(&'a mut self) -> Self::Mut {
        ContextOf {
            buffers: self.buffers.to_mut(),
            framebuffer: self.framebuffer.to_mut(),
            program: self.program.to_mut(),
            renderbuffer: self.renderbuffer.to_mut(),
            tex_units: self.tex_units.to_mut(),
            _phantom: PhantomData
        }
    }
}



/// A marker trait for types that represent an active OpenGL context.
///
/// # Safety
/// This type should only be implemented for types that can guarantee that
/// an OpenGL context will be available for the lifetime of an instance
/// of the type.
pub unsafe trait BaseContext {

}

unsafe impl<B, F, P, R, T> BaseContext for ContextOf<B, F, P, R, T> {

}

unsafe impl<'a, B, F, P, R, T> BaseContext
    for &'a mut ContextOf<B, F, P, R, T>
{

}



/// An extension trait that contains some of the core OpenGL methods that
/// maintain state, such as the current clear color or whether depth testing
/// is enabled.
pub trait ContextExt: BaseContext {
    /// Set the clear value when clearing a color buffer with
    /// [`gl.clear(glitter::COLOR_BUFFER_BIT)`]
    /// (framebuffer_context/trait.ContextFramebufferExt.html#method.clear).
    ///
    /// # Example
    /// ```no_run
    /// #[macro_use] extern crate glitter;
    /// use glitter::prelude::*;
    ///
    /// # fn main() {
    /// let mut gl = unsafe { glitter::Context::current_context() };
    /// // Clear the screen with solid red
    /// gl.clear_color(glitter::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    /// gl.clear(glitter::COLOR_BUFFER_BIT);
    /// # }
    /// ```
    fn clear_color(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    /// Enable an OpenGL capability.
    ///
    /// # Example
    /// ```no_run
    /// #[macro_use] extern crate glitter;
    /// use glitter::prelude::*;
    ///
    /// # fn main() {
    /// let mut gl = unsafe { glitter::Context::current_context() };
    /// // Enable depth testing
    /// gl.enable(glitter::DEPTH_TEST);
    /// # }
    /// ```
    fn enable(&mut self, cap: Capability) {
        unsafe {
            gl::Enable(cap.gl_enum());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`cap` is not a valid OpenGL capability",
                _ => "Unknown error"
            }
        }
    }

    /// Disable an OpenGL capability.
    ///
    /// # Example
    /// ```no_run
    /// #[macro_use] extern crate glitter;
    /// use glitter::prelude::*;
    ///
    /// # fn main() {
    /// let mut gl = unsafe { glitter::Context::current_context() };
    /// // Disable depth testing
    /// gl.disable(glitter::DEPTH_TEST);
    /// # }
    /// ```
    fn disable(&mut self, cap: Capability) {
        unsafe {
            gl::Disable(cap.gl_enum());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`cap` is not a valid OpenGL capability",
                _ => "Unknown error"
            }
        }
    }

    /// Enable the vertex attribute array to be used while drawing with
    /// [`gl.draw_arrays_range`](buffer_context/trait.ContextBufferExt.html#method.draw_arrays_range),
    /// [`gl.draw_elements`](buffer_context/trait.ContextBufferExt.html#method.draw_elements),
    /// [`gl.draw_n_elements`](buffer_context/trait.ContextBufferExt.html#method.draw_n_elements),
    /// [`gl.draw_n_elements_buffered`](buffer_context/trait.ContextBufferExt.html#method.draw_n_elements_buffered).
    ///
    /// # Panics
    /// This function will panics on an OpenGL error in debug mode.
    fn enable_vertex_attrib_array(&self, attrib: ProgramAttrib) {
        unsafe {
            gl::EnableVertexAttribArray(attrib.gl_index);
            dbg_gl_error! {
                GLError::InvalidValue => "`index` is >= GL_MAX_VERTEX_ATTRIBS",
                _ => "Unknown error"
            }
        }
    }

    /// Set the OpenGL viewport dimensions, which maps from device coordinates
    /// to window coordinates.
    fn viewport(&self, viewport: Viewport) {
        unsafe {
            gl::Viewport(viewport.x as GLint,
                         viewport.y as GLint,
                         viewport.width as GLsizei,
                         viewport.height as GLsizei);
            dbg_gl_sanity_check! {
                GLError::InvalidValue => "`width` or `height` is negative",
                _ => "Unknown error"
            }
        }
    }
}

impl<C: BaseContext> ContextExt for C {

}

/// Contains all of the [`ContextOf`](../struct.ContextOf.html) extension
/// traits that implement core OpenGL functionality.
pub mod ext {
    pub use BaseContext;
    pub use ContextExt;
    pub use ContextBufferExt;
    pub use ContextFramebufferExt;
    pub use ContextProgramExt;
    pub use ContextRenderbufferExt;
    pub use ContextShaderExt;
    pub use ContextTextureExt;
}

/// The 'core' OpenGL context trait. This trait provides access to any OpenGL
/// functionality that don't deal with binding. This trait is implemented for
/// `ContextOf<_, _, _, _, _>`, as well as for `&mut ContextOf<_, _, _, _, _>`.
pub trait AContext: ContextExt +
                    ContextBufferExt +
                    ContextFramebufferExt +
                    ContextProgramExt +
                    ContextRenderbufferExt +
                    ContextShaderExt +
                    ContextTextureExt
{

}

impl<B, F, P, R, T> AContext for ContextOf<B, F, P, R, T> {

}

impl<'a, B, F, P, R, T> AContext for &'a mut ContextOf<B, F, P, R, T> {

}
