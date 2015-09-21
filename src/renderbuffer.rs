use std::marker::PhantomData;
use gl;
use gl::types::*;
use context::Context;
use types::GLError;

pub struct Renderbuffer {
    gl_id: GLuint
}

impl Renderbuffer {
    pub fn gl_id(&self) -> GLuint {
        self.gl_id
    }
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.gl_id as *const GLuint);
        }
    }
}

impl Context {
    pub fn gen_renderbuffer(&self) -> Renderbuffer {
        unsafe {
            let mut id : GLuint = 0;

            gl::GenRenderbuffers(1, &mut id as *mut GLuint);
            dbg_gl_sanity_check! {
                GLError::InvalidValue => "`n` is negative",
                _ => "Unknown error"
            }

            Renderbuffer {
                gl_id: id
            }
        }
    }
}

pub struct RenderbufferBinding<'a> {
    phantom: PhantomData<&'a mut Renderbuffer>
}

impl<'a> RenderbufferBinding<'a> {
    fn target(&self) -> GLenum {
        gl::RENDERBUFFER
    }
}

pub struct RenderbufferBinder;
impl RenderbufferBinder {
    pub fn bind<'a>(&'a mut self, renderbuffer: &mut Renderbuffer)
        -> RenderbufferBinding<'a>
    {
        let binding = RenderbufferBinding { phantom: PhantomData };
        unsafe {
            gl::BindRenderbuffer(binding.target(), renderbuffer.gl_id());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_RENDERBUFFER`",
                _ => "Unknown error"
            }
        }
        binding
    }
}