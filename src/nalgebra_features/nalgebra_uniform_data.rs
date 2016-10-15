use nalgebra;
use uniform_data::{UniformDatumType, UniformDatum, UniformPrimitive};

unsafe impl<T: UniformPrimitive> UniformDatum for nalgebra::Vector2<T> {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec2(T::uniform_primitive_type())
    }
}

unsafe impl<T: UniformPrimitive> UniformDatum for nalgebra::Vector3<T> {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec3(T::uniform_primitive_type())
    }
}

unsafe impl<T: UniformPrimitive> UniformDatum for nalgebra::Vector4<T> {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec4(T::uniform_primitive_type())
    }
}

unsafe impl UniformDatum for nalgebra::Matrix2<f32> {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Matrix2x2
    }
}

unsafe impl UniformDatum for nalgebra::Matrix3<f32> {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Matrix3x3
    }
}

unsafe impl UniformDatum for nalgebra::Matrix4<f32> {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Matrix4x4
    }
}
