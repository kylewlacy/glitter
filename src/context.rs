use super::gl_lib as gl;
use super::buffer::{ArrayBufferBinder, ElementArrayBufferBinder};
use super::program::{ProgramBinder};

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

    pub fn clear_color(&mut self, color: super::Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    pub fn clear(&mut self, buffers: super::BufferBits) {
        unsafe {
            gl::Clear(buffers.bits())
        }
    }

    pub fn enable_vertex_attrib_array(&self, attrib: super::ProgramAttrib) {
        unsafe {
            gl::EnableVertexAttribArray(attrib.gl_index);
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
