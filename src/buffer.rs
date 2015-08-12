use super::gl;
use super::gl_lib::types::*;

pub struct Buffer {
    gl_id: GLuint
}

impl Buffer {
    pub unsafe fn from_id(id: GLuint) -> Buffer {
        Buffer { gl_id: id }
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_id as *const GLuint);
        }
    }
}
