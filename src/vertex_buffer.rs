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
}
