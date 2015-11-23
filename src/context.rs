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

    pub fn split_array_buffer<'a>(&'a self)
        -> (
            &'a ArrayBufferBinder,
            ContextOf<(), &'a EAB, &'a P, &'a FB, &'a RB, &'a TU>
        )
        where AB: Borrow<ArrayBufferBinder>
    {
        (
            self.array_buffer.borrow(),
            ContextOf {
                array_buffer: (),
                element_array_buffer: &self.element_array_buffer,
                program: &self.program,
                framebuffer: &self.framebuffer,
                renderbuffer: &self.renderbuffer,
                tex_units: &self.tex_units
            }
        )
    }

    pub fn split_array_buffer_mut<'a>(&'a mut self)
        -> (
            &'a mut ArrayBufferBinder,
            ContextOf<(),
                      &'a mut EAB,
                      &'a mut P,
                      &'a mut FB,
                      &'a mut RB,
                      &'a mut TU>
        )
        where AB: BorrowMut<ArrayBufferBinder>
    {
        (
            self.array_buffer.borrow_mut(),
            ContextOf {
                array_buffer: (),
                element_array_buffer: &mut self.element_array_buffer,
                program: &mut self.program,
                framebuffer: &mut self.framebuffer,
                renderbuffer: &mut self.renderbuffer,
                tex_units: &mut self.tex_units
            }
        )
    }

    pub fn split_element_array_buffer<'a>(&'a self)
        -> (
            &'a ElementArrayBufferBinder,
            ContextOf<&'a AB, (), &'a P, &'a FB, &'a RB, &'a TU>
        )
        where EAB: Borrow<ElementArrayBufferBinder>
    {
        (
            self.element_array_buffer.borrow(),
            ContextOf {
                array_buffer: &self.array_buffer,
                element_array_buffer: (),
                program: &self.program,
                framebuffer: &self.framebuffer,
                renderbuffer: &self.renderbuffer,
                tex_units: &self.tex_units
            }
        )
    }

    pub fn split_element_array_buffer_mut<'a>(&'a mut self)
        -> (
            &'a mut ElementArrayBufferBinder,
            ContextOf<&'a mut AB,
                      (),
                      &'a mut P,
                      &'a mut FB,
                      &'a mut RB,
                      &'a mut TU>
        )
        where EAB: BorrowMut<ElementArrayBufferBinder>
    {
        (
            self.element_array_buffer.borrow_mut(),
            ContextOf {
                array_buffer: &mut self.array_buffer,
                element_array_buffer: (),
                program: &mut self.program,
                framebuffer: &mut self.framebuffer,
                renderbuffer: &mut self.renderbuffer,
                tex_units: &mut self.tex_units
            }
        )
    }

    pub fn split_program<'a>(&'a self)
        -> (
            &'a ProgramBinder,
            ContextOf<&'a AB, &'a EAB, (), &'a FB, &'a RB, &'a TU>
        )
        where P: Borrow<ProgramBinder>
    {
        (
            self.program.borrow(),
            ContextOf {
                array_buffer: &self.array_buffer,
                element_array_buffer: &self.element_array_buffer,
                program: (),
                framebuffer: &self.framebuffer,
                renderbuffer: &self.renderbuffer,
                tex_units: &self.tex_units
            }
        )
    }

    pub fn split_program_mut<'a>(&'a mut self)
        -> (
            &'a mut ProgramBinder,
            ContextOf<&'a mut AB,
                      &'a mut EAB,
                      (),
                      &'a mut FB,
                      &'a mut RB,
                      &'a mut TU>
        )
        where P: BorrowMut<ProgramBinder>
    {
        (
            self.program.borrow_mut(),
            ContextOf {
                array_buffer: &mut self.array_buffer,
                element_array_buffer: &mut self.element_array_buffer,
                program: (),
                framebuffer: &mut self.framebuffer,
                renderbuffer: &mut self.renderbuffer,
                tex_units: &mut self.tex_units
            }
        )
    }

    pub fn split_framebuffer<'a>(&'a self)
        -> (
            &'a FramebufferBinder,
            ContextOf<&'a AB, &'a EAB, &'a P, (), &'a RB, &'a TU>
        )
        where FB: Borrow<FramebufferBinder>
    {
        (
            self.framebuffer.borrow(),
            ContextOf {
                array_buffer: &self.array_buffer,
                element_array_buffer: &self.element_array_buffer,
                program: &self.program,
                framebuffer: (),
                renderbuffer: &self.renderbuffer,
                tex_units: &self.tex_units
            }
        )
    }

    pub fn split_framebuffer_mut<'a>(&'a mut self)
        -> (
            &'a mut FramebufferBinder,
            ContextOf<&'a mut AB,
                      &'a mut EAB,
                      &'a mut P,
                      (),
                      &'a mut RB,
                      &'a mut TU>
        )
        where FB: BorrowMut<FramebufferBinder>
    {
        (
            self.framebuffer.borrow_mut(),
            ContextOf {
                array_buffer: &mut self.array_buffer,
                element_array_buffer: &mut self.element_array_buffer,
                program: &mut self.program,
                framebuffer: (),
                renderbuffer: &mut self.renderbuffer,
                tex_units: &mut self.tex_units
            }
        )
    }

    pub fn split_renderbuffer<'a>(&'a self)
        -> (
            &'a RenderbufferBinder,
            ContextOf<&'a AB, &'a EAB, &'a P, &'a FB, (), &'a TU>
        )
        where RB: Borrow<RenderbufferBinder>
    {
        (
            self.renderbuffer.borrow(),
            ContextOf {
                array_buffer: &self.array_buffer,
                element_array_buffer: &self.element_array_buffer,
                program: &self.program,
                framebuffer: &self.framebuffer,
                renderbuffer: (),
                tex_units: &self.tex_units
            }
        )
    }

    pub fn split_renderbuffer_mut<'a>(&'a mut self)
        -> (
            &'a mut RenderbufferBinder,
            ContextOf<&'a mut AB,
                      &'a mut EAB,
                      &'a mut P,
                      &'a mut FB,
                      (),
                      &'a mut TU>
        )
        where RB: BorrowMut<RenderbufferBinder>
    {
        (
            self.renderbuffer.borrow_mut(),
            ContextOf {
                array_buffer: &mut self.array_buffer,
                element_array_buffer: &mut self.element_array_buffer,
                program: &mut self.program,
                framebuffer: &mut self.framebuffer,
                renderbuffer: (),
                tex_units: &mut self.tex_units
            }
        )
    }

    pub fn split_tex_units<'a>(&'a self)
        -> (
            &'a TextureUnits,
            ContextOf<&'a AB, &'a EAB, &'a P, &'a FB, &'a RB, ()>
        )
        where TU: Borrow<TextureUnits>
    {
        (
            self.tex_units.borrow(),
            ContextOf {
                array_buffer: &self.array_buffer,
                element_array_buffer: &self.element_array_buffer,
                program: &self.program,
                framebuffer: &self.framebuffer,
                renderbuffer: &self.renderbuffer,
                tex_units: ()
            }
        )
    }

    pub fn split_tex_units_mut<'a>(&'a mut self)
        -> (
            &'a mut TextureUnits,
            ContextOf<&'a mut AB,
                      &'a mut EAB,
                      &'a mut P,
                      &'a mut FB,
                      &'a mut RB,
                      ()>
        )
        where TU: BorrowMut<TextureUnits>
    {
        (
            self.tex_units.borrow_mut(),
            ContextOf {
                array_buffer: &mut self.array_buffer,
                element_array_buffer: &mut self.element_array_buffer,
                program: &mut self.program,
                framebuffer: &mut self.framebuffer,
                renderbuffer: &mut self.renderbuffer,
                tex_units: ()
            }
        )
    }
}

#[macro_export]
macro_rules! bind_array_buffer {
    ($gl:expr, $buffer:expr) => {
        $gl.array_buffer.bind($buffer)
    }
}

#[macro_export]
macro_rules! bind_element_array_buffer {
    ($gl:expr, $buffer:expr) => {
        $gl.element_array_buffer.bind($buffer)
    }
}

#[macro_export]
macro_rules! use_program {
    ($gl:expr, $program:expr) => {
        $gl.program.bind($program)
    }
}

#[macro_export]
macro_rules! bind_framebuffer {
    ($gl:expr, $fbo:expr) => {
        $gl.framebuffer.bind($fbo)
    }
}

#[macro_export]
macro_rules! current_framebuffer_binding {
    ($gl:expr) => {
        $gl.framebuffer.current_binding()
    }
}

#[macro_export]
macro_rules! bind_renderbuffer {
    ($gl:expr, $renderbuffer:expr) => {
        $gl.renderbuffer.bind($renderbuffer)
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
