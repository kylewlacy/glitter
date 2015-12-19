use std::marker::PhantomData;
use std::collections::{HashMap, HashSet};
use context::{ContextOf, AContext};
use framebuffer::FramebufferBinding;
use program::ProgramAttrib;
use vertex_data::{VertexData, VertexBytes, VertexAttribute};
use index_data::{IndexData, IndexDatum};
use buffer::{Buffer, BufferBinding,
             ArrayBufferBinding, ArrayBufferContext,
             ElementArrayBufferBinding, ElementArrayBufferContext};
use types::DrawingMode;

#[derive(Debug)]
pub enum AttribAddError {
    DuplicateAttrib(String)
}

#[derive(Debug)]
pub struct AttribError {
    missing_attribs: Vec<String>,
    unknown_attribs: Vec<String>
}

pub struct AttribBinder {
    attribs: HashMap<String, ProgramAttrib>
}

impl AttribBinder {
    pub fn new() -> Self {
        AttribBinder {
            attribs: HashMap::new()
        }
    }

    pub fn add(&mut self, name: &str, attrib: ProgramAttrib)
        -> Result<(), AttribAddError>
    {
        match self.attribs.insert(name.into(), attrib) {
            None => Ok(()),
            Some(_) => Err(AttribAddError::DuplicateAttrib(name.into()))
        }
    }

    fn for_each<T, F>(&self, mut f: F) -> Result<(), AttribError>
        where T: VertexData, F: FnMut(VertexAttribute, ProgramAttrib)
    {
        // TODO: Avoid heap allocations
        // TODO: Avoid redundant calls to T::visit_attributes
        let mut attribs =
            HashMap::<String, (VertexAttribute, ProgramAttrib)>::new();
        let mut missing = Vec::<String>::new();

        T::visit_attributes(|vertex_attrib| {
            match self.attribs.get(&vertex_attrib.name) {
                Some(program_attrib) => {
                    let pair = (vertex_attrib.clone(), *program_attrib);
                    attribs.insert(vertex_attrib.name, pair);
                },
                None => {
                    missing.push(vertex_attrib.name);
                }
            }
        });

        let unknown: Vec<_> = {
            let expected: HashSet<_> = self.attribs.keys().collect();
            let actual: HashSet<_> = attribs.keys().collect();
            expected.difference(&actual).cloned().cloned().collect()
        };

        if missing.is_empty() && unknown.is_empty() {
            for (_, (vertex_attrib, program_attrib)) in attribs.into_iter() {
                f(vertex_attrib, program_attrib);
            }
            Ok(())
        }
        else {
            Err(AttribError {
                missing_attribs: missing,
                unknown_attribs: unknown
            })
        }
    }

    pub fn enable<V, C>(&self, gl: &mut C) -> Result<(), AttribError>
        where V: VertexData, C: AContext
    {
        self.for_each::<V, _>(|_, program_attrib| {
            gl.enable_vertex_attrib_array(program_attrib);
        })
    }

    pub fn bind<V: VertexData>(&self, gl_buffer: &ArrayBufferBinding)
        -> Result<(), AttribError>
    {
        self.for_each::<V, _>(|vertex_attrib, program_attrib| {
            unsafe {
                gl_buffer.vertex_attrib_pointer(
                    program_attrib,
                    vertex_attrib.ty.components,
                    vertex_attrib.ty.data,
                    vertex_attrib.ty.normalize,
                    vertex_attrib.stride,
                    vertex_attrib.offset
                );
            }
        })
    }
}



#[derive(Debug)]
pub enum VertexBindError {
    BindingError(AttribError),
    NoAttributeBindings
}

impl From<AttribError> for VertexBindError {
    fn from(attrib_error: AttribError) -> VertexBindError {
        VertexBindError::BindingError(attrib_error)
    }
}

pub struct VertexBuffer<T: VertexData> {
    attrib_binder: Option<AttribBinder>,
    buffer: Buffer,
    count: usize,
    phantom: PhantomData<*const T>
}

impl<V: VertexData> VertexBuffer<V> {
    pub fn bind_attrib_pointers(&mut self, binder: AttribBinder) {
        self.attrib_binder = Some(binder);
    }

    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}

pub struct VertexBufferBinding<'a, T: VertexData + 'a> {
    gl_buffer: ArrayBufferBinding<'a>,
    count: &'a mut usize,
    _phantom: PhantomData<*const VertexBuffer<T>>
}

impl<'a, T: VertexData + 'a> VertexBufferBinding<'a, T> {
    pub fn buffer_data(&mut self, data: &[T], usage: super::BufferDataUsage)
        where [T]: VertexBytes
    {
        *self.count = data.len();
        self.gl_buffer.buffer_bytes(data.vertex_bytes(), usage);
    }
}

impl<'a> FramebufferBinding<'a> {
    pub fn draw_arrays_range_vbo<V>(&mut self,
                                    gl_vbo: &VertexBufferBinding<V>,
                                    mode: DrawingMode,
                                    start: u32,
                                    length: usize)
        where V: VertexData
    {
        debug_assert!((start as usize) + length <= *gl_vbo.count);

        unsafe {
            self.draw_arrays_range(&gl_vbo.gl_buffer, mode, start, length);
        }
    }

    pub fn draw_arrays_vbo<V>(&mut self,
                              gl_vbo: &VertexBufferBinding<V>,
                              mode: DrawingMode)
        where V: VertexData
    {
        unsafe {
            self.draw_arrays_range(&gl_vbo.gl_buffer,
                                   mode,
                                   0,
                                   *gl_vbo.count);
        }
    }

    pub fn draw_n_elements_buffered_vbo<V, I>(&mut self,
                                              gl_vbo: &VertexBufferBinding<V>,
                                              gl_ibo: &IndexBufferBinding<I>,
                                              mode: DrawingMode,
                                              length: usize)
        where V: VertexData, I: IndexDatum
    {
        debug_assert!(length <= *gl_ibo.count);

        unsafe {
            self.draw_n_elements_buffered(&gl_vbo.gl_buffer,
                                          &gl_ibo.gl_buffer,
                                          mode,
                                          length,
                                          I::index_datum_type());
        }
    }

    pub fn draw_elements_buffered_vbo<V, I>(&mut self,
                                            gl_vbo: &VertexBufferBinding<V>,
                                            gl_ibo: &IndexBufferBinding<I>,
                                            mode: DrawingMode)
        where V: VertexData, I: IndexDatum
    {
        unsafe {
            self.draw_n_elements_buffered(&gl_vbo.gl_buffer,
                                          &gl_ibo.gl_buffer,
                                          mode,
                                          *gl_ibo.count,
                                          I::index_datum_type());
        }
    }

    pub fn draw_n_elements_vbo<V, I>(&mut self,
                                     gl_vbo: &VertexBufferBinding<'a, V>,
                                     mode: DrawingMode,
                                     count: usize,
                                     indices: &[I])
        where V: VertexData, I: IndexDatum, [I]: IndexData
    {
        unsafe {
            self.draw_n_elements(&gl_vbo.gl_buffer, mode, count, indices);
        }
    }

    pub fn draw_elements_vbo<V, I>(&mut self,
                                   gl_vbo: &VertexBufferBinding<'a, V>,
                                   mode: DrawingMode,
                                   indices: &[I])
        where V: VertexData, I: IndexDatum, [I]: IndexData
    {
        unsafe {
            self.draw_elements(&gl_vbo.gl_buffer, mode, indices);
        }
    }
}

impl<B, F, P, R, T> ContextOf<B, F, P, R, T> {
    pub fn new_vertex_buffer<V: VertexData>(&self) -> VertexBuffer<V> {
        VertexBuffer {
            attrib_binder: None,
            buffer: self.gen_buffer(),
            count: 0,
            phantom: PhantomData
        }
    }
}

pub trait VertexBufferContext {
    type Rest;

    fn bind_vertex_buffer<'a, V>(self, vbo: &'a mut VertexBuffer<V>)
        -> (VertexBufferBinding<V>, Self::Rest)
        where V: VertexData;
}

impl<C> VertexBufferContext for C
    where C: ArrayBufferContext,
          C::Rest: AContext
{
    type Rest = C::Rest;

    fn bind_vertex_buffer<'a, V>(self, vbo: &'a mut VertexBuffer<V>)
        -> (VertexBufferBinding<V>, Self::Rest)
        where V: VertexData
    {
        // TODO: Cleanup error handling
        let (gl_array_buffer, rest) = match vbo.attrib_binder {
            Some(ref binder) => {
                let buf = &mut vbo.buffer;
                let (gl_buffer, mut rest) = self.bind_array_buffer(buf);
                binder.enable::<V, _>(&mut rest).unwrap();
                binder.bind::<V>(&gl_buffer).unwrap();
                (gl_buffer, rest)
            },
            None => {
                panic!("No attribute bindings provided for vertex buffer");
            }
        };

        (
            VertexBufferBinding {
                gl_buffer: gl_array_buffer,
                count: &mut vbo.count,
                _phantom: PhantomData
            },
            rest
        )
    }
}

pub trait IndexBufferContext {
    type Rest;

    fn bind_index_buffer<'a, I>(self, ibo: &'a mut IndexBuffer<I>)
        -> (IndexBufferBinding<I>, Self::Rest)
        where I: IndexDatum;
}

impl<C> IndexBufferContext for C
    where C: ElementArrayBufferContext
{
    type Rest = C::Rest;

    fn bind_index_buffer<'a, I>(self, ibo: &'a mut IndexBuffer<I>)
        -> (IndexBufferBinding<I>, Self::Rest)
        where I: IndexDatum
    {
        let (gl_be, rest) = self.bind_element_array_buffer(&mut ibo.buffer);
        (
            IndexBufferBinding {
                gl_buffer: gl_be,
                count: &mut ibo.count,
                _phantom: PhantomData
            },
            rest
        )
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
    count: &'a mut usize,
    _phantom: PhantomData<*const IndexBuffer<T>>
}

impl<'a, T: IndexDatum + 'a> IndexBufferBinding<'a, T> {
    pub fn buffer_data(&mut self, data: &[T], usage: super::BufferDataUsage)
        where [T]: IndexData
    {
        *self.count = data.len();
        self.gl_buffer.buffer_bytes(data.index_bytes(), usage);
    }
}

impl<B, F, P, R, T> ContextOf<B, F, P, R, T> {
    pub fn new_index_buffer<I: IndexDatum>(&self) -> IndexBuffer<I> {
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
        {
            let mut binder = $crate::AttribBinder::new();
            $(binder.add(stringify!($field_name), $field_attrib).unwrap());*;
            binder
        }
    }
}

#[macro_export]
macro_rules! bind_attrib_pointers {
    ($gl:expr, $vbo:expr, {
        $($field_name:ident => $field_attrib:expr),*
    }) => {
        {
            let vbo = $vbo;
            let binder = {
                attrib_pointers!($gl, vbo, {
                    $($field_name => $field_attrib),*
                })
            };
            vbo.bind_attrib_pointers(binder);
        }
    }
}

#[macro_export]
macro_rules! bind_vertex_buffer {
    ($gl:expr, $vbo:expr) => {
        {
            use std::borrow::BorrowMut;
            let mut vbo = $vbo;

            let (gl_vbo, _) = $gl.bind_vertex_buffer(vbo.borrow_mut());
            gl_vbo
        }
    }
}

#[macro_export]
macro_rules! bind_index_buffer {
    ($gl:expr, $ibo:expr) => {
        {
            use std::borrow::BorrowMut;
            let mut ibo = $ibo;

            let (gl_ibo, _) = $gl.bind_index_buffer(ibo.borrow_mut());
            gl_ibo
        }
    }
}
