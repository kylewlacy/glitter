use cgmath;
use types::DataType;
use vertex_data::{VertexDatum, VertexPrimitive};

unsafe impl<T: VertexPrimitive> VertexDatum for cgmath::Vector2<T> {
    fn gl_type() -> DataType { T::gl_type() }
    fn components() -> i8 { 2 }
}

unsafe impl<T: VertexPrimitive> VertexDatum for cgmath::Vector3<T> {
    fn gl_type() -> DataType { T::gl_type() }
    fn components() -> i8 { 3 }
}

unsafe impl<T: VertexPrimitive> VertexDatum for cgmath::Vector4<T> {
    fn gl_type() -> DataType { T::gl_type() }
    fn components() -> i8 { 4 }
}
