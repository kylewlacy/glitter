use std::mem;
use std::ptr;
use std::marker::PhantomData;
use std::ffi::CString;
use super::gl_lib as gl;
use super::gl_lib::types::*;
use super::types::GLError;
use super::context::Context;
use super::shader::Shader;
use super::uniform_data::{UniformData, UniformDatumType, UniformPrimitiveType};

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

                Err(GLError::Message(msg))
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
}


pub struct ProgramBinding<'a> {
    phantom: PhantomData<&'a mut Program>
}

impl<'a> ProgramBinding<'a> {
    pub fn set_uniform<T>(&self, uniform: ProgramUniform, val: T)
        where T: UniformData
    {
        let idx = uniform.gl_index as GLint;
        let count = val.uniform_elements() as GLsizei;
        let ptr = val.uniform_bytes().as_ptr();
        unsafe {
            match T::uniform_datum_type() {
                UniformDatumType::Vec1(p) => {
                    match p {
                        UniformPrimitiveType::Float => {
                            gl::Uniform1fv(idx, count, ptr as *const GLfloat)
                        },
                        UniformPrimitiveType::Int => {
                            gl::Uniform1iv(idx, count, ptr as *const GLint)
                        }
                    }
                },
                UniformDatumType::Vec2(p) => {
                    match p {
                        UniformPrimitiveType::Float => {
                            gl::Uniform2fv(idx, count, ptr as *const GLfloat)
                        },
                        UniformPrimitiveType::Int => {
                            gl::Uniform2iv(idx, count, ptr as *const GLint)
                        }
                    }
                },
                UniformDatumType::Vec3(p) => {
                    match p {
                        UniformPrimitiveType::Float => {
                            gl::Uniform2fv(idx, count, ptr as *const GLfloat)
                        },
                        UniformPrimitiveType::Int => {
                            gl::Uniform2iv(idx, count, ptr as *const GLint)
                        }
                    }
                },
                UniformDatumType::Vec4(p) => {
                    match p {
                        UniformPrimitiveType::Float => {
                            gl::Uniform2fv(idx, count, ptr as *const GLfloat)
                        },
                        UniformPrimitiveType::Int => {
                            gl::Uniform2iv(idx, count, ptr as *const GLint)
                        }
                    }
                },
                UniformDatumType::Matrix2x2 => {
                    gl::UniformMatrix2fv(idx,
                                         count,
                                         gl::FALSE,
                                         ptr as *const GLfloat)
                },
                UniformDatumType::Matrix3x3 => {
                    gl::UniformMatrix3fv(idx,
                                         count,
                                         gl::FALSE,
                                         ptr as *const GLfloat)
                },
                UniformDatumType::Matrix4x4 => {
                    gl::UniformMatrix4fv(idx,
                                         count,
                                         gl::FALSE,
                                         ptr as *const GLfloat)
                },
            }
        }
    }
}

pub struct ProgramBinder;
impl ProgramBinder {
    pub fn bind<'a>(&'a mut self, program: &mut Program)
        -> ProgramBinding<'a>
    {
        let binding = ProgramBinding { phantom: PhantomData };
        unsafe {
            gl::UseProgram(program.gl_id());
        }
        binding
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
