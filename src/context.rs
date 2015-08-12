use std::mem;
use super::gl;
use super::gl_lib::types::*;
use super::Buffer;

pub struct Context;

impl Context {
    pub unsafe fn current_context() -> Self {
        Context
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
