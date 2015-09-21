use gl;
use gl::types::*;
use context::Context;
use types::GLError;

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

impl Context {
    pub fn gen_framebuffer(&self) -> Framebuffer {
        unsafe {
            let mut id : GLuint = 0;

            gl::GenFramebuffers(1, &mut id as *mut GLuint);
            dbg_gl_sanity_check! {
                GLError::InvalidValue => "`n` is negative",
                _ => "Unknown error"
            }

            Framebuffer {
                gl_id: id
            }
        }
    }
}
