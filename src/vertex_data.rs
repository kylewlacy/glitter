pub trait VertexData: Copy { }

pub trait VertexBytes {
    fn vertex_bytes(&self) -> &[u8];
}
