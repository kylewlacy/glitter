use std::marker::PhantomData;
use std::borrow::BorrowMut;
use gl;
use gl::types::*;
use context::ContextOf;
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



pub struct RenderbufferBuilder<'a, AB, EAB, P, FB, RB, TU>
    where  AB: 'a,
          EAB: 'a,
            P: 'a,
           FB: 'a,
           RB: 'a + BorrowMut<RenderbufferBinder>,
           TU: 'a
{
    gl: &'a mut ContextOf<AB, EAB, P, FB, RB, TU>,
    storage_params: Option<(RenderbufferFormat, u32, u32)>
}

impl<'a, AB, EAB, P, FB, RB, TU> RenderbufferBuilder<'a, AB, EAB, P, FB, RB, TU>
    where  AB: 'a,
          EAB: 'a,
            P: 'a,
           FB: 'a,
           RB: 'a + BorrowMut<RenderbufferBinder>,
           TU: 'a
{
    fn new(gl: &'a mut ContextOf<AB, EAB, P, FB, RB, TU>) -> Self {
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

impl<AB, EAB, P, FB, RB, TU> ContextOf<AB, EAB, P, FB, RB, TU> {
    pub fn build_renderbuffer<'a>(&'a mut self)
        -> RenderbufferBuilder<'a, AB, EAB, P, FB, RB, TU>
        where RB: BorrowMut<RenderbufferBinder>
    {
        RenderbufferBuilder::new(self)
    }

    pub unsafe fn gen_renderbuffer(&self) -> Renderbuffer {
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

    pub fn bind_renderbuffer<'a>(self, renderbuffer: &'a mut Renderbuffer)
        -> (
            RenderbufferBinding<'a>,
            ContextOf<AB, EAB, P, FB, (), TU>
        )
        where RB: BorrowMut<RenderbufferBinder>
    {
        let (mut renderbuffer_binder, gl) = self.split_renderbuffer();
        (renderbuffer_binder.borrow_mut().bind(renderbuffer), gl)
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
