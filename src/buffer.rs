use std::mem;
use std::marker::PhantomData;
use super::gl_lib as gl;
use super::gl_lib::types::*;
use super::context::Context;
use super::types::DrawingMode;

pub struct Buffer {
    gl_id: GLuint
}

impl Buffer {
    pub fn gl_id(&self) -> GLuint {
        self.gl_id
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_id as *const GLuint);
        }
    }
}

impl Context {
    pub fn gen_buffer(&self) -> Buffer {
        unsafe {
            let mut id : GLuint = mem::uninitialized();
            gl::GenBuffers(1, &mut id as *mut GLuint);
            Buffer { gl_id: id }
        }
    }
}



#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum BufferDataUsage {
    STREAM_DRAW = gl::STREAM_DRAW as isize,
    STATIC_DRAW = gl::STATIC_DRAW as isize,
    DYNAMIC_DRAW = gl::DYNAMIC_DRAW as isize
}
pub use self::BufferDataUsage::*;

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum BufferBindingTarget {
    ARRAY_BUFFER = gl::ARRAY_BUFFER as isize,
    ELEMENT_ARRAY_BUFFER = gl::ELEMENT_ARRAY_BUFFER as isize
}
pub use self::BufferBindingTarget::*;



pub trait BufferBinding {
    fn target(&self) -> BufferBindingTarget;

    fn buffer_bytes(&mut self, bytes: &[u8], usage: BufferDataUsage) {
        let ptr = bytes.as_ptr();
        let size = bytes.len() * mem::size_of::<u8>();
        unsafe {
            gl::BufferData(self.target() as GLenum,
                           size as GLsizeiptr,
                           ptr as *const GLvoid,
                           usage as GLenum)
        }
    }
}

pub struct ArrayBufferBinding<'a> {
    phantom: PhantomData<&'a mut Buffer>
}

impl<'a> BufferBinding for ArrayBufferBinding<'a> {
    fn target(&self) -> BufferBindingTarget {
        ARRAY_BUFFER
    }
}

impl<'a> ArrayBufferBinding<'a> {
    pub unsafe fn vertex_attrib_pointer(&self,
                                        attrib: super::ProgramAttrib,
                                        components: i8,
                                        gl_type: super::DataType,
                                        normalized: bool,
                                        stride: usize,
                                        offset: usize)
    {
        debug_assert!(1 <= components && components <= 4);

        let gl_normalized = if normalized { gl::TRUE } else { gl::FALSE };
        gl::VertexAttribPointer(attrib.gl_index,
                                components as GLint,
                                gl_type as GLenum,
                                gl_normalized,
                                stride as GLsizei,
                                offset as *const GLvoid);
    }

    pub unsafe fn draw_arrays_range(&self,
                                    mode: DrawingMode,
                                    first: u32,
                                    count: usize)
    {
        gl::DrawArrays(mode as GLenum, first as GLint, count as GLsizei);
    }
}

pub struct ElementArrayBufferBinding<'a> {
    phantom: PhantomData<&'a mut Buffer>
}

impl<'a> BufferBinding for ElementArrayBufferBinding<'a> {
    fn target(&self) -> BufferBindingTarget {
        ELEMENT_ARRAY_BUFFER
    }
}



pub struct ArrayBufferBinder;
impl ArrayBufferBinder {
    pub fn bind<'a>(&'a mut self, buffer: &mut Buffer)
        -> ArrayBufferBinding<'a>
    {
        let binding = ArrayBufferBinding { phantom: PhantomData };
        unsafe {
            gl::BindBuffer(binding.target() as GLuint, buffer.gl_id);
        }
        binding
    }
}

pub struct ElementArrayBufferBinder;
impl ElementArrayBufferBinder {
    pub fn bind<'a>(&'a mut self, buffer: &mut Buffer)
        -> ElementArrayBufferBinding<'a>
    {
        let binding = ElementArrayBufferBinding { phantom: PhantomData };
        unsafe {
            gl::BindBuffer(binding.target() as GLuint, buffer.gl_id);
        }
        binding
    }
}
