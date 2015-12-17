use std::marker::PhantomData;
use std::collections::hash_map::{HashMap, Entry};
use std::borrow::BorrowMut;
use gl;
use gl::types::*;
use context::ContextOf;
use renderbuffer::{Renderbuffer, RenderbufferTarget};
use texture::{Texture, TextureType, ImageTargetType,
              Texture2d, Tx2dImageTarget};
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

enum BuilderAttachment<'a> {
    Texture2d(&'a mut Texture2d, i32),
    Renderbuffer(&'a mut Renderbuffer)
}

pub struct FramebufferBuilder<'a, B, F, P, R, T>
    where   B: 'a,
            F: 'a + BorrowMut<FramebufferBinder>,
            P: 'a,
            R: 'a,
            T: 'a
{
    gl: &'a mut ContextOf<B, F, P, R, T>,
    attachments: HashMap<FramebufferAttachment, BuilderAttachment<'a>>
}

impl<'a, B, F, P, R, T> FramebufferBuilder<'a, B, F, P, R, T>
    where B: 'a,
          F: 'a + BorrowMut<FramebufferBinder>,
          P: 'a,
          R: 'a,
          T: 'a
{
    fn new(gl: &'a mut ContextOf<B, F, P, R, T>) -> Self {
        FramebufferBuilder {
            gl: gl,
            attachments: HashMap::new()
        }
    }

    pub fn texture_2d(mut self,
                      attachment: FramebufferAttachment,
                      texture: &'a mut Texture2d,
                      level: i32)
        -> Self
    {
        let attached = BuilderAttachment::Texture2d(texture, level);
        match self.attachments.entry(attachment) {
            Entry::Occupied(mut e) => { e.insert(attached); },
            Entry::Vacant(e) => { e.insert(attached); }
        };

        self
    }

    pub fn renderbuffer(mut self,
                        attachment: FramebufferAttachment,
                        renderbuffer: &'a mut Renderbuffer)
        -> Self
    {
        let attached = BuilderAttachment::Renderbuffer(renderbuffer);
        match self.attachments.entry(attachment) {
            Entry::Occupied(mut e) => { e.insert(attached); },
            Entry::Vacant(e) => { e.insert(attached); }
        };

        self
    }

    pub fn try_unwrap(self) -> Result<Framebuffer, GLError> {
        let mut fbo = unsafe { self.gl.gen_framebuffer() };
        let fbo_status = {
            let gl = self.gl.borrowed_mut();
            let (mut gl_fbo, _) = gl.bind_framebuffer(&mut fbo);

            for (attachment, attached) in self.attachments.into_iter() {
                match attached {
                    BuilderAttachment::Texture2d(texture, level) => {
                        gl_fbo.texture_2d(attachment,
                                          Tx2dImageTarget::Texture2d,
                                          texture,
                                          level);
                    },
                    BuilderAttachment::Renderbuffer(renderbuffer) => {
                        gl_fbo.renderbuffer(attachment, renderbuffer);
                    }
                }
            }

            gl_fbo.check_framebuffer_status()
        };

        match fbo_status {
            Some(err) => { Err(err.into()) },
            None => { Ok(fbo) }
        }
    }

    pub fn unwrap(self) -> Framebuffer {
        self.try_unwrap().unwrap()
    }
}

impl<B, F, P, R, T> ContextOf<B, F, P, R, T> {
    pub fn build_framebuffer<'a>(&'a mut self)
        -> FramebufferBuilder<'a, B, F, P, R, T>
        where B: 'a,
              F: 'a + BorrowMut<FramebufferBinder>,
              P: 'a,
              R: 'a,
              T: 'a
    {
        FramebufferBuilder::new(self)
    }

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

pub trait FramebufferContext {
    type Rest;

    fn bind_framebuffer<'a>(self, fbo: &'a mut Framebuffer)
        -> (FramebufferBinding<'a>, Self::Rest);
}

impl<B, F, P, R, T> FramebufferContext for ContextOf<B, F, P, R, T>
    where F: BorrowMut<FramebufferBinder>
{
    type Rest = ContextOf<B, (), P, R, T>;

    fn bind_framebuffer<'a>(self, fbo: &'a mut Framebuffer)
        -> (FramebufferBinding<'a>, Self::Rest)
    {
        let (mut fbo_binder, rest) = self.split_framebuffer();
        (fbo_binder.borrow_mut().bind(fbo), rest)
    }
}

impl<'b, B, F, P, R, T> FramebufferContext for &'b mut ContextOf<B, F, P, R, T>
    where F: BorrowMut<FramebufferBinder>
{
    type Rest = ContextOf<&'b mut B, (), &'b mut P, &'b mut R, &'b mut T>;

    fn bind_framebuffer<'a>(self, fbo: &'a mut Framebuffer)
        -> (FramebufferBinding<'a>, Self::Rest)
    {
        let gl = self.mut_into();
        let (framebuffer_binder, rest): (&mut F, _) = gl.split_framebuffer();
        (framebuffer_binder.borrow_mut().bind(fbo), rest)
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

    pub fn texture_2d<T, I>(&mut self,
                            attachment: FramebufferAttachment,
                            tex_target: I,
                            texture: &mut Texture<T>,
                            level: i32)
        where T: TextureType, I: Into<T::ImageTargetType>
    {
        debug_assert!(level == 0);

        unsafe {
            gl::FramebufferTexture2D(self.target().gl_enum(),
                                     attachment.gl_enum(),
                                     tex_target.into().gl_enum(),
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

    pub fn bind<'a>(&mut self, fbo: &'a mut Framebuffer)
        -> FramebufferBinding<'a>
    {
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
