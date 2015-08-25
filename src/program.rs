use std::mem;
use std::ptr;
use std::ffi::CString;
use super::gl_lib as gl;
use super::gl_lib::types::*;
use super::types::GLError;
use super::context::Context;
use super::shader::Shader;

pub struct Program {
    gl_id: GLuint
}

impl Program {
    pub fn gl_id(&self) -> GLuint {
        self.gl_id
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_id);
        }
    }
}

impl Context {
    pub fn create_program(&self) -> Result<Program, ()> {
        unsafe {
            let id = gl::CreateProgram();
            if id > 0 {
                Ok(Program { gl_id: id })
            }
            else {
                Err(())
            }
        }
    }

    pub fn attach_shader(&self, program: &mut Program, shader: &Shader) {
        unsafe {
            gl::AttachShader(program.gl_id(), shader.gl_id());
        }
    }

    pub fn link_program(&self, program: &mut Program) -> Result<(), GLError> {
        let success = unsafe {
            gl::LinkProgram(program.gl_id());
            let mut link_status : GLint = 0;
            gl::GetProgramiv(program.gl_id(),
                             gl::LINK_STATUS,
                             &mut link_status as *mut GLint);

            link_status == gl::TRUE as GLint
        };

        if success {
            Ok(())
        }
        else {
            unsafe {
                let mut info_length : GLint = mem::uninitialized();
                gl::GetProgramiv(program.gl_id(),
                                 gl::INFO_LOG_LENGTH,
                                 &mut info_length as *mut GLint);

                let mut bytes = Vec::<u8>::with_capacity(info_length as usize);

                gl::GetProgramInfoLog(program.gl_id(),
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

    pub fn get_attrib_location(&self, program: &Program, name: &str)
        -> Result<ProgramAttrib, ()>
    {
        let c_str = try!(CString::new(name).or(Err(())));
        let str_ptr = c_str.as_ptr() as *const GLchar;
        unsafe {
            let index = gl::GetAttribLocation(program.gl_id(), str_ptr);
            if index >= 0 {
                Ok(ProgramAttrib { gl_index: index as GLuint })
            }
            else {
                Err(())
            }
        }
    }

    pub fn get_uniform_location(&self, program: &Program, name: &str)
        -> Result<ProgramUniform, ()>
    {
        let c_str = try!(CString::new(name).or(Err(())));
        let str_ptr = c_str.as_ptr() as *const GLchar;
        unsafe {
            let index = gl::GetUniformLocation(program.gl_id(), str_ptr);
            if index >= 0 {
                Ok(ProgramUniform { gl_index: index as GLuint })
            }
            else {
                Err(())
            }
        }
    }

    pub fn use_program(&self, program: &Program) {
        unsafe {
            gl::UseProgram(program.gl_id())
        }
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
