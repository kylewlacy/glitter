use std::borrow::{Borrow, BorrowMut};
use gl;
use gl::types::*;
use types::{Color, Viewport, Capability, GLError};
use buffer::{ArrayBufferBinder, ElementArrayBufferBinder};
use program::{ProgramBinder, ProgramAttrib};
use framebuffer::FramebufferBinder;
use renderbuffer::RenderbufferBinder;
use texture_units::TextureUnits;

pub type Context = ContextOf<ArrayBufferBinder,
                             ElementArrayBufferBinder,
                             ProgramBinder,
                             FramebufferBinder,
                             RenderbufferBinder,
                             TextureUnits>;

pub struct ContextOf<AB, EAB, P, FB, RB, TU> {
    pub array_buffer: AB,
    pub element_array_buffer: EAB,
    pub program: P,
    pub framebuffer: FB,
    pub renderbuffer: RB,
    pub tex_units: TU
}

impl<AB, EAB, P, FB, RB, TU> ContextOf<AB, EAB, P, FB, RB, TU> {
    pub unsafe fn current_context() -> Context {
        ContextOf {
            array_buffer: ArrayBufferBinder,
            element_array_buffer: ElementArrayBufferBinder,
            program: ProgramBinder,
            framebuffer: FramebufferBinder,
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

    pub fn borrowed<'a,
                    BAB = AB,
                    BEAB = EAB,
                    BP = P,
                    BFB = FB,
                    BRB = RB,
                    BTU = TU>
                   (&'a self)
        -> ContextOf<&'a BAB,
                     &'a BEAB,
                     &'a BP,
                     &'a BFB,
                     &'a BRB,
                     &'a BTU>
        where  AB: Borrow<BAB>,
              EAB: Borrow<BEAB>,
                P: Borrow<BP>,
               FB: Borrow<BFB>,
               RB: Borrow<BRB>,
               TU: Borrow<BTU>
    {
        ContextOf {
            array_buffer: self.array_buffer.borrow(),
            element_array_buffer: self.element_array_buffer.borrow(),
            program: self.program.borrow(),
            framebuffer: self.framebuffer.borrow(),
            renderbuffer: self.renderbuffer.borrow(),
            tex_units: self.tex_units.borrow()
        }
    }

    pub fn borrowed_mut<'a,
                        BAB = AB,
                        BEAB = EAB,
                        BP = P,
                        BFB = FB,
                        BRB = RB,
                        BTU = TU>
                       (&'a mut self)
        -> ContextOf<&'a mut BAB,
                     &'a mut BEAB,
                     &'a mut BP,
                     &'a mut BFB,
                     &'a mut BRB,
                     &'a mut BTU>
        where  AB: BorrowMut<BAB>,
              EAB: BorrowMut<BEAB>,
                P: BorrowMut<BP>,
               FB: BorrowMut<BFB>,
               RB: BorrowMut<BRB>,
               TU: BorrowMut<BTU>
    {
        ContextOf {
            array_buffer: self.array_buffer.borrow_mut(),
            element_array_buffer: self.element_array_buffer.borrow_mut(),
            program: self.program.borrow_mut(),
            framebuffer: self.framebuffer.borrow_mut(),
            renderbuffer: self.renderbuffer.borrow_mut(),
            tex_units: self.tex_units.borrow_mut()
        }
    }

    pub fn split_array_buffer(self)
        -> (AB, ContextOf<(), EAB, P, FB, RB, TU>)
    {
        (
            self.array_buffer,
            ContextOf {
                array_buffer: (),
                element_array_buffer: self.element_array_buffer,
                program: self.program,
                framebuffer: self.framebuffer,
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units
            }
        )
    }

    pub fn split_element_array_buffer(self)
        -> (EAB, ContextOf<AB, (), P, FB, RB, TU>)
    {
        (
            self.element_array_buffer,
            ContextOf {
                array_buffer: self.array_buffer,
                element_array_buffer: (),
                program: self.program,
                framebuffer: self.framebuffer,
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units
            }
        )
    }

    pub fn split_program(self)
        -> (P, ContextOf<AB, EAB, (), FB, RB, TU>)
    {
        (
            self.program,
            ContextOf {
                array_buffer: self.array_buffer,
                element_array_buffer: self.element_array_buffer,
                program: (),
                framebuffer: self.framebuffer,
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units
            }
        )
    }

    pub fn split_framebuffer(self)
        -> (FB, ContextOf<AB, EAB, P, (), RB, TU>)
    {
        (
            self.framebuffer,
            ContextOf {
                array_buffer: self.array_buffer,
                element_array_buffer: self.element_array_buffer,
                program: self.program,
                framebuffer: (),
                renderbuffer: self.renderbuffer,
                tex_units: self.tex_units
            }
        )
    }

    pub fn split_renderbuffer(self)
        -> (RB, ContextOf<AB, EAB, P, FB, (), TU>)
    {
        (
            self.renderbuffer,
            ContextOf {
                array_buffer: self.array_buffer,
                element_array_buffer: self.element_array_buffer,
                program: self.program,
                framebuffer: self.framebuffer,
                renderbuffer: (),
                tex_units: self.tex_units
            }
        )
    }

    pub fn split_tex_units(self)
        -> (TU, ContextOf<AB, EAB, P, FB, RB, ()>)
    {
        (
            self.tex_units,
            ContextOf {
                array_buffer: self.array_buffer,
                element_array_buffer: self.element_array_buffer,
                program: self.program,
                framebuffer: self.framebuffer,
                renderbuffer: self.renderbuffer,
                tex_units: ()
            }
        )
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
