use gl;
use gl::types::*;
use types::GLObject;

pub struct Framebuffer {
    gl_id: GLuint
}

impl Framebuffer {
    pub unsafe fn from_gl(id: GLuint) -> Self {
        Framebuffer { gl_id: id }
    }

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

impl GLObject for Framebuffer {
    type Id = GLuint;

    unsafe fn from_raw(id: Self::Id) -> Self {
        Framebuffer { gl_id: id }
    }

    fn id(&self) -> Self::Id {
        self.gl_id
    }
}
