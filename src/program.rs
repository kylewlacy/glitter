use super::gl;
use super::gl_lib::types::*;
use super::Shader;

pub struct Program {
    gl_id: GLuint
}

impl Program {
    pub unsafe fn from_id(id: GLuint) -> Program {
        Program { gl_id: id }
    }

    pub fn attach_shader(&mut self, shader: &Shader) {
        unsafe {
            gl::AttachShader(self.gl_id, shader.gl_id());
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_id);
        }
    }
}



#[derive(Debug, Clone, Copy)]
pub struct ProgramAttrib {
    pub gl_index: GLuint
}
