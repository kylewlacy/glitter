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



pub struct RenderbufferBuilder<C>
    where C: RenderbufferContext
{
    gl: C,
    storage_params: Option<(RenderbufferFormat, u32, u32)>
}

impl<C> RenderbufferBuilder<C>
    where C: RenderbufferContext
{
    fn new(gl: C) -> Self {
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
        let gl = self.gl;
        let mut rbo = unsafe { gl.gen_renderbuffer() };

        match self.storage_params {
            Some((format, width, height)) => {
                {
                    let (mut gl_rbo, gl) = gl.bind_renderbuffer(&mut rbo);
                    gl.storage(&mut gl_rbo, format, width, height);
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

pub trait ContextRenderbufferBuilderExt: RenderbufferContext + Sized {
    fn build_renderbuffer(self) -> RenderbufferBuilder<Self>;
}

impl<'a, C: 'a> ContextRenderbufferBuilderExt for &'a mut C
    where &'a mut C: RenderbufferContext
{
    fn build_renderbuffer(self) -> RenderbufferBuilder<&'a mut C> {
        RenderbufferBuilder::new(self)
    }
}

pub unsafe trait ContextRenderbufferExt {
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

    fn storage(&self,
               gl_rbo: &mut RenderbufferBinding,
               format: RenderbufferFormat,
               width: u32,
               height: u32)
    {
        unsafe {
            gl::RenderbufferStorage(gl_rbo.target().gl_enum(),
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

unsafe impl<B, F, P, R, T> ContextRenderbufferExt for ContextOf<B, F, P, R, T> {

}

unsafe impl<'a, B, F, P, R, T> ContextRenderbufferExt
    for &'a mut ContextOf<B, F, P, R, T>
{

}




pub trait RenderbufferContext: AContext {
    type Binder: BorrowMut<RenderbufferBinder>;
    type Rest: AContext;

    fn split_renderbuffer(self) -> (Self::Binder, Self::Rest);

    fn bind_renderbuffer<'a>(self, rbo: &'a mut Renderbuffer)
        -> (RenderbufferBinding<'a>, Self::Rest)
        where Self: Sized
    {
        let (mut binder, rest) = self.split_renderbuffer();
        (binder.borrow_mut().bind(rbo), rest)
    }
}

impl<B, F, P, R, T> RenderbufferContext for ContextOf<B, F, P, R, T>
    where R: BorrowMut<RenderbufferBinder>
{
    type Binder = R;
    type Rest = ContextOf<B, F, P, (), T>;

    fn split_renderbuffer(self) -> (Self::Binder, Self::Rest) {
        self.split_renderbuffer()
    }
}

impl<'a, B, F, P, R, T> RenderbufferContext
    for &'a mut ContextOf<B, F, P, R, T>
    where R: BorrowMut<RenderbufferBinder>
{
    type Binder = &'a mut RenderbufferBinder;
    type Rest = ContextOf<&'a mut B, &'a mut F, &'a mut P, (), &'a mut T>;

    fn split_renderbuffer(self) -> (Self::Binder, Self::Rest) {
        let gl = self.borrowed_mut();
        gl.split_renderbuffer()
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
