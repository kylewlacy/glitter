use nalgebra;
use vertex_data::{VertexAttributeType, VertexDatum, VertexPrimitive};

unsafe impl<T: VertexPrimitive> VertexDatum for nalgebra::Vector2<T> {
    fn attrib_type() -> VertexAttributeType {
        VertexAttributeType {
            data: T::data_type(),
            components: 2,
            normalize: false
        }
    }
}

unsafe impl<T: VertexPrimitive> VertexDatum for nalgebra::Vector3<T> {
    fn attrib_type() -> VertexAttributeType {
        VertexAttributeType {
            data: T::data_type(),
            components: 3,
            normalize: false
        }
    }
}

unsafe impl<T: VertexPrimitive> VertexDatum for nalgebra::Vector4<T> {
    fn attrib_type() -> VertexAttributeType {
        VertexAttributeType {
            data: T::data_type(),
            components: 4,
            normalize: false
        }
    }
}
