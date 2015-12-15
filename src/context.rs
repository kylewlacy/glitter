use std::borrow::{Borrow, BorrowMut};
use gl;
use gl::types::*;
use ref_into::{RefInto, MutInto};
use types::{Color, Viewport, Capability, GLError};
use buffer::{BufferBinder, BufferBinderRef, BufferBinderMut};
use program::{ProgramBinder, ProgramAttrib};
use framebuffer::FramebufferBinder;
use renderbuffer::RenderbufferBinder;
use texture_units::TextureUnits;

pub type Context = ContextOf<BufferBinder,
                             FramebufferBinder,
                             ProgramBinder,
                             RenderbufferBinder,
                             TextureUnits>;

pub type ContextRef<'a> = ContextOf<&'a BufferBinder,
                                    &'a FramebufferBinder,
                                    &'a ProgramBinder,
                                    &'a RenderbufferBinder,
                                    &'a TextureUnits>;

pub type ContextMut<'a> = ContextOf<&'a mut BufferBinder,
                                    &'a mut FramebufferBinder,
                                    &'a mut ProgramBinder,
                                    &'a mut RenderbufferBinder,
                                    &'a mut TextureUnits>;

pub type ContextSubRef<'a> = ContextOf<BufferBinderRef<'a>,
                                       &'a FramebufferBinder,
                                       &'a ProgramBinder,
                                       &'a RenderbufferBinder,
                                       &'a TextureUnits>;

pub type ContextSubMut<'a> = ContextOf<BufferBinderMut<'a>,
                                       &'a mut FramebufferBinder,
                                       &'a mut ProgramBinder,
                                       &'a mut RenderbufferBinder,
                                       &'a mut TextureUnits>;

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

    pub fn clear_color(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    pub fn enable(&mut self, cap: Capability) {
        unsafe {
            gl::Enable(cap.gl_enum());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`cap` is not a valid OpenGL capability",
                _ => "Unknown error"
            }
        }
    }

    pub fn disable(&mut self, cap: Capability) {
        unsafe {
            gl::Disable(cap.gl_enum());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`cap` is not a valid OpenGL capability",
                _ => "Unknown error"
            }
        }
    }

    pub fn enable_vertex_attrib_array(&self, attrib: ProgramAttrib) {
        unsafe {
            gl::EnableVertexAttribArray(attrib.gl_index);
            dbg_gl_error! {
                GLError::InvalidValue => "`index` is >= GL_MAX_VERTEX_ATTRIBS",
                _ => "Unknown error"
            }
        }
    }

    pub fn viewport(&self, viewport: Viewport) {
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

    pub fn split_buffers(self)
        -> (B, ContextOf<(), F, P, R, T>)
    {
        (
            self.buffers,
            ContextOf {
                buffers: (),
                framebuffer: self.framebuffer,
                program: self.program,
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units
            }
        )
    }

    pub fn join_buffers<JB>(self, buffers: JB)
        -> ContextOf<JB, F, P, R, T>
    {
        ContextOf {
            buffers: buffers,
            framebuffer: self.framebuffer,
            program: self.program,
            renderbuffer: self.renderbuffer,
            tex_units: self.tex_units
        }
    }

    pub fn map_buffers<FN, BO>(self, f: FN) -> ContextOf<BO, F, P, R, T>
        where FN: FnOnce(B) -> BO
    {
        ContextOf {
            buffers: f(self.buffers),
            framebuffer: self.framebuffer,
            program: self.program,
            renderbuffer: self.renderbuffer,
            tex_units: self.tex_units
        }
    }

    pub fn split_framebuffer(self)
        -> (F, ContextOf<B, (), P, R, T>)
    {
        (
            self.framebuffer,
            ContextOf {
                buffers: self.buffers,
                framebuffer: (),
                program: self.program,
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units
            }
        )
    }

    pub fn split_program(self)
        -> (P, ContextOf<B, F, (), R, T>)
    {
        (
            self.program,
            ContextOf {
                buffers: self.buffers,
                framebuffer: self.framebuffer,
                program: (),
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units
            }
        )
    }

    pub fn split_renderbuffer(self)
        -> (R, ContextOf<B, F, P, (), T>)
    {
        (
            self.renderbuffer,
            ContextOf {
                buffers: self.buffers,
                framebuffer: self.framebuffer,
                program: self.program,
                renderbuffer: (),
                tex_units: self.tex_units
            }
        )
    }

    pub fn split_tex_units(self)
        -> (T, ContextOf<B, F, P, R, ()>)
    {
        (
            self.tex_units,
            ContextOf {
                buffers: self.buffers,
                framebuffer: self.framebuffer,
                program: self.program,
                renderbuffer: self.renderbuffer,
                tex_units: ()
            }
        )
    }
}

impl<'a, B, F, P, R, T, BI, FI, PI, RI, TI>
    RefInto<'a, ContextOf<BI, FI, PI, RI, TI>>
    for ContextOf<B, F, P, R, T>
    where B: RefInto<'a, BI>,
          F: RefInto<'a, FI>,
          P: RefInto<'a, PI>,
          R: RefInto<'a, RI>,
          T: RefInto<'a, TI>,
          BI: 'a, FI: 'a, PI: 'a, RI: 'a, TI: 'a
{
    fn ref_into(&'a self) -> ContextOf<BI, FI, PI, RI, TI> {
        ContextOf {
            buffers: self.buffers.ref_into(),
            framebuffer: self.framebuffer.ref_into(),
            program: self.program.ref_into(),
            renderbuffer: self.renderbuffer.ref_into(),
            tex_units: self.tex_units.ref_into()
        }
    }
}

impl<'a, B, F, P, R, T, BI, FI, PI, RI, TI>
    RefInto<'a, ContextOf<BI, FI, PI, RI, TI>>
    for &'a ContextOf<B, F, P, R, T>
    where B: RefInto<'a, BI>,
          F: RefInto<'a, FI>,
          P: RefInto<'a, PI>,
          R: RefInto<'a, RI>,
          T: RefInto<'a, TI>,
          BI: 'a, FI: 'a, PI: 'a, RI: 'a, TI: 'a
{
    fn ref_into(&'a self) -> ContextOf<BI, FI, PI, RI, TI> {
        ContextOf {
            buffers: self.buffers.ref_into(),
            framebuffer: self.framebuffer.ref_into(),
            program: self.program.ref_into(),
            renderbuffer: self.renderbuffer.ref_into(),
            tex_units: self.tex_units.ref_into()
        }
    }
}

impl<'a, B, F, P, R, T, BI, FI, PI, RI, TI>
    MutInto<'a, ContextOf<BI, FI, PI, RI, TI>>
    for ContextOf<B, F, P, R, T>
    where B: MutInto<'a, BI>,
          F: MutInto<'a, FI>,
          P: MutInto<'a, PI>,
          R: MutInto<'a, RI>,
          T: MutInto<'a, TI>,
          BI: 'a, FI: 'a, PI: 'a, RI: 'a, TI: 'a
{
    fn mut_into(&'a mut self) -> ContextOf<BI, FI, PI, RI, TI> {
        ContextOf {
            buffers: self.buffers.mut_into(),
            framebuffer: self.framebuffer.mut_into(),
            program: self.program.mut_into(),
            renderbuffer: self.renderbuffer.mut_into(),
            tex_units: self.tex_units.mut_into()
        }
    }
}

impl<'a, B, F, P, R, T, BI, FI, PI, RI, TI>
    MutInto<'a, ContextOf<BI, FI, PI, RI, TI>>
    for &'a ContextOf<B, F, P, R, T>
    where B: RefInto<'a, BI>,
          F: RefInto<'a, FI>,
          P: RefInto<'a, PI>,
          R: RefInto<'a, RI>,
          T: RefInto<'a, TI>,
          BI: 'a, FI: 'a, PI: 'a, RI: 'a, TI: 'a
{
    fn mut_into(&'a mut self) -> ContextOf<BI, FI, PI, RI, TI> {
        ContextOf {
            buffers: self.buffers.ref_into(),
            framebuffer: self.framebuffer.ref_into(),
            program: self.program.ref_into(),
            renderbuffer: self.renderbuffer.ref_into(),
            tex_units: self.tex_units.ref_into()
        }
    }
}

impl<'a, B, F, P, R, T, BI, FI, PI, RI, TI>
    MutInto<'a, ContextOf<BI, FI, PI, RI, TI>>
    for &'a mut ContextOf<B, F, P, R, T>
    where B: MutInto<'a, BI>,
          F: MutInto<'a, FI>,
          P: MutInto<'a, PI>,
          R: MutInto<'a, RI>,
          T: MutInto<'a, TI>,
          BI: 'a, FI: 'a, PI: 'a, RI: 'a, TI: 'a
{
    fn mut_into(&'a mut self) -> ContextOf<BI, FI, PI, RI, TI> {
        ContextOf {
            buffers: self.buffers.mut_into(),
            framebuffer: self.framebuffer.mut_into(),
            program: self.program.mut_into(),
            renderbuffer: self.renderbuffer.mut_into(),
            tex_units: self.tex_units.mut_into()
        }
    }
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
