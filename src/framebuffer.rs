use gl;
use gl::types::*;

pub struct Framebuffer {
    gl_id: GLuint
}

impl Framebuffer {
    pub fn gl_id(&self) -> GLuint {
        self.gl_id
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.gl_id as *const GLuint);
        }
    }
}
