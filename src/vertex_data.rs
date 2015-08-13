use std::mem;
use std::raw;

pub trait VertexData: Copy { }

pub trait VertexBytes {
    fn vertex_bytes(&self) -> &[u8];
}

impl<T> VertexBytes for T where T: VertexData {
    fn vertex_bytes(&self) -> &[u8] {
        unsafe {
            mem::transmute(raw::Slice::<Self> {
                data: self,
                len: mem::size_of::<Self>()
            })
        }
    }
}
