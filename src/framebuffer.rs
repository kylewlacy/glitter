use std::marker::PhantomData;
use gl;
use gl::types::*;
use context::Context;
use renderbuffer::{Renderbuffer, RenderbufferTarget};
use texture::{Texture, TextureType, ImageTargetType};
use types::{BufferBits, GLError, GLFramebufferError};

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

impl Context {
    pub unsafe fn gen_framebuffer(&self) -> Framebuffer {
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



gl_enum! {
    pub gl_enum FramebufferTarget {
        Framebuffer as FRAMEBUFFER = gl::FRAMEBUFFER
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
    fn target(&self) -> FramebufferTarget {
        FramebufferTarget::Framebuffer
    }

    pub fn check_framebuffer_status(&self) -> Option<GLFramebufferError> {
        unsafe {
            match gl::CheckFramebufferStatus(self.target().gl_enum()) {
                gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                    Some(GLFramebufferError::IncompleteAttachment)
                },
                // gl::FRAMEBUFFER_INCOMPLETE_DIMENSIONS => {
                //     Some(GLFramebufferError::IncompleteDimensions)
                // },
                gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                    Some(GLFramebufferError::IncompleteMissingAttachment)
                },
                gl::FRAMEBUFFER_UNSUPPORTED => {
                    Some(GLFramebufferError::Unsupported)
                },
                _ => { None }
            }
        }
    }

    pub fn renderbuffer(&mut self,
                        attachment: FramebufferAttachment,
                        renderbuffer: &mut Renderbuffer)
    {
        let renderbuffer_target = RenderbufferTarget::Renderbuffer;
        unsafe {
            gl::FramebufferRenderbuffer(self.target().gl_enum(),
                                        attachment.gl_enum(),
                                        renderbuffer_target.gl_enum(),
                                        renderbuffer.gl_id());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_FRAMEBUFFER`, `attachment` is not a valid attachment point, or `renderbuffer` is not `GL_RENDERBUFFER` and `renderbuffer` is not 0",
                GLError::InvalidOperation => "Framebuffer 0 is bound, or `renderbuffer` is neither 0 nor the name of an existing renderbuffer object",
                _ => "Unknown error"
            }
        }
    }

    pub fn texture_2d<T: TextureType>(&mut self,
                                      attachment: FramebufferAttachment,
                                      tex_target: T::ImageTargetType,
                                      texture: &mut Texture<T>,
                                      level: i32)
    {
        debug_assert!(level == 0);

        unsafe {
            gl::FramebufferTexture2D(self.target().gl_enum(),
                                     attachment.gl_enum(),
                                     tex_target.gl_enum(),
                                     texture.gl_id(),
                                     level as GLint);
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_FRAMEBUFFER`, `attachment` is not an accepted attachment point, or `textarget` is not an accepted texture target and texture is not 0",
                GLError::InvalidValue => "`level` is not 0 and `texture` is not 0",
                GLError::InvalidOperation => "Framebuffer object 0 is bound, `texture` is neither 0 nor the name of an existing texture object, or `textarget` is not a valid target for `texture`",
                _ => "Unknown error"
            }
        }
    }

    pub fn clear(&mut self, buffers: BufferBits) {
        unsafe {
            gl::Clear(buffers.bits());
            dbg_gl_sanity_check! {
                GLError::InvalidValue => "`mask` includes a bit other than an allowed value",
                _ => "Unkown error"
            }
        }
    }
}

pub struct FramebufferBinder;
impl FramebufferBinder {
    pub unsafe fn current_binding(&mut self) -> FramebufferBinding {
        FramebufferBinding { phantom: PhantomData }
    }

    pub fn bind(&mut self, fbo: &mut Framebuffer) -> FramebufferBinding {
        let binding = FramebufferBinding { phantom: PhantomData };
        unsafe {
            gl::BindFramebuffer(binding.target().gl_enum(), fbo.gl_id());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_FRAMEBUFFER`",
                _ => "Unknown error"
            }
        }
        binding
    }
}
