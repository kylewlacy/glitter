use std::mem;
use std::ptr;
use std::marker::PhantomData;
use gl;
use gl::types::*;
use context::Context;
use types::{DrawingMode, GLError};
use index_data::{IndexData, IndexDatum, IndexDatumType};

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
            dbg_gl_sanity_check! {
                GLError::InvalidValue => "`n` is negative",
                _ => "Unknown error"
            }

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
                           usage as GLenum);
            dbg_gl_error! {
                GLError::InvalidEnum => "Invalid `target` or `usage`",
                GLError::InvalidValue => "`size` is negative",
                GLError::InvalidOperation => "Object 0 is bound to buffer target",
                GLError::OutOfMemory => "Unable to create a large enough buffer",
                _ => "Unknown error"
            }
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



unsafe fn _draw_elements(mode: DrawingMode,
                         count: usize,
                         index_type: IndexDatumType,
                         indicies: *const GLvoid)
{
    let gl_index_type: GLenum = match index_type {
        IndexDatumType::UnsignedByte => gl::UNSIGNED_BYTE,
        IndexDatumType::UnsignedShort => gl::UNSIGNED_SHORT
    };
    gl::DrawElements(mode as GLenum,
                     count as GLsizei,
                     gl_index_type,
                     indicies);
    dbg_gl_error! {
        GLError::InvalidEnum => "`mode` or `type` is not an accepted value",
        GLError::InvalidValue => "`count` is negative",
        GLError::InvalidFramebufferOperation => "The current framebuffer is not framebuffer-complete",
        _ => "Unknown error"
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
        dbg_gl_error! {
            GLError::InvalidEnum => "Illegal vertex attribute type",
            GLError::InvalidValue => "`stride` is negative, `size` is not in range, or `index` is >= GL_MAX_VERTEX_ATTRIBS",
            GLError::InvalidFramebufferOperation => "Currently bound framebuffer is not framebuffer complete",
            _ => "Unknown error"
        }
    }

    pub unsafe fn draw_arrays_range(&self,
                                    mode: DrawingMode,
                                    first: u32,
                                    count: usize)
    {
        gl::DrawArrays(mode as GLenum, first as GLint, count as GLsizei);
        dbg_gl_sanity_check! {
            GLError::InvalidEnum => "`mode` is not an accepted value",
            GLError::InvalidValue => "`count` is negative",
            _ => "Unknown error"
        }
    }

    pub unsafe fn draw_n_elements_buffered(&self,
                                           _eab: &ElementArrayBufferBinding,
                                           mode: DrawingMode,
                                           count: usize,
                                           index_type: IndexDatumType)
    {
        _draw_elements(mode, count, index_type, ptr::null());
    }

    pub unsafe fn draw_n_elements<I>(&self,
                                     mode: DrawingMode,
                                     count: usize,
                                     indicies: &[I])
        where I: IndexDatum, [I]: IndexData
    {
        debug_assert!(count <= indicies.len());

        let ptr = indicies.index_bytes().as_ptr();
        let index_type = I::index_datum_type();
        _draw_elements(mode, count, index_type, mem::transmute(ptr));
    }

    pub unsafe fn draw_elements<I>(&self,
                                   mode: DrawingMode,
                                   indicies: &[I])
        where I: IndexDatum, [I]: IndexData
    {
        self.draw_n_elements(mode, indicies.len(), indicies);
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



unsafe fn _bind_buffer(target: BufferBindingTarget, buffer: &mut Buffer) {
    gl::BindBuffer(target as GLuint, buffer.gl_id());
    dbg_gl_sanity_check! {
        GLError::InvalidEnum => "`target` is not an allowed value",
        _ => "Unknown error"
    }
}

pub struct ArrayBufferBinder;
impl ArrayBufferBinder {
    pub fn bind<'a>(&'a mut self, buffer: &mut Buffer)
        -> ArrayBufferBinding<'a>
    {
        let binding = ArrayBufferBinding { phantom: PhantomData };
        unsafe {
            _bind_buffer(binding.target(), buffer);
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
            _bind_buffer(binding.target(), buffer);
        }
        binding
    }
}
