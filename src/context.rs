use std::borrow::{Borrow, BorrowMut};
use gl;
use gl::types::*;
use types::{Color, Viewport, Capability, GLError};
use buffer::{BufferBinder, ContextBufferExt};
use framebuffer::{FramebufferBinder, ContextFramebufferExt};
use program::{ProgramBinder, ContextProgramExt, ProgramAttrib};
use renderbuffer::{RenderbufferBinder, ContextRenderbufferExt};
use shader::ContextShaderExt;
use texture::ContextTextureExt;
use texture_units::TextureUnits;
use to_ref::{ToRef, ToMut};

pub type Context = ContextOf<BufferBinder,
                             FramebufferBinder,
                             ProgramBinder,
                             RenderbufferBinder,
                             TextureUnits>;

pub struct ContextOf<B, F, P, R, T> {
    pub buffers: B,
    pub framebuffer: F,
    pub program: P,
    pub renderbuffer: R,
    pub tex_units: T
}

impl<B, F, P, R, T> ContextOf<B, F, P, R, T> {
    pub unsafe fn current_context() -> Context {
        ContextOf {
            buffers: BufferBinder::current(),
            framebuffer: FramebufferBinder,
            program: ProgramBinder,
            renderbuffer: RenderbufferBinder,
            tex_units: TextureUnits::current()
        }
    }

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

    pub fn borrowed<'a, BB = B, BF = F, BP = P, BR = R, BT = T>(&'a self)
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
            tex_units: self.tex_units.borrow()
        }
    }

    pub fn borrowed_mut<'a, BB = B, BF = F, BP = P, BR = R, BT = T>
                       (&'a mut self)
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
            tex_units: self.tex_units.borrow_mut()
        }
    }

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
                tex_units: self.tex_units
            }
        )
    }

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
                tex_units: self.tex_units
            }
        )
    }

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
                tex_units: self.tex_units
            }
        )
    }

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
                tex_units: self.tex_units
            }
        )
    }

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
                tex_units: new_tex_units
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
            tex_units: self.tex_units.to_ref()
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
            tex_units: self.tex_units.to_mut()
        }
    }
}

pub unsafe trait ContextExt {
    fn clear_color(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    fn enable(&mut self, cap: Capability) {
        unsafe {
            gl::Enable(cap.gl_enum());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`cap` is not a valid OpenGL capability",
                _ => "Unknown error"
            }
        }
    }

    fn disable(&mut self, cap: Capability) {
        unsafe {
            gl::Disable(cap.gl_enum());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`cap` is not a valid OpenGL capability",
                _ => "Unknown error"
            }
        }
    }

    fn enable_vertex_attrib_array(&self, attrib: ProgramAttrib) {
        unsafe {
            gl::EnableVertexAttribArray(attrib.gl_index);
            dbg_gl_error! {
                GLError::InvalidValue => "`index` is >= GL_MAX_VERTEX_ATTRIBS",
                _ => "Unknown error"
            }
        }
    }

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

unsafe impl<B, F, P, R, T> ContextExt for ContextOf<B, F, P, R, T> {

}

unsafe impl<'a, B, F, P, R, T> ContextExt for &'a mut ContextOf<B, F, P, R, T> {

}

pub mod ext {
    pub use ContextExt;
    pub use ContextBufferExt;
    pub use ContextFramebufferExt;
    pub use ContextProgramExt;
    pub use ContextRenderbufferExt;
    pub use ContextShaderExt;
    pub use ContextTextureExt;
}

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



// HACK: Workaround for issue described here:
// https://www.reddit.com/r/rust/comments/339yj3/tuple_indexing_in_a_macro/cqiyv4n
#[macro_export]
macro_rules! _glitter_expr {
    ($x:expr) => ($x)
}

#[macro_export]
macro_rules! active_texture {
    ($gl:expr, $idx:tt) => {
        _glitter_expr!($gl.tex_units.$idx.active())
    }
}

#[macro_export]
macro_rules! active_texture_n {
    ($gl:expr, $idx:expr) => {
        $gl.tex_units.nth_unit($idx).active()
    }
}
