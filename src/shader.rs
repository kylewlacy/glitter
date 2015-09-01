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



unsafe fn _get_shader_iv(shader: &Shader,
                         pname: GLenum,
                         params: *mut GLint)
{
    gl::GetShaderiv(shader.gl_id(), pname, params);
    dbg_gl_sanity_check! {
        GLError::InvalidEnum => "`pname` is not an accepted value",
        GLError::InvalidValue => "`shader` is not a value generated by OpenGL",
        GLError::InvalidOperation => "`shader` is not a shader object, or `pname` is GL_COMPILE_STATUS, GL_INFO_LOG_LENGTH or GL_SHADER_SOURCE_LENGTH but a shader compiler is not supported",
        _ => "Unknown error"
    }
}

impl Context {
    pub fn create_shader(&self, shader_type: ShaderType) -> Result<Shader, ()> {
        unsafe {
            let id = gl::CreateShader(shader_type as GLenum);
            dbg_gl_sanity_check! {
                GLError::InvalidEnum => "`shaderType` is not an accepted value",
                _ => "Unknown error"
            }
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
            dbg_gl_error! {
                GLError::InvalidOperation => "`shader` is not a shader object or shader compiler is not supported",
                GLError::InvalidValue => "`shader` is not a value generated by OpenGL or `count` < 0",
                _ => "Unknown error"
            }
        }
    }

    pub fn compile_shader(&self, shader: &mut Shader) -> Result<(), GLError> {
        let success = unsafe {
            gl::CompileShader(shader.gl_id());
            dbg_gl_error! {
                GLError::InvalidOperation => "`shader` is not a shader object or shader compiler is not supported",
                GLError::InvalidValue => "`shader` is not a value generated by OpenGL",
                _ => "Unknown error"
            }

            let mut compile_status : GLint = 0;
            _get_shader_iv(shader,
                           gl::COMPILE_STATUS,
                           &mut compile_status as *mut GLint);

            compile_status == gl::TRUE as GLint
        };

        if success {
            Ok(())
        }
        else {
            let msg = match self.get_shader_info_log(&shader) {
                Some(s) => { s },
                None => { String::from("[Unknown shader error]") }
            };
            Err(GLError::Message(msg))
        }
    }

    pub fn get_shader_info_log(&self, shader: &Shader) -> Option<String> {
        unsafe {
            let mut info_length : GLint = 0;
            _get_shader_iv(shader,
                           gl::INFO_LOG_LENGTH,
                           &mut info_length as *mut GLint);

            if info_length > 0 {
                let mut bytes = Vec::<u8>::with_capacity(info_length as usize);

                gl::GetShaderInfoLog(shader.gl_id(),
                                     info_length,
                                     ptr::null_mut(),
                                     bytes.as_mut_ptr() as *mut GLchar);
                dbg_gl_sanity_check! {
                    GLError::InvalidValue => "`shader` is not a value generated by OpenGL, or `maxLength` < 0",
                    GLError::InvalidOperation => "`shader` is not a shader object",
                    _ => "Unknown error"
                }

                bytes.set_len((info_length - 1) as usize);

                String::from_utf8(bytes).ok()
            }
            else {
                None
            }
        }
    }
}

gl_enum! {
    pub gl_enum ShaderType {
        VertexShader as VERTEX_SHADER = gl::VERTEX_SHADER,
        FragmentShader as FRAGMENT_SHADER = gl::FRAGMENT_SHADER
    }
}
