use super::gl;
use super::gl_lib::types::*;

pub struct Shader {
    gl_id: GLuint
}

impl Shader {
    pub unsafe fn from_id(id: GLuint) -> Shader {
        Shader { gl_id: id }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.gl_id);
        }
    }
}
