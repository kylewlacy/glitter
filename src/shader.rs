use std::ptr;
use super::gl_lib as gl;
use super::gl_lib::types::*;
use super::context::Context;
use super::types::GLError;

pub struct Shader {
    gl_id: GLuint
}

impl Shader {
    pub fn gl_id(&self) -> GLuint {
        self.gl_id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.gl_id);
        }
    }
}

impl Context {
    pub fn create_shader(&self, shader_type: ShaderType) -> Result<Shader, ()> {
        unsafe {
            let id = gl::CreateShader(shader_type as GLenum);
            if id > 0 {
                Ok(Shader { gl_id: id })
            }
            else {
                Err(())
            }
        }
    }

    pub fn shader_source(&self, shader: &mut Shader, source: &str) {
        unsafe {
            let source_ptr = source.as_ptr() as *const GLchar;
            let source_len = source.len() as GLint;

            gl::ShaderSource(shader.gl_id(), 1,
                             &source_ptr as *const *const GLchar,
                             &source_len as *const GLint);
        }
    }

    pub fn compile_shader(&self, shader: &mut Shader) -> Result<(), GLError> {
        let success = unsafe {
            gl::CompileShader(shader.gl_id());
            let mut compile_status : GLint = 0;
            gl::GetShaderiv(shader.gl_id(),
                            gl::COMPILE_STATUS,
                            &mut compile_status as *mut GLint);

            compile_status == gl::TRUE as GLint
        };

        if success {
            Ok(())
        }
        else {
            unsafe {
                let mut info_length : GLint = 0;
                gl::GetShaderiv(shader.gl_id(),
                                gl::INFO_LOG_LENGTH,
                                &mut info_length as *mut GLint);

                let mut bytes = Vec::<u8>::with_capacity(info_length as usize);

                gl::GetShaderInfoLog(shader.gl_id(),
                                     info_length,
                                     ptr::null_mut(),
                                     bytes.as_mut_ptr() as *mut GLchar);
                bytes.set_len((info_length - 1) as usize);

                let msg = String::from_utf8(bytes)
                                 .unwrap_or(String::from("<Unknown error>"));

                Err(GLError { message: msg })
            }
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
