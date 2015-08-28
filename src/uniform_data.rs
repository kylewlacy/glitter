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

impl UniformPrimitive for f32 {
    fn uniform_primitive_type() -> UniformPrimitiveType {
        UniformPrimitiveType::Float
    }
}

impl UniformPrimitive for i32 {
    fn uniform_primitive_type() -> UniformPrimitiveType {
        UniformPrimitiveType::Int
    }
}



impl<T: UniformPrimitive> UniformDatum for T {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

impl<T: UniformPrimitive> UniformDatum for [T; 1] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

impl<T: UniformPrimitive> UniformDatum for [T; 2] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

impl<T: UniformPrimitive> UniformDatum for [T; 3] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

impl<T> UniformDatum for [T; 4] where T: UniformPrimitive {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

impl UniformDatum for [[f32; 2]; 2] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Matrix2x2
    }
}

impl UniformDatum for [[f32; 3]; 3] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Matrix3x3
    }
}

impl UniformDatum for [[f32; 4]; 4] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Matrix4x4
    }
}

impl<T: UniformDatum> UniformData for T {
    fn uniform_datum_type() -> UniformDatumType {
        T::uniform_datum_type()
    }

    fn uniform_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(mem::transmute(self), mem::size_of::<T>())
        }
    }

    fn uniform_elements(&self) -> usize {
        1
    }
}

impl<T: UniformDatum> UniformData for [T] {
    fn uniform_datum_type() -> UniformDatumType {
        T::uniform_datum_type()
    }

    fn uniform_bytes(&self) -> &[u8] {
        let size = mem::size_of::<T>() * self.len();
        unsafe {
            slice::from_raw_parts(mem::transmute(&self[0]), size)
        }
    }

    fn uniform_elements(&self) -> usize {
        self.len()
    }
}

#[macro_export]
macro_rules! transpose {
    [[$a1:expr, $b1:expr],
     [$a2:expr, $b2:expr]] => {
        [[$a1, $a2],
         [$b1, $b2]]
    };
    [[$a1:expr, $b1:expr, $c1:expr],
     [$a2:expr, $b2:expr, $c2:expr],
     [$a3:expr, $b3:expr, $c3:expr]] => {
        [[$a1, $a2, $a3],
         [$b1, $b2, $b3],
         [$c1, $c2, $c3]]
    };
    [[$a1:expr, $b1:expr, $c1:expr, $d1:expr],
     [$a2:expr, $b2:expr, $c2:expr, $d2:expr],
     [$a3:expr, $b3:expr, $c3:expr, $d3:expr],
     [$a4:expr, $b4:expr, $c4:expr, $d4:expr]] => {
        [[$a1, $a2, $a3, $a4],
         [$b1, $b2, $b3, $b4],
         [$c1, $c2, $c3, $c4],
         [$d1, $d2, $d3, $d4]]
    }
}
