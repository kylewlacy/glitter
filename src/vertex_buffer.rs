use std::marker::PhantomData;
use context::Context;
use framebuffer::FramebufferBinding;
use vertex_data::{VertexData, VertexBytes, VertexAttribBinder};
use index_data::{IndexData, IndexDatum};
use buffer::{Buffer, BufferBinding,
             ArrayBufferBinding, ElementArrayBufferBinding};
use types::DrawingMode;

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

    pub fn bind_attrib_pointers(&mut self, binder: T::Binder) {
        self.attrib_binder = Some(binder);
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
}

impl<'a> FramebufferBinding<'a> {
    pub fn draw_arrays_range_vbo<T>(&mut self,
                                    gl_vbo: &VertexBufferBinding<T>,
                                    mode: DrawingMode,
                                    start: u32,
                                    length: usize)
        where T: VertexData
    {
        debug_assert!((start as usize) + length <= gl_vbo.vbo.count);

        unsafe {
            self.draw_arrays_range(&gl_vbo.gl_buffer, mode, start, length);
        }
    }

    pub fn draw_arrays_vbo<T>(&mut self,
                              gl_vbo: &VertexBufferBinding<T>,
                              mode: DrawingMode)
        where T: VertexData
    {
        unsafe {
            self.draw_arrays_range(&gl_vbo.gl_buffer,
                                   mode,
                                   0,
                                   gl_vbo.vbo.count);
        }
    }

    pub fn draw_n_elements_buffered_vbo<T, I>(&mut self,
                                              gl_vbo: &VertexBufferBinding<T>,
                                              gl_ibo: &IndexBufferBinding<I>,
                                              mode: DrawingMode,
                                              length: usize)
        where T: VertexData, I: IndexDatum
    {
        debug_assert!(length <= gl_ibo.ibo.count);

        unsafe {
            self.draw_n_elements_buffered(&gl_vbo.gl_buffer,
                                          &gl_ibo.gl_buffer,
                                          mode,
                                          length,
                                          I::index_datum_type());
        }
    }

    pub fn draw_elements_buffered_vbo<T, I>(&mut self,
                                            gl_vbo: &VertexBufferBinding<T>,
                                            gl_ibo: &IndexBufferBinding<I>,
                                            mode: DrawingMode)
        where T: VertexData, I: IndexDatum
    {
        unsafe {
            self.draw_n_elements_buffered(&gl_vbo.gl_buffer,
                                          &gl_ibo.gl_buffer,
                                          mode,
                                          gl_ibo.ibo.count,
                                          I::index_datum_type());
        }
    }

    pub fn draw_n_elements_vbo<T, I>(&mut self,
                                     gl_vbo: &VertexBufferBinding<'a, T>,
                                     mode: DrawingMode,
                                     count: usize,
                                     indicies: &[I])
        where T: VertexData, I: IndexDatum, [I]: IndexData
    {
        unsafe {
            self.draw_n_elements(&gl_vbo.gl_buffer, mode, count, indicies);
        }
    }

    pub fn draw_elements_vbo<T, I>(&mut self,
                                   gl_vbo: &VertexBufferBinding<'a, T>,
                                   mode: DrawingMode,
                                   indicies: &[I])
        where T: VertexData, I: IndexDatum, [I]: IndexData
    {
        unsafe {
            self.draw_elements(&gl_vbo.gl_buffer, mode, indicies);
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

impl<T: IndexDatum> IndexBuffer<T> {
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}

pub struct IndexBufferBinding<'a, T: IndexDatum + 'a> {
    gl_buffer: ElementArrayBufferBinding<'a>,
    ibo: &'a mut IndexBuffer<T>
}

impl<'a, T: IndexDatum + 'a> IndexBufferBinding<'a, T> {
    pub fn new(gl_buffer: ElementArrayBufferBinding<'a>,
               ibo: &'a mut IndexBuffer<T>)
        -> Self
    {
        IndexBufferBinding {
            gl_buffer: gl_buffer,
            ibo: ibo
        }
    }

    pub fn buffer_data(&mut self, data: &[T], usage: super::BufferDataUsage)
        where [T]: IndexData
    {
        self.ibo.count = data.len();
        self.gl_buffer.buffer_bytes(data.index_bytes(), usage);
    }
}

impl Context {
    pub fn new_index_buffer<T: IndexDatum>(&self) -> IndexBuffer<T> {
        IndexBuffer {
            buffer: self.gen_buffer(),
            count: 0,
            phantom: PhantomData
        }
    }
}



#[macro_export]
macro_rules! attrib_pointers {
    ($gl:expr, $vbo:expr, {
        $($field_name:ident => $field_attrib:expr),*
    }) => {
        $vbo.build_attrib_binder()
            $(.$field_name($field_attrib))*
            .unwrap($gl);
    }
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

#[macro_export]
macro_rules! bind_index_buffer {
    ($gl:expr, $ibo:expr) => {
        {
            let ibo = $ibo;

            let gl_buffer = bind_element_array_buffer!($gl, ibo.buffer_mut());
            $crate::IndexBufferBinding::new(gl_buffer, ibo)
        }
    }
}
