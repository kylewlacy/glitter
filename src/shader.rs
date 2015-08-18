use std::mem;
use std::ptr;
use super::gl;
use super::gl_lib::types::*;
use super::types::GLError;

pub struct Shader {
    gl_id: GLuint
}

impl Shader {
    pub unsafe fn from_id(id: GLuint) -> Shader {
        Shader { gl_id: id }
    }

    pub fn gl_id(&self) -> GLuint {
        self.gl_id
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

    pub fn compile(&mut self) -> Result<(), GLError> {
        let success = unsafe {
            gl::CompileShader(self.gl_id);
            let mut compile_status : GLint = mem::uninitialized();
            gl::GetShaderiv(self.gl_id,
                            gl::COMPILE_STATUS,
                            &mut compile_status as *mut GLint);

            compile_status == gl::TRUE as GLint
        };

        if success {
            Ok(())
        }
        else {
            unsafe {
                let mut info_length : GLint = mem::uninitialized();
                gl::GetShaderiv(self.gl_id,
                                gl::INFO_LOG_LENGTH,
                                &mut info_length as *mut GLint);

                let mut bytes = Vec::<u8>::with_capacity(info_length as usize);

                gl::GetShaderInfoLog(self.gl_id,
                                     info_length,
                                     ptr::null_mut(),
                                     bytes.as_mut_ptr() as *mut GLchar);

                let msg = String::from_utf8(bytes)
                                 .unwrap_or(String::from("<Unknown error>"));

                Err(GLError { message: msg })
            }
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
