use std::marker::PhantomData;
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

gl_enum! {
    pub gl_enum FramebufferAttachment {
        ColorAttachment0 as COLOR_ATTACHMENT0 = gl::COLOR_ATTACHMENT0,
        DepthAttachment as DEPTH_ATTACHMENT = gl::DEPTH_ATTACHMENT,
        StencilAttachment as STENCIL_ATTACHMENT = gl::STENCIL_ATTACHMENT
    }
}

pub struct FramebufferBinding<'a> {
    phantom: PhantomData<&'a mut Framebuffer>
}

impl<'a> FramebufferBinding<'a> {
    fn target(&self) -> GLenum {
        gl::FRAMEBUFFER
    }
}

pub struct FramebufferBinder;
impl FramebufferBinder {
    pub fn bind<'a>(&'a mut self, fbo: &mut Framebuffer)
        -> FramebufferBinding<'a>
    {
        let binding = FramebufferBinding { phantom: PhantomData };
        unsafe {
            gl::BindFramebuffer(binding.target(), fbo.gl_id());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_FRAMEBUFFER`",
                _ => "Unknown error"
            }
        }
        binding
    }
}
