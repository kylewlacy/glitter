use std::mem;
use super::gl;
use super::gl_lib::types::*;
use super::{Buffer, ArrayBufferBinder, ElementArrayBufferBinder};

pub struct Context {
    pub array_buffer: ArrayBufferBinder,
    pub element_array_buffer: ElementArrayBufferBinder
}

impl Context {
    pub unsafe fn current_context() -> Self {
        Context {
            array_buffer: ArrayBufferBinder,
            element_array_buffer: ElementArrayBufferBinder
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

    pub fn gen_buffer(&self) -> Buffer {
        unsafe {
            let mut id : GLuint = mem::uninitialized();
            gl::GenBuffers(1, &mut id as *mut GLuint);
            Buffer::from_id(id)
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
