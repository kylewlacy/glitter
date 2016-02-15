use std::marker::PhantomData;
use gl;
use gl::types::*;
use types::GLObject;

pub struct Renderbuffer {
    gl_id: GLuint,
    _phantom: PhantomData<*mut ()>
}

impl Drop for Renderbuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteRenderbuffers(1, &self.gl_id as *const GLuint);
        }
    }
}

impl GLObject for Renderbuffer {
    type Id = GLuint;

    unsafe fn from_raw(id: Self::Id) -> Self {
        Renderbuffer {
            gl_id: id,
            _phantom: PhantomData
        }
    }

    fn id(&self) -> Self::Id {
        self.gl_id
    }
}



gl_enum! {
    pub gl_enum RenderbufferTarget {
        Renderbuffer as RENDERBUFFER = gl::RENDERBUFFER
    }
}
