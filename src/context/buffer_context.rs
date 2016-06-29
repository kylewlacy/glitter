//! Contains all of the OpenGL state types related to buffer objects.

use std::mem;
use std::ptr;
use std::marker::PhantomData;
use std::borrow::BorrowMut;
use gl;
use gl::types::*;
use context::{ContextOf, BaseContext, AContext};
use buffer::{Buffer, BufferDataUsage, BufferBindingTarget};
use program::{ProgramAttrib};
use index_data::{IndexData, IndexDatum, IndexDatumType};
use types::{DataType, DrawingMode, GLObject, GLError};
use to_ref::{ToRef, ToMut};

unsafe fn _draw_elements(mode: DrawingMode,
                         count: usize,
                         index_type: IndexDatumType,
                         indices: *const GLvoid)
{
    let gl_index_type: GLenum = match index_type {
        IndexDatumType::UnsignedByte => gl::UNSIGNED_BYTE,
        IndexDatumType::UnsignedShort => gl::UNSIGNED_SHORT
    };
    gl::DrawElements(mode.gl_enum(),
                     count as GLsizei,
                     gl_index_type,
                     indices);
    dbg_gl_error! {
        GLError::InvalidEnum => "`mode` or `type` is not an accepted value",
        GLError::InvalidValue => "`count` is negative",
        GLError::InvalidFramebufferOperation => "The current framebuffer is not framebuffer-complete",
        _ => "Unknown error"
    }
}

fn _bind_buffer(target: BufferBindingTarget, buffer: &mut Buffer) {
    unsafe {
        gl::BindBuffer(target as GLuint, buffer.id());
        dbg_gl_sanity_check! {
            GLError::InvalidEnum => "`target` is not an allowed value",
            _ => "Unknown error"
        }
    }
}

/// An extension trait that includes buffer-object-related OpenGL methods.
pub trait ContextBufferExt: BaseContext {
    /// Create a new, empty OpenGL buffer object.
    ///
    /// # See also
    /// [`glGenBuffers`](http://docs.gl/es2/glGenBuffers) OpenGL docs
    ///
    /// [`gl.new_vertex_buffer`](../../vertex_buffer/trait.ContextVertexBufferExt.html#method.new_vertex_buffer):
    /// Create a new vertex buffer, which wraps a raw OpenGL buffer.
    ///
    /// [`gl.new_index_buffer`](../struct.ContextOf.html#method.new_index_buffer):
    /// Create a new index buffer, which wraps a raw OpenGL buffer.
    ///
    /// [`gl.buffer_bytes`](trait.ContextBufferExt.html#method.buffer_bytes):
    /// Send data to a buffer.
    fn gen_buffer(&self) -> Buffer {
        let mut id : GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut id as *mut GLuint);
        }
        dbg_gl_sanity_check! {
            GLError::InvalidValue => "`n` is negative",
            _ => "Unknown error"
        }

        unsafe { Buffer::from_raw(id) }
    }

    /// Send data to a buffer object. Note that this will replace the buffer's
    /// current contents, if any.
    ///
    /// # See also
    /// [`glBufferData`](http://docs.gl/es2/glBufferData) OpenGL docs
    fn buffer_bytes<B>(&self,
                       gl_buffer: &mut B,
                       bytes: &[u8],
                       usage: BufferDataUsage)
        where B: BufferBinding
    {
        let ptr = bytes.as_ptr();
        let size = bytes.len() * mem::size_of::<u8>();
        unsafe {
            gl::BufferData(gl_buffer.target().gl_enum(),
                           size as GLsizeiptr,
                           ptr as *const GLvoid,
                           usage.gl_enum());
            dbg_gl_error! {
                GLError::InvalidEnum => "Invalid `target` or `usage`",
                GLError::InvalidValue => "`size` is negative",
                GLError::InvalidOperation => "Object 0 is bound to buffer target",
                GLError::OutOfMemory => "Unable to create a large enough buffer",
                _ => "Unknown error"
            }
        }
    }

    /// Specify how an array of vertex data will be treated while rendering.
    /// Most uses of this function can be replaced by using a [`VertexBuffer`]
    /// (../../vertex_buffer/struct.VertexBuffer.html), which provides a nicer
    /// interface for setting up vertex attributes.
    ///
    /// # Panics
    /// This function will panic in debug mode if `components` is less than 1 or
    /// greater than 4.
    ///
    /// # Safety
    /// Using this function can cause an OpenGL draw call to read uninitialized
    /// memory from a buffer.
    ///
    /// # See also
    /// [`glVertexAttribPointer`](http://docs.gl/es2/glVertexAttribPointer) OpenGL docs
    unsafe fn vertex_attrib_pointer(&self,
                                    attrib: ProgramAttrib,
                                    components: i8,
                                    gl_type: DataType,
                                    normalized: bool,
                                    stride: usize,
                                    offset: usize)
    {
        debug_assert!(1 <= components && components <= 4);

        let gl_normalized = if normalized { gl::TRUE } else { gl::FALSE };
        gl::VertexAttribPointer(attrib.gl_index,
                                components as GLint,
                                gl_type.gl_enum(),
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

    /// Use the vertex data from the provided array buffer binding to render
    /// primitives.
    ///
    /// - `_ab`: The binding for the array buffer to read vertices from.
    /// - `mode`: The type of primitives to draw.
    /// - `first`: The index of the first vertex to read.
    /// - `count`: The number of vertices to read.
    ///
    /// # Safety
    /// The vertex attributes for the need to be set up before calling this
    /// method by using the [`gl.vertex_attrib_pointer`]
    /// (trait.ContextBufferExt.html#method.vertex_attrib_pointer) method.
    ///
    /// # See also
    /// [`glDrawArrays`](http://docs.gl/es2/glDrawArrays) OpenGL docs
    unsafe fn draw_arrays_range(&self,
                                _ab: &ArrayBufferBinding,
                                mode: DrawingMode,
                                first: u32,
                                count: usize)
    {
        gl::DrawArrays(mode.gl_enum(), first as GLint, count as GLsizei);
        dbg_gl_sanity_check! {
            GLError::InvalidEnum => "`mode` is not an accepted value",
            GLError::InvalidValue => "`count` is negative",
            _ => "Unknown error"
        }
    }

    /// Draw primitives specified by the provided element array buffer, treated
    /// as indices of the vertices from the provided array buffer.
    ///
    /// - `_ab`: The binding for the array buffer that contains the vertex
    ///          data.
    /// - `_eab`: The binding for the element array buffer that contains the
    ///           index data.
    /// - `mode`: The type of primitives to draw.
    /// - `count`: The number of indices to read.
    /// - `index_type`: Specifies the data type of the index (whether it is
    ///                 a byte or short, signed unsigned, etc).
    ///
    /// # See also
    /// [`glDrawElements`](http://docs.gl/es2/glDrawElements) OpenGL docs
    unsafe fn draw_n_elements_buffered(&self,
                                       _ab: &ArrayBufferBinding,
                                       _eab: &ElementArrayBufferBinding,
                                       mode: DrawingMode,
                                       count: usize,
                                       index_type: IndexDatumType)
    {
        _draw_elements(mode, count, index_type, ptr::null());
    }

    /// Draw primitives specified by the provided index array, treated as
    /// indices of the vertices from the provided array buffer.
    ///
    /// - `_ab`: The binding for the array buffer that contains the vertex
    ///          data.
    /// - `mode`: The type of primitives to draw.
    /// - `count`: The number of indices to read.
    /// - `indices`: The index array to use.
    ///
    /// # See also
    /// [`glDrawElements`](http://docs.gl/es2/glDrawElements) OpenGL docs
    unsafe fn draw_n_elements<I>(&self,
                                 _ab: &ArrayBufferBinding,
                                 mode: DrawingMode,
                                 count: usize,
                                 indices: &[I])
        where I: IndexDatum, [I]: IndexData
    {
        debug_assert!(count <= indices.len());

        let ptr = indices.index_bytes().as_ptr();
        let index_type = I::index_datum_type();
        _draw_elements(mode, count, index_type, mem::transmute(ptr));
    }

    /// Draw primitives specified by the provided index array, treated as
    /// indices of the vertices from the provided array buffer.
    ///
    /// - `_ab`: The binding for the array buffer that contains the vertex
    ///          data.
    /// - `mode`: The type of primitives to draw.
    /// - `indices`: The index array to use.
    ///
    /// # See also
    /// [`glDrawElements`](http://docs.gl/es2/glDrawElements) OpenGL docs
    unsafe fn draw_elements<I>(&self,
                               _ab: &ArrayBufferBinding,
                               mode: DrawingMode,
                               indices: &[I])
        where I: IndexDatum, [I]: IndexData
    {
        self.draw_n_elements(_ab, mode, indices.len(), indices);
    }
}

impl<C: BaseContext> ContextBufferExt for C {

}



/// An OpenGL context that has a free `GL_ARRAY_BUFFER` binding.
pub trait ArrayBufferContext: AContext {
    /// The type of binder this context contains.
    type Binder: BorrowMut<ArrayBufferBinder>;

    /// The OpenGL context that will be returned after binding the array buffer.
    type Rest: AContext;

    /// Split this context into a binder and the remaining context.
    fn split_array_buffer(self) -> (Self::Binder, Self::Rest);

    /// Bind a buffer to this context's array buffer, returning
    /// a new context and a binding.
    ///
    /// # See also
    /// [`glBindBuffer`](http://docs.gl/es2/glBindBuffer) OpenGL docs
    fn bind_array_buffer<'a>(self, buffer: &'a mut Buffer)
        -> (ArrayBufferBinding<'a>, Self::Rest)
        where Self: Sized
    {
        let (mut binder, rest) = self.split_array_buffer();
        (binder.borrow_mut().bind(buffer), rest)
    }
}

/// An OpenGL context that has a free `GL_ELEMENT_ARRAY_BUFFER` binding.
pub trait ElementArrayBufferContext: AContext {
    /// The type of binder this context contains.
    type Binder: BorrowMut<ElementArrayBufferBinder>;

    /// The OpenGL context that will be returned after binding the element
    /// array buffer.
    type Rest: AContext;

    /// Split this context into a binder and the remaining context.
    fn split_element_array_buffer(self) -> (Self::Binder, Self::Rest);

    /// Bind a buffer to this context's element array buffer, returning
    /// a new context and a binding.
    ///
    /// # See also
    /// [`glBindBuffer`](http://docs.gl/es2/glBindBuffer) OpenGL docs
    fn bind_element_array_buffer<'a>(self, buffer: &'a mut Buffer)
        -> (ElementArrayBufferBinding<'a>, Self::Rest)
        where Self: Sized
    {
        let (mut binder, rest) = self.split_element_array_buffer();
        (binder.borrow_mut().bind(buffer), rest)
    }
}

impl<BA, BE, F, P, R, T> ArrayBufferContext
    for ContextOf<BufferBinderOf<BA, BE>, F, P, R, T>
    where BA: BorrowMut<ArrayBufferBinder>
{
    type Binder = BA;
    type Rest = ContextOf<BufferBinderOf<(), BE>, F, P, R, T>;

    fn split_array_buffer(self) -> (Self::Binder, Self::Rest) {
        let (buffers, gl) = self.swap_buffers(());
        let (binder, rest_buffers) = buffers.swap_array(());
        let ((), gl) = gl.swap_buffers(rest_buffers);

        (binder, gl)
    }
}

impl<'a, BA, BE, F, P, R, T> ArrayBufferContext
    for &'a mut ContextOf<BufferBinderOf<BA, BE>, F, P, R, T>
    where BA: BorrowMut<ArrayBufferBinder>
{
    type Binder = &'a mut ArrayBufferBinder;
    type Rest = ContextOf<BufferBinderOf<(), &'a mut BE>,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          &'a mut T>;

    fn split_array_buffer(self) -> (Self::Binder, Self::Rest) {
        let gl = self.borrowed_mut();
        let (buffers, gl) = gl.swap_buffers(());
        let buffers = buffers.borrowed_mut();
        let (binder, rest_buffers) = buffers.swap_array(());
        let ((), gl) = gl.swap_buffers(rest_buffers);

        (binder, gl)
    }
}

impl<'a, BA, BE, F, P, R, T> ArrayBufferContext
    for &'a mut ContextOf<&'a mut BufferBinderOf<BA, BE>,
                          F,
                          P,
                          R,
                          T>
    where BA: BorrowMut<ArrayBufferBinder>,
          F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>, T: ToMut<'a>
{
    type Binder = &'a mut ArrayBufferBinder;
    type Rest = ContextOf<BufferBinderOf<(), &'a mut BE>,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          T::Mut>;

    fn split_array_buffer(self) -> (Self::Binder, Self::Rest) {
        let gl = self.to_mut();
        let (buffers, gl): (&mut BufferBinderOf<_, _>, _) = gl.swap_buffers(());
        let buffers = buffers.borrowed_mut();
        let (binder, rest_buffers) = buffers.swap_array(());
        let ((), gl) = gl.swap_buffers(rest_buffers);

        (binder, gl)
    }
}

impl<BA, BE, F, P, R, T> ElementArrayBufferContext
    for ContextOf<BufferBinderOf<BA, BE>, F, P, R, T>
    where BE: BorrowMut<ElementArrayBufferBinder>
{
    type Binder = BE;
    type Rest = ContextOf<BufferBinderOf<BA, ()>, F, P, R, T>;

    fn split_element_array_buffer(self) -> (Self::Binder, Self::Rest) {
        let (buffers, gl) = self.swap_buffers(());
        let (binder, rest_buffers) = buffers.swap_element_array(());
        let ((), gl) = gl.swap_buffers(rest_buffers);

        (binder, gl)
    }
}

impl<'a, BA, BE, F, P, R, T> ElementArrayBufferContext
    for &'a mut ContextOf<BufferBinderOf<BA, BE>, F, P, R, T>
    where BE: BorrowMut<ElementArrayBufferBinder>
{
    type Binder = &'a mut ElementArrayBufferBinder;
    type Rest = ContextOf<BufferBinderOf<&'a mut BA, ()>,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          &'a mut T>;

    fn split_element_array_buffer(self) -> (Self::Binder, Self::Rest) {
        let gl = self.borrowed_mut();
        let (buffers, gl) = gl.swap_buffers(());
        let buffers = buffers.borrowed_mut();
        let (binder, rest_buffers) = buffers.swap_element_array(());
        let ((), gl) = gl.swap_buffers(rest_buffers);

        (binder, gl)
    }
}

impl<'a, BA, BE, F, P, R, T> ElementArrayBufferContext
    for &'a mut ContextOf<&'a mut BufferBinderOf<BA, BE>,
                          F,
                          P,
                          R,
                          T>
    where BE: BorrowMut<ElementArrayBufferBinder>,
    F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>, T: ToMut<'a>
{
    type Binder = &'a mut ElementArrayBufferBinder;
    type Rest = ContextOf<BufferBinderOf<&'a mut BA, ()>,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          T::Mut>;

    fn split_element_array_buffer(self) -> (Self::Binder, Self::Rest) {
        let gl = self.to_mut();
        let (buffers, gl): (&mut BufferBinderOf<_, _>, _) = gl.swap_buffers(());
        let buffers = buffers.borrowed_mut();
        let (binder, rest_buffers) = buffers.swap_element_array(());
        let ((), gl) = gl.swap_buffers(rest_buffers);

        (binder, gl)
    }
}



/// An OpenGL context that has all free buffer bindings. This trait implies
/// both [`ArrayBufferContext`](trait.ArrayBufferContext.html) and
/// [`ElementArrayBufferContext`](trait.ElementArrayBufferContext.html).
pub trait BufferContext: ArrayBufferContext + ElementArrayBufferContext {

}

impl<BA, BE, F, P, R, T> BufferContext
    for ContextOf<BufferBinderOf<BA, BE>, F, P, R, T>
    where BA: BorrowMut<ArrayBufferBinder>,
          BE: BorrowMut<ElementArrayBufferBinder>
{

}

impl<'a, BA, BE, F, P, R, T> BufferContext
    for &'a mut ContextOf<BufferBinderOf<BA, BE>, F, P, R, T>
    where BA: BorrowMut<ArrayBufferBinder>,
          BE: BorrowMut<ElementArrayBufferBinder>,
          F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>, T: ToMut<'a>
{

}

impl<'a, BA, BE, F, P, R, T> BufferContext
    for &'a mut ContextOf<&'a mut BufferBinderOf<BA, BE>,
                          F,
                          P,
                          R,
                          T>
    where BA: BorrowMut<ArrayBufferBinder>,
          BE: BorrowMut<ElementArrayBufferBinder>,
          F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>, T: ToMut<'a>
{

}

/// A buffer that has been bound to an OpenGL buffer binding point.
pub trait BufferBinding {
    /// Returns the OpenGL binding target that this buffer binding
    /// references.
    fn target(&self) -> BufferBindingTarget;
}

/// Represents a buffer that has been bound to the `GL_ARRAY_BUFFER`
/// binding target.
pub struct ArrayBufferBinding<'a> {
    _phantom_ref: PhantomData<&'a mut Buffer>,
    _phantom_ptr: PhantomData<*mut ()>
}

impl<'a> BufferBinding for ArrayBufferBinding<'a> {
    fn target(&self) -> BufferBindingTarget {
        BufferBindingTarget::ArrayBuffer
    }
}

/// Represents a buffer that has been bound to the `GL_ELEMENT_ARRAY_BUFFER`
/// binding target.
pub struct ElementArrayBufferBinding<'a> {
    _phantom_ref: PhantomData<&'a mut Buffer>,
    _phantom_ptr: PhantomData<*mut ()>
}

impl<'a> BufferBinding for ElementArrayBufferBinding<'a> {
    fn target(&self) -> BufferBindingTarget {
        BufferBindingTarget::ElementArrayBuffer
    }
}



/// This type holds all of the OpenGL-state-related buffer objects. See the
/// [`ContextOf`](../struct.ContextOf.html) docs for more details.
pub struct BufferBinderOf<A, E> {
    array: A,
    element_array: E,
    _phantom: PhantomData<*mut ()>
}

/// A part of the OpenGL context that has all free buffer bindings.
pub type BufferBinder = BufferBinderOf<ArrayBufferBinder,
                                       ElementArrayBufferBinder>;

impl<A, E> BufferBinderOf<A, E> {
    /// Get the current buffer-object binders.
    ///
    /// # Safety
    /// The same rules apply to this method as the
    /// [`ContextOf::current_context()` method]
    /// (../struct.ContextOf.html#method.current_context).
    pub unsafe fn current() -> BufferBinder {
        BufferBinderOf {
            array: ArrayBufferBinder::current(),
            element_array: ElementArrayBufferBinder::current(),
            _phantom: PhantomData
        }
    }

    fn borrowed_mut<'a, BA, BE>(&'a mut self)
        -> BufferBinderOf<&'a mut BA, &'a mut BE>
        where A: BorrowMut<BA>,
              E: BorrowMut<BE>
    {
        BufferBinderOf {
            array: self.array.borrow_mut(),
            element_array: self.element_array.borrow_mut(),
            _phantom: PhantomData
        }
    }

    /// Replace the array-buffer-related context with a new value, returning
    /// the old value and a new buffer context.
    pub fn swap_array<NA>(self, new_array: NA)
        -> (A, BufferBinderOf<NA, E>)
    {
        (
            self.array,
            BufferBinderOf {
                array: new_array,
                element_array: self.element_array,
                _phantom: PhantomData
            }
        )
    }

    /// Replace the element-array-buffer-related context with a new value,
    /// returning the old value and a new buffer context.
    pub fn swap_element_array<NE>(self, new_element_array: NE)
        -> (E, BufferBinderOf<A, NE>)
    {
        (
            self.element_array,
            BufferBinderOf {
                array: self.array,
                element_array: new_element_array,
                _phantom: PhantomData
            }
        )
    }
}

impl<'a, A, E> ToRef<'a> for BufferBinderOf<A, E>
    where A: 'a + ToRef<'a>, E: 'a + ToRef<'a>
{
    type Ref = BufferBinderOf<A::Ref, E::Ref>;

    fn to_ref(&'a self) -> Self::Ref {
        BufferBinderOf {
            array: self.array.to_ref(),
            element_array: self.element_array.to_ref(),
            _phantom: PhantomData
        }
    }
}

impl<'a, A, E> ToMut<'a> for BufferBinderOf<A, E>
    where A: 'a + ToMut<'a>, E: 'a + ToMut<'a>
{
    type Mut = BufferBinderOf<A::Mut, E::Mut>;

    fn to_mut(&'a mut self) -> Self::Mut {
        BufferBinderOf {
            array: self.array.to_mut(),
            element_array: self.element_array.to_mut(),
            _phantom: PhantomData
        }
    }
}



/// The OpenGL state representing the `GL_ARRAY_BUFFER` target.
pub struct ArrayBufferBinder {
    _phantom: PhantomData<*mut ()>
}

impl ArrayBufferBinder {
    /// Get the current `GL_ARRAY_BUFFER` binder.
    ///
    /// # Safety
    /// The same rules apply to this method as the
    /// [`ContextOf::current_context()` method]
    /// (../struct.ContextOf.html#method.current_context).
    pub unsafe fn current() -> Self {
        ArrayBufferBinder {
            _phantom: PhantomData
        }
    }

    /// Bind a buffer to the `GL_ARRAY_BUFFER` target, returning a binding.
    pub fn bind<'a>(&mut self, buffer: &'a mut Buffer) -> ArrayBufferBinding<'a>
    {
        let binding = ArrayBufferBinding {
            _phantom_ref: PhantomData,
            _phantom_ptr: PhantomData
        };
        _bind_buffer(binding.target(), buffer);
        binding
    }
}

/// The OpenGL state representing the `GL_ELEMENT_ARRAY_BUFFER` target.
pub struct ElementArrayBufferBinder {
    _phantom: PhantomData<*mut ()>
}

impl ElementArrayBufferBinder {
    /// Get the current `GL_ELEMENT_ARRAY_BUFFER` binder.
    ///
    /// # Safety
    /// The same rules apply to this method as the
    /// [`ContextOf::current_context()` method]
    /// (../struct.ContextOf.html#method.current_context).
    pub unsafe fn current() -> Self {
        ElementArrayBufferBinder {
            _phantom: PhantomData
        }
    }

    /// Bind a buffer to the `GL_ELEMENT_ARRAY_BUFFER` target, returning
    /// a binding.
    pub fn bind<'a>(&mut self, buffer: &'a mut Buffer)
        -> ElementArrayBufferBinding<'a>
    {
        let binding = ElementArrayBufferBinding {
            _phantom_ref: PhantomData,
            _phantom_ptr: PhantomData
        };
        _bind_buffer(binding.target(), buffer);
        binding
    }
}
