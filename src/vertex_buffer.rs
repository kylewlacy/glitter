//! Contains a higher-level abstraction for creating vertex and index
//! buffer.

use std::marker::PhantomData;
use std::collections::{HashMap, HashSet};
use context::{ContextOf, AContext, ContextBufferExt,
              ArrayBufferBinding, ArrayBufferContext,
              ElementArrayBufferBinding, ElementArrayBufferContext};
use program::ProgramAttrib;
use vertex_data::{VertexData, VertexBytes, VertexAttribute};
use index_data::{IndexData, IndexDatum};
use buffer::Buffer;
use types::DrawingMode;

/// An error generated when trying add an attribute to an [`AttribBinder`]
/// (struct.AttribBinder.html) using the [`AttribBinder::add`]
/// (struct.AttribBinder.html#method.add) method.
#[derive(Debug)]
pub enum AttribAddError {
    /// An attribute was added that already exists in the [`AttribBinder`]
    /// (struct.AttribBinder.html).
    DuplicateAttrib(String)
}

/// An error generated when trying to iterate over the lists of attributes
/// in an [`AttribBinder`](struct.AttribBinder.html).
#[derive(Debug)]
pub struct AttribError {
    missing_attribs: Vec<String>,
    unknown_attribs: Vec<String>
}

/// A type used to perform operations on a list of program attributes. An
/// instance of an `AttribBinder` can be created with the [`attrib_pointers!`]
/// (../macro.attrib_pointers!.html) macro or with the [`new`]
/// (struct.AttribBinder.html#method.new) method.
///
/// # Note
/// Currently, `AttribBinder` when both adding vertex attributes and when
/// operating on vertex attributes. Consider using the direct lower-level
/// glitter API's if heap allocations become a performance bottleneck.
pub struct AttribBinder {
    attribs: HashMap<String, ProgramAttrib>
}

impl AttribBinder {
    /// Create a new, empty `AttribBinder`.
    pub fn new() -> Self {
        AttribBinder {
            attribs: HashMap::new()
        }
    }

    /// Add an attribute to the `AttribBinder`.
    ///
    /// # Failures
    /// `add` will return an error if the attribute being added is already
    /// present.
    ///
    /// # Note
    /// Each call to `add` can potentially cause a heap allocation.
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

    /// Enable all of the vertex attributes contained within the `AttribBinder`.
    /// The provided type of `VertexData` is used to to verify the list
    /// of attributes.
    ///
    /// # Failures
    /// An error will be returned if the set of vertex attributes contained
    /// by the `VertexData` types does not exactly match the list of attributes
    /// contained by the `AttribBinder`.
    ///
    /// # Note
    /// Currently, calling `enable` will perform a heap allocation. Consider
    /// using [`gl.enable_vertex_attrib_array`]
    /// (../context/trait.ContextExt.html#method.enable_vertex_attrib_array)
    /// if this becomes a performance concern.
    pub fn enable<V, C>(&self, gl: &mut C) -> Result<(), AttribError>
        where V: VertexData, C: AContext
    {
        // TODO: Use a plain `for` loop? Do we actually want the `V` parameter?
        //       Do we actually *only* want the `V` parameter?
        self.for_each::<V, _>(|_, program_attrib| {
            gl.enable_vertex_attrib_array(program_attrib);
        })
    }

    /// Set up each vertex attribute with the appropriate attribute options
    /// (using [`glVertexAttribPointer`]
    /// (http://docs.gl/es2/glVertexAttribPointer)). The `VertexData` type
    /// parameter is used to get the attribute options for each attribute.
    ///
    /// # Failures
    /// An error will be returned if the set of vertex attributes contained
    /// by the `VertexData` types does not exactly match the list of attributes
    /// contained by the `AttribBinder.`
    ///
    /// # Note
    /// Currently, calling `bind` will perform a heap allocation. Consider
    /// using [`gl.enable_vertex_attrib_array`]
    /// (../context/trait.ContextExt.html#method.enable_vertex_attrib_array)
    /// if this becomes a performance concern.
    pub fn bind<V, C>(&self, gl: &C) -> Result<(), AttribError>
        where V: VertexData, C: AContext
    {
        self.for_each::<V, _>(|vertex_attrib, program_attrib| {
            unsafe {
                // TODO: Refactor!
                // (Make vertex_attrib_pointer take vertex_attrib)
                gl.vertex_attrib_pointer(program_attrib,
                                         vertex_attrib.ty.components,
                                         vertex_attrib.ty.data,
                                         vertex_attrib.ty.normalize,
                                         vertex_attrib.stride,
                                         vertex_attrib.offset);
            }
        })
    }
}



/// An error generated when binding vertex attributes.
#[derive(Debug)]
pub enum VertexBindError {
    /// An `AttribError` that was generated
    BindingError(AttribError),

    /// No attribute bindings were provided.
    NoAttributeBindings
}

impl From<AttribError> for VertexBindError {
    fn from(attrib_error: AttribError) -> VertexBindError {
        VertexBindError::BindingError(attrib_error)
    }
}

/// A buffer that contains vertex data. In addition to storing a buffer,
/// a `VertexBuffer` stores an [`AttribBinder`](struct.AttribBinder.html)
/// and a count of the amount of `VertexData` that has been buffered.
pub struct VertexBuffer<T: VertexData> {
    attrib_binder: Option<AttribBinder>,
    buffer: Buffer,
    count: usize,
    phantom: PhantomData<*const T>
}

impl<V: VertexData> VertexBuffer<V> {
    /// Set the `AttribBinder` that will contain all of the vertex attributes
    /// used when rendering.
    pub fn bind_attrib_pointers(&mut self, binder: AttribBinder) {
        self.attrib_binder = Some(binder);
    }

    /// Get a reference to underlying OpenGL buffer.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get a mutable reference to the underlying OpenGL buffer.
    // TODO: Is this safe?
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}

/// Represents a [`VertexBuffer`](struct.VertexBuffer.html) that has
/// been bound to a context.
pub struct VertexBufferBinding<'a, T: VertexData + 'a> {
    gl_buffer: ArrayBufferBinding<'a>,
    count: &'a mut usize,
    _phantom: PhantomData<*const VertexBuffer<T>>
}

/// An extension trait that adds vertex buffer and index buffer-related methods
/// to OpenGL contexts.
pub trait ContextVertexBufferExt: AContext {
    /// Create a new, empty vertex buffer.
    fn new_vertex_buffer<V: VertexData>(&self) -> VertexBuffer<V> {
        VertexBuffer {
            attrib_binder: None,
            buffer: self.gen_buffer(),
            count: 0,
            phantom: PhantomData
        }
    }

    /// Send data to a vertex buffer. Note that this will replace the buffer's
    /// current contents, if any.
    fn buffer_vertices<T>(&self,
                          gl_vbo: &mut VertexBufferBinding<T>,
                          vertices: &[T],
                          usage: super::BufferDataUsage)
        where T: VertexData, [T]: VertexBytes
    {

        *gl_vbo.count = vertices.len();
        self.buffer_bytes(&mut gl_vbo.gl_buffer,
                          vertices.vertex_bytes(),
                          usage);
    }

    /// Send data to an index buffer. Note that this will replace the buffer's
    /// current contents, if any.
    fn buffer_indices<T>(&self,
                         gl_ibo: &mut IndexBufferBinding<T>,
                         indices: &[T],
                         usage: super::BufferDataUsage)
        where T: IndexDatum, [T]: IndexData
    {
        *gl_ibo.count = indices.len();
        self.buffer_bytes(&mut gl_ibo.gl_buffer, indices.index_bytes(), usage);
    }

    /// Use the data from the provided vertex buffer binding to render
    /// primitives.
    ///
    /// - `gl_vbo`: The binding of the vertex buffer to read vertices from.
    /// - `mode`: The type of primitives to draw.
    /// - `start`: The index of the first vertex to draw.
    /// - `length`: The number of vertices to draw.
    ///
    /// # Panics
    /// This function will panic if the `start` and `length` are out
    /// of bounds of the currently-buffered data.
    fn draw_arrays_range_vbo<V>(&self,
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

    /// Use the data from the provided vertex buffer binding to render
    /// primitives. This function will use the full range of vertices
    /// that have been buffered.
    ///
    /// - `gl_vbo`: The binding of the vertex buffer to read vertices from.
    /// - `mode`: The type of primitives to draw.
    fn draw_arrays_vbo<V>(&self,
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

    /// Draw primitives using the provided index buffer as
    /// the indices into the provided vertex buffer.
    ///
    /// - `gl_vbo`: The binding of the buffer that contains the vertex data.
    /// - `gl_ibo`: The binding of the buffer that contains the index data.
    /// - `mode`: The type of primitives to draw.
    /// - `length`: The number of indices to read.
    fn draw_n_elements_buffered_vbo<V, I>(&self,
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

    /// Draw primitives using the provided index buffer as
    /// the indices into the provided vertex buffer. All buffered
    /// indices will be used.
    ///
    /// - `gl_vbo`: The binding of the buffer that contains the vertex data.
    /// - `gl_ibo`: The binding of the buffer that contains the index data.
    /// - `mode`: The type of primitives to draw.
    fn draw_elements_buffered_vbo<V, I>(&self,
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

    /// Draw primitives specified by the provided index array,
    /// treated as indices into the provided vertex buffer.
    ///
    /// - `gl_vbo`: The binding of the buffer that contains the vertex data.
    /// - `mode`: The type of primitives to draw.
    /// - `count`: The number of indices to read.
    /// - `indices`: The index array to use.
    fn draw_n_elements_vbo<V, I>(&self,
                                 gl_vbo: &VertexBufferBinding<V>,
                                 mode: DrawingMode,
                                 count: usize,
                                 indices: &[I])
        where V: VertexData, I: IndexDatum, [I]: IndexData
    {
        unsafe {
            self.draw_n_elements(&gl_vbo.gl_buffer, mode, count, indices);
        }
    }

    /// Draw primitives specified by the provided index array,
    /// treated as indices into the provided vertex buffer.
    ///
    /// - `gl_vbo`: The binding of the buffer that contains the vertex data.
    /// - `mode`: The type of primitives to draw.
    /// - `indices`: The index array to use.
    fn draw_elements_vbo<V, I>(&mut self,
                               gl_vbo: &VertexBufferBinding<V>,
                               mode: DrawingMode,
                               indices: &[I])
        where V: VertexData, I: IndexDatum, [I]: IndexData
    {
        unsafe {
            self.draw_elements(&gl_vbo.gl_buffer, mode, indices);
        }
    }
}

impl<C: AContext> ContextVertexBufferExt for C {

}



/// An OpenGL context that can have a vertex buffer bound.
///
/// # Note
/// Internally, vertex buffers are bound to the `GL_ARRAY_BUFFER`
/// binding, so any context that has a free `GL_ARRAY_BUFFER` is
/// a `VertexBufferContext`.
pub trait VertexBufferContext: ArrayBufferContext + Sized {
    /// Bind a vertex buffer to this context, returning a binding
    /// and a new context.
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
                binder.bind::<V, _>(&rest).unwrap();
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

impl<C: ArrayBufferContext> VertexBufferContext for C {

}



/// An OpenGL context that can have an index buffer bound.
///
/// # Note
/// Internally, index buffers are bound to the `GL_ELEMENT_ARRAY_BUFFER`
/// binding, so any context that has a free `GL_ELEMENT_ARRAY_BUFFER` is
/// an `IndexBufferContext`.
pub trait IndexBufferContext: ElementArrayBufferContext + Sized {
    /// Bind an index buffer to this context, returning a binding
    /// and the remaining context.
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

impl<C: ElementArrayBufferContext> IndexBufferContext for C {

}



/// A buffer that contains index data. In addition to storing a buffer, an
/// `IndexBuffer` store a count of the amount of `IndexData` that has been
/// buffered.
pub struct IndexBuffer<T: IndexDatum> {
    buffer: Buffer,
    count: usize,
    phantom: PhantomData<*const T>
}

impl<T: IndexDatum> IndexBuffer<T> {
    /// Get a reference to the underlying OpenGL buffer.
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get a mutable reference to the underlying OpenGL buffer.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }
}

/// Represents an [`IndexBuffer`](struct.IndexBuffer.html) that has been
/// bound to a context.
pub struct IndexBufferBinding<'a, T: IndexDatum + 'a> {
    gl_buffer: ElementArrayBufferBinding<'a>,
    count: &'a mut usize,
    _phantom: PhantomData<*const IndexBuffer<T>>
}

impl<B, F, P, R, T> ContextOf<B, F, P, R, T> {
    /// Create a new, empty index buffer.
    pub fn new_index_buffer<I: IndexDatum>(&self) -> IndexBuffer<I> {
        IndexBuffer {
            buffer: self.gen_buffer(),
            count: 0,
            phantom: PhantomData
        }
    }
}



/// Create an [`AttribBinder`](vertex_buffer/struct.AttribBinder.html) from
/// a set of associations from vertex attribute names to [`ProgramAttribs`]
/// (program/struct.ProgramAttrib.html).
///
/// # Examples
///
/// ```no_run
/// # #[macro_use] extern crate glitter;
/// # use glitter::prelude::*;
/// # fn main() {
/// # let gl = unsafe { glitter::Context::current_context() };
/// # let program: glitter::Program = unsafe { ::std::mem::uninitialized() };
/// // Create an `AttribBinder`, where "position" and "color" are set
/// // to their respective program attributes.
/// let attribs = attrib_pointers! {
///    position => gl.get_attrib_location(&program, "position").unwrap(),
///    color => gl.get_attrib_location(&program, "color").unwrap()
/// };
/// # }
/// ```
#[macro_export]
macro_rules! attrib_pointers {
    ($($field_name:ident => $field_attrib:expr),*) => {
        {
            let mut binder = $crate::AttribBinder::new();
            $(binder.add(stringify!($field_name), $field_attrib).unwrap());*;
            binder
        }
    }
}
