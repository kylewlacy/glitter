use std::mem;
use std::ptr;
use std::marker::PhantomData;
use std::borrow::{Borrow, BorrowMut};
use gl;
use gl::types::*;
use context::{AContext, ContextOf};
use types::{DrawingMode, GLError};
use index_data::{IndexData, IndexDatum, IndexDatumType};
use to_ref::{ToRef, ToMut};

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
        gl::BindBuffer(target as GLuint, buffer.gl_id());
        dbg_gl_sanity_check! {
            GLError::InvalidEnum => "`target` is not an allowed value",
            _ => "Unknown error"
        }
    }
}

pub unsafe trait ContextBufferExt {
    fn gen_buffer(&self) -> Buffer {
        let mut id : GLuint = 0;

        unsafe {
            gl::GenBuffers(1, &mut id as *mut GLuint);
        }
        dbg_gl_sanity_check! {
            GLError::InvalidValue => "`n` is negative",
            _ => "Unknown error"
        }

        Buffer { gl_id: id }
    }

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

    unsafe fn vertex_attrib_pointer(&self,
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

    unsafe fn draw_n_elements_buffered(&self,
                                       _ab: &ArrayBufferBinding,
                                       _eab: &ElementArrayBufferBinding,
                                       mode: DrawingMode,
                                       count: usize,
                                       index_type: IndexDatumType)
    {
        _draw_elements(mode, count, index_type, ptr::null());
    }

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

    unsafe fn draw_elements<I>(&self,
                               _ab: &ArrayBufferBinding,
                               mode: DrawingMode,
                               indices: &[I])
        where I: IndexDatum, [I]: IndexData
    {
        self.draw_n_elements(_ab, mode, indices.len(), indices);
    }
}

unsafe impl<B, F, P, R, T> ContextBufferExt for ContextOf<B, F, P, R, T> {

}

unsafe impl<'a, B, F, P, R, T> ContextBufferExt
    for &'a mut ContextOf<B, F, P, R, T>
{

}



pub trait ArrayBufferContext: AContext {
    type Binder: BorrowMut<ArrayBufferBinder>;
    type Rest: AContext;

    fn split_array_buffer(self) -> (Self::Binder, Self::Rest);

    fn bind_array_buffer<'a>(self, buffer: &'a mut Buffer)
        -> (ArrayBufferBinding<'a>, Self::Rest)
        where Self: Sized
    {
        let (mut binder, rest) = self.split_array_buffer();
        (binder.borrow_mut().bind(buffer), rest)
    }
}

pub trait ElementArrayBufferContext: AContext {
    type Binder: BorrowMut<ElementArrayBufferBinder>;
    type Rest: AContext;

    fn split_element_array_buffer(self) -> (Self::Binder, Self::Rest);

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



gl_enum! {
    pub gl_enum BufferDataUsage {
        StreamDraw as STREAM_DRAW = gl::STREAM_DRAW,
        StaticDraw as STATIC_DRAW = gl::STATIC_DRAW,
        DynamicDraw as DYNAMIC_DRAW = gl::DYNAMIC_DRAW
    }
}

gl_enum! {
    pub gl_enum BufferBindingTarget {
        ArrayBuffer as ARRAY_BUFFER = gl::ARRAY_BUFFER,
        ElementArrayBuffer as ELEMENT_ARRAY_BUFFER = gl::ELEMENT_ARRAY_BUFFER
    }
}



pub trait BufferBinding {
    fn target(&self) -> BufferBindingTarget;
}

pub struct ArrayBufferBinding<'a> {
    phantom: PhantomData<&'a mut Buffer>
}

impl<'a> BufferBinding for ArrayBufferBinding<'a> {
    fn target(&self) -> BufferBindingTarget {
        ARRAY_BUFFER
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



pub struct BufferBinderOf<A, E> {
    array: A,
    element_array: E
}

pub type BufferBinder = BufferBinderOf<ArrayBufferBinder,
                                       ElementArrayBufferBinder>;

impl<A, E> BufferBinderOf<A, E> {
    pub unsafe fn current() -> BufferBinder {
        BufferBinderOf {
            array: ArrayBufferBinder,
            element_array: ElementArrayBufferBinder
        }
    }

    pub fn borrowed<'a, BA = A, BE = E>(&'a self)
        -> BufferBinderOf<&'a BA, &'a BE>
        where A: Borrow<BA>,
              E: Borrow<BE>
    {
        BufferBinderOf {
            array: self.array.borrow(),
            element_array: self.element_array.borrow()
        }
    }

    pub fn borrowed_mut<'a, BA = A, BE = E>(&'a mut self)
        -> BufferBinderOf<&'a mut BA, &'a mut BE>
        where A: BorrowMut<BA>,
              E: BorrowMut<BE>
    {
        BufferBinderOf {
            array: self.array.borrow_mut(),
            element_array: self.element_array.borrow_mut()
        }
    }

    pub fn swap_array<NA>(self, new_array: NA)
        -> (A, BufferBinderOf<NA, E>)
    {
        (
            self.array,
            BufferBinderOf {
                array: new_array,
                element_array: self.element_array
            }
        )
    }

    pub fn swap_element_array<NE>(self, new_element_array: NE)
        -> (E, BufferBinderOf<A, NE>)
    {
        (
            self.element_array,
            BufferBinderOf {
                array: self.array,
                element_array: new_element_array
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
            element_array: self.element_array.to_ref()
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
            element_array: self.element_array.to_mut()
        }
    }
}



pub struct ArrayBufferBinder;
impl ArrayBufferBinder {
    pub fn bind<'a>(&mut self, buffer: &'a mut Buffer) -> ArrayBufferBinding<'a>
    {
        let binding = ArrayBufferBinding { phantom: PhantomData };
        _bind_buffer(binding.target(), buffer);
        binding
    }
}

pub struct ElementArrayBufferBinder;
impl ElementArrayBufferBinder {
    pub fn bind<'a>(&mut self, buffer: &'a mut Buffer)
        -> ElementArrayBufferBinding<'a>
    {
        let binding = ElementArrayBufferBinding { phantom: PhantomData };
        _bind_buffer(binding.target(), buffer);
        binding
    }
}
