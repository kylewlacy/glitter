use gl;
use gl::types::*;
use types::GLObject;

pub struct Program {
    gl_id: GLuint
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_id);
        }
    }
}

impl GLObject for Program {
    type Id = GLuint;

    unsafe fn from_raw(id: Self::Id) -> Self {
        Program { gl_id: id }
    }

    fn id(&self) -> Self::Id {
        self.gl_id
    }
}



#[derive(Debug, Clone, Copy)]
pub struct ProgramAttrib {
    pub gl_index: GLuint
}

#[derive(Debug, Clone, Copy)]
pub struct ProgramUniform {
    pub gl_index: GLuint
}
