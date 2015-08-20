use std::marker::PhantomData;
use super::Context;
use super::{VertexData, VertexBytes, VertexAttribBinder};
use super::Buffer;
use super::{BufferBinding, ArrayBufferBinding};

pub struct VertexBuffer<T: VertexData> {
    pub attrib_binder: Option<T::Binder>,
    pub buffer: Buffer,
    count: usize,
    phantom: PhantomData<*const T>
}

impl<T: VertexData> VertexBuffer<T> {
    pub fn new(buffer: Buffer) -> Self {
        VertexBuffer {
            attrib_binder: None,
            buffer: buffer,
            count: 0,
            phantom: PhantomData
        }
    }

    pub fn build_attrib_binder(&self)
        -> <T::Binder as VertexAttribBinder>::Builder
    {
        T::build_attrib_binder()
    }

    pub fn bind(&self, gl: &super::Context) -> Result<(), ()> {
        match self.attrib_binder {
            Some(ref binder) => {
                binder.bind(gl);
                Ok(())
            },
            None => { Err(()) }
        }
    }
}

pub struct VertexBufferBinding<'a, T: VertexData>
    where T: 'a, T::Binder: 'a
{
    gl_buffer: ArrayBufferBinding<'a>,
    vbo: &'a mut VertexBuffer<T>
}

impl<'a, T: VertexData> VertexBufferBinding<'a, T>
    where T: 'a, T::Binder: 'a
{
    pub fn new(gl_buffer: ArrayBufferBinding<'a>, vbo: &'a mut VertexBuffer<T>)
        -> Self
    {
        VertexBufferBinding {
            gl_buffer: gl_buffer,
            vbo: vbo
        }
    }
}
