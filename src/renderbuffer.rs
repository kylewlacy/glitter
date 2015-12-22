use std::marker::PhantomData;
use std::borrow::BorrowMut;
use gl;
use gl::types::*;
use context::{AContext, ContextOf};
use image_data::{RenderbufferFormat};
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



pub struct RenderbufferBuilder<'a, B, F, P, R, T>
    where B: 'a,
          F: 'a,
          P: 'a,
          R: 'a + BorrowMut<RenderbufferBinder>,
          T: 'a
{
    gl: &'a mut ContextOf<B, F, P, R, T>,
    storage_params: Option<(RenderbufferFormat, u32, u32)>
}

impl<'a, B, F, P, R, T> RenderbufferBuilder<'a, B, F, P, R, T>
    where B: 'a,
          F: 'a,
          P: 'a,
          R: 'a + BorrowMut<RenderbufferBinder>,
          T: 'a
{
    fn new(gl: &'a mut ContextOf<B, F, P, R, T>) -> Self {
        RenderbufferBuilder {
            gl: gl,
            storage_params: None
        }
    }

    pub fn storage(mut self,
                   format: RenderbufferFormat,
                   width: u32,
                   height: u32)
        -> Self
    {
        self.storage_params = Some((format, width, height));
        self
    }

    pub fn try_unwrap(self) -> Result<Renderbuffer, GLError> {
        let gl = self.gl.borrowed_mut();
        let mut rbo = unsafe { gl.gen_renderbuffer() };

        match self.storage_params {
            Some((format, width, height)) => {
                {
                    let (mut gl_rbo, _) = gl.bind_renderbuffer(&mut rbo);
                    gl_rbo.storage(format, width, height);
                }

                Ok(rbo)
            },
            None => {
                let msg = "Error building renderbuffer: no format or dimensions provided";
                Err(GLError::Message(msg.to_owned()))
            }
        }
    }

    pub fn unwrap(self) -> Renderbuffer {
        self.try_unwrap().unwrap()
    }
}

impl<B, F, P, R, T> ContextOf<B, F, P, R, T> {
    // TODO: Move this method into `ContextRenderbufferExt`
    pub fn build_renderbuffer<'a>(&'a mut self)
        -> RenderbufferBuilder<'a, B, F, P, R, T>
        where R: BorrowMut<RenderbufferBinder>
    {
        RenderbufferBuilder::new(self)
    }
}

pub trait ContextRenderbufferExt {
    unsafe fn gen_renderbuffer(&self) -> Renderbuffer;
}

impl<B, F, P, R, T> ContextRenderbufferExt for ContextOf<B, F, P, R, T> {
    unsafe fn gen_renderbuffer(&self) -> Renderbuffer {
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

// TODO: Add a macro to reduce this boilerplate
impl<'a, B, F, P, R, T> ContextRenderbufferExt
    for &'a mut ContextOf<B, F, P, R, T>
{
    unsafe fn gen_renderbuffer(&self) -> Renderbuffer {
        (**self).gen_renderbuffer()
    }
}




pub trait RenderbufferContext: AContext {
    type Rest: AContext;

    fn bind_renderbuffer<'a>(self, rbo: &'a mut Renderbuffer)
        -> (RenderbufferBinding<'a>, Self::Rest);
}

impl<B, F, P, R, T> RenderbufferContext for ContextOf<B, F, P, R, T>
    where R: BorrowMut<RenderbufferBinder>
{
    type Rest = ContextOf<B, F, P, (), T>;

    fn bind_renderbuffer<'a>(self, rbo: &'a mut Renderbuffer)
        -> (RenderbufferBinding<'a>, Self::Rest)
    {
        let (mut rbo_binder, rest) = self.split_renderbuffer();
        (rbo_binder.borrow_mut().bind(rbo), rest)
    }
}

impl<'b, B, F, P, R, T> RenderbufferContext
    for &'b mut ContextOf<B, F, P, R, T>
    where R: BorrowMut<RenderbufferBinder>
{
    type Rest = ContextOf<&'b mut B, &'b mut F, &'b mut P, (), &'b mut T>;

    fn bind_renderbuffer<'a>(self, rbo: &'a mut Renderbuffer)
        -> (RenderbufferBinding<'a>, Self::Rest)
    {
        let gl = self.mut_into();
        let (rbo_binder, rest): (&mut R, _) = gl.split_renderbuffer();
        (rbo_binder.borrow_mut().bind(rbo), rest)
    }
}



gl_enum! {
    pub gl_enum RenderbufferTarget {
        Renderbuffer as RENDERBUFFER = gl::RENDERBUFFER
    }
}

pub struct RenderbufferBinding<'a> {
    phantom: PhantomData<&'a mut Renderbuffer>
}

impl<'a> RenderbufferBinding<'a> {
    fn target(&self) -> RenderbufferTarget {
        RenderbufferTarget::Renderbuffer
    }

    pub fn storage(&mut self,
                   format: RenderbufferFormat,
                   width: u32,
                   height: u32)
    {
        unsafe {
            gl::RenderbufferStorage(self.target().gl_enum(),
                                    format.gl_enum(),
                                    width as GLint,
                                    height as GLint);
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_RENDERBUFFER` or `internalformat` is not an accepted format",
                GLError::InvalidValue => "`width` or `height` is less than zero or greater than `GL_MAX_RENDERBUFFER_SIZE`",
                GLError::OutOfMemory => "Unable to allocate enough memory for requested size",
                GLError::InvalidOperation => "Renderbuffer object 0 is bound",
                _ => "Unknown error"
            }
        }
    }
}

pub struct RenderbufferBinder;
impl RenderbufferBinder {
    pub fn bind<'a>(&mut self, renderbuffer: &'a mut Renderbuffer)
        -> RenderbufferBinding<'a>
    {
        let binding = RenderbufferBinding { phantom: PhantomData };
        unsafe {
            gl::BindRenderbuffer(binding.target().gl_enum(),
                                 renderbuffer.gl_id());
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`target` is not `GL_RENDERBUFFER`",
                _ => "Unknown error"
            }
        }
        binding
    }
}
