use super::gl;
use super::gl_lib::types::*;

pub struct Shader {
    gl_id: GLuint
}

impl Shader {
    pub unsafe fn from_id(id: GLuint) -> Shader {
        Shader { gl_id: id }
    }

    pub fn source(&mut self, shader_source: &str) {
        unsafe {
            let source = shader_source.as_ptr() as *const GLchar;
            let length = shader_source.len() as GLint;

            gl::ShaderSource(self.gl_id, 1,
                             &source as *const *const GLchar,
                             &length as *const GLint);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.gl_id);
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum ShaderType {
    VERTEX_SHADER = gl::VERTEX_SHADER as isize,
    FRAGMENT_SHADER = gl::FRAGMENT_SHADER as isize
}
pub use self::ShaderType::*;
