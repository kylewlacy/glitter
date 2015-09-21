use gl;
use gl::types::*;
use types::{Color, Viewport, GLError};
use buffer::{ArrayBufferBinder, ElementArrayBufferBinder};
use program::{ProgramBinder, ProgramAttrib};
use texture_units::{TextureUnits};

pub struct Context {
    pub array_buffer: ArrayBufferBinder,
    pub element_array_buffer: ElementArrayBufferBinder,
    pub program: ProgramBinder,
    pub tex_units: TextureUnits
}

impl Context {
    pub unsafe fn current_context() -> Self {
        Context {
            array_buffer: ArrayBufferBinder,
            element_array_buffer: ElementArrayBufferBinder,
            program: ProgramBinder,
            tex_units: TextureUnits::current()
        }
    }

    pub fn clear_color(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
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
