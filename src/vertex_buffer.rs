use std::marker::PhantomData;
use super::context::Context;
use super::vertex_data::{VertexData, VertexBytes, VertexAttribBinder};
use super::index_data::{IndexDatum};
use super::buffer::{Buffer, BufferBinding, ArrayBufferBinding};
use super::types::DrawingMode;

pub struct VertexBuffer<T: VertexData> {
    pub attrib_binder: Option<T::Binder>,
    buffer: Buffer,
    count: usize,
    phantom: PhantomData<*const T>
}

impl<T: VertexData> VertexBuffer<T> {
    pub fn build_attrib_binder(&self)
        -> <T::Binder as VertexAttribBinder>::Builder
    {
        T::build_attrib_binder()
    }

    pub fn bind(&self, gl_buffer: &ArrayBufferBinding) -> Result<(), ()> {
        match self.attrib_binder {
            Some(ref binder) => {
                binder.bind(gl_buffer);
                Ok(())
            },
            None => { Err(()) }
        }
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
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

    pub fn draw_arrays_range(&self,
                             mode: DrawingMode,
                             start: u32,
                             length: usize)
    {
        debug_assert!((start as usize) + length <= self.vbo.count);

        unsafe {
            self.gl_buffer.draw_arrays_range(mode, start, length);
        }
    }

    pub fn draw_arrays(&self, mode: DrawingMode) {
        unsafe {
            self.gl_buffer.draw_arrays_range(mode, 0, self.vbo.count);
        }
    }
}

impl Context {
    pub fn new_vertex_buffer<T: VertexData>(&self) -> VertexBuffer<T> {
        VertexBuffer {
            attrib_binder: None,
            buffer: self.gen_buffer(),
            count: 0,
            phantom: PhantomData
        }
    }
}



pub struct IndexBuffer<T: IndexDatum> {
    buffer: Buffer,
    count: usize,
    phantom: PhantomData<*const T>
}



#[macro_export]
macro_rules! bind_attrib_pointers {
    ($gl:expr, $vbo:expr, {
        $($field_name:ident => $field_attrib:expr),*
    }) => {
        {
            let vbo = $vbo;
            let binder = vbo.build_attrib_binder()
                            $(.$field_name($field_attrib))*
                            .unwrap($gl);
            vbo.attrib_binder = Some(binder)
        }
    }
}

#[macro_export]
macro_rules! bind_vertex_buffer {
    ($gl:expr, $vbo:expr) => {
        {
            let vbo = $vbo;

            let gl_buffer = bind_array_buffer!($gl, vbo.buffer_mut());
            vbo.bind(&gl_buffer).unwrap();
            $crate::VertexBufferBinding::new(gl_buffer, vbo)
        }
    }
}
