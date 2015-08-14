use std::mem;
use super::gl;
use super::gl_lib::types::*;
use super::{Buffer, ArrayBufferBinder, ElementArrayBufferBinder};
use super::{Shader, ShaderType};

pub struct Context {
    pub array_buffer: ArrayBufferBinder,
    pub element_array_buffer: ElementArrayBufferBinder
}

impl Context {
    pub unsafe fn current_context() -> Self {
        Context {
            array_buffer: ArrayBufferBinder,
            element_array_buffer: ElementArrayBufferBinder
        }
    }

    pub fn clear_color(&mut self, color: super::Color) {
        unsafe {
            gl::ClearColor(color.r, color.g, color.b, color.a);
        }
    }

    pub fn clear(&mut self, buffers: super::BufferBits) {
        unsafe {
            gl::Clear(buffers.bits())
        }
    }

    pub fn vertex_attrib_pointer(&self,
                                 attrib: super::ProgramAttrib,
                                 components: i8,
                                 gl_type: super::DataType,
                                 normalized: bool,
                                 stride: usize,
                                 offset: usize)
    {
        unsafe {
            debug_assert!(1 <= components && components <= 4);

            let gl_normalized = if normalized { gl::TRUE } else { gl::FALSE };
            gl::VertexAttribPointer(attrib.gl_index,
                                    components as GLint,
                                    gl_type as GLenum,
                                    gl_normalized,
                                    stride as GLsizei,
                                    offset as *const GLvoid);
        }
    }

    pub fn enable_vertex_attrib_array(&self, attrib: super::ProgramAttrib) {
        unsafe {
            gl::EnableVertexAttribArray(attrib.gl_index);
        }
    }

    pub fn gen_buffer(&self) -> Buffer {
        unsafe {
            let mut id : GLuint = mem::uninitialized();
            gl::GenBuffers(1, &mut id as *mut GLuint);
            Buffer::from_id(id)
        }
    }

    pub fn create_shader(&self, shader_type: ShaderType) -> Result<Shader, ()> {
        unsafe {
            let id = gl::CreateShader(shader_type as GLenum);
            if id > 0 {
                Ok(Shader::from_id(id))
            }
            else {
                Err(())
            }
        }
    }
}

#[macro_export]
macro_rules! bind_array_buffer {
    ($gl:expr, $buffer:expr) => {
        $gl.array_buffer.bind($buffer)
    }
}

#[macro_export]
macro_rules! bind_element_array_buffer {
    ($gl:expr, $buffer:expr) => {
        $gl.element_array_buffer.bind($buffer)
    }
}
