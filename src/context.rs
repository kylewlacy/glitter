use super::gl_lib as gl;
use super::gl_lib::types::*;
use super::types::{Color, Viewport, BufferBits, GLError};
use super::buffer::{ArrayBufferBinder, ElementArrayBufferBinder};
use super::program::{ProgramBinder, ProgramAttrib};

// Used to specify checks that shouldn't fail (but might in unsafe)
macro_rules! dbg_gl_error {
    ($($pat:pat => $msg:expr),*) => {
        if cfg!(debug_assertions) {
            let err = $crate::Context::get_error();
            match err {
                $(Some($pat) => {
                    panic!("OpenGL error {:?} - {}", err, $msg)
                }),*
                None => { }
            }
        }
    }
}

// Used to specify checks that should *never* be able to fail (even in unsafe!)
macro_rules! dbg_gl_sanity_check {
    ($($pat:pat => $msg:expr),*) => {
        dbg_gl_error! { $($pat => concat!("Sanity check failed: ", $msg)),* }
    }
}



pub struct Context {
    pub array_buffer: ArrayBufferBinder,
    pub element_array_buffer: ElementArrayBufferBinder,
    pub program: ProgramBinder
}

impl Context {
    pub unsafe fn current_context() -> Self {
        Context {
            array_buffer: ArrayBufferBinder,
            element_array_buffer: ElementArrayBufferBinder,
            program: ProgramBinder
        }
    }

    pub fn clear_color(&mut self, color: Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    pub fn clear(&mut self, buffers: BufferBits) {
        unsafe {
            gl::Clear(buffers.bits())
        }
    }

    pub fn enable_vertex_attrib_array(&self, attrib: ProgramAttrib) {
        unsafe {
            gl::EnableVertexAttribArray(attrib.gl_index);
        }
    }

    pub fn viewport(&self, viewport: Viewport) {
        unsafe {
            gl::Viewport(viewport.x as GLint,
                         viewport.y as GLint,
                         viewport.width as GLsizei,
                         viewport.height as GLsizei);
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
