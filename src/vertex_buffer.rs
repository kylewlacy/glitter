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

    pub fn buffer_data(&mut self, data: &[T], usage: super::BufferDataUsage)
        where [T]: VertexBytes
    {
        self.vbo.count = data.len();
        self.gl_buffer.buffer_bytes(data.vertex_bytes(), usage);
    }
}

#[macro_export]
macro_rules! bind_vertex_buffer {
    ($gl:expr, $vbo:expr) => {
        {
            let vbo = $vbo;
            let gl : &mut _ = $gl;

            {
                vbo.bind(gl).unwrap();
            }

            let gl_buffer = bind_array_buffer!(gl, &mut vbo.buffer);
            $crate::VertexBufferBinding::new(gl_buffer, vbo)
        }
    }
}
