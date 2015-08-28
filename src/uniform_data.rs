use std::slice;
use std::mem;

pub enum UniformPrimitiveType {
    Float,
    Int
}

pub enum UniformDatumType {
    Vec1(UniformPrimitiveType),
    Vec2(UniformPrimitiveType),
    Vec3(UniformPrimitiveType),
    Vec4(UniformPrimitiveType),
    Matrix2x2,
    Matrix3x3,
    Matrix4x4
}

pub trait UniformData {
    fn uniform_datum_type() -> UniformDatumType;
    fn uniform_bytes(&self) -> &[u8];
    fn uniform_elements(&self) -> usize;
}



pub trait UniformDatum {
    fn uniform_datum_type() -> UniformDatumType;
}


pub trait UniformPrimitive {
    fn uniform_primitive_type() -> UniformPrimitiveType;
}
