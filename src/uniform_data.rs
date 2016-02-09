//! Contains types that represent uniform data, which is used
//! for methods such as [`gl.set_uniform`]
//! (../context/program_context/trait.ContextProgramExt.html#method.set_uniform).

use std::slice;
use std::mem;

/// The basic value types that are composed in the [`UniformDatumTypes`]
/// (enum.UniformDatumType.html).
pub enum UniformPrimitiveType {
    /// A 32-bit floating point value.
    Float,

    /// A 32-bit signed integer value.
    Int
}

/// The basic types that can be used as uniform values in a program object.
pub enum UniformDatumType {
    /// A single scalar value, containing one primitive (essentially a vector
    /// of one component).
    Vec1(UniformPrimitiveType),

    /// A vector with 2 components.
    Vec2(UniformPrimitiveType),

    /// A vector with 3 components.
    Vec3(UniformPrimitiveType),

    /// A vector with 4 components.
    Vec4(UniformPrimitiveType),

    /// A 2x2 matrix of floating-point values.
    Matrix2x2,

    /// A 3x3 matrix of floating-point values.
    Matrix3x3,

    /// A 4x4 matrix of floating-point values.
    Matrix4x4
}

/// A type that can be set to a uniform value in a program object, using
/// the [`gl.set_uniform`]
/// (../context/program_context/trait.ContextProgramExt.html#method.set_uniform)
/// method. A `UniformData` type can be composed of one or more
/// [`UniformDatums`](trait.UniformDatum.html), and this is likely the type
/// that user types will implement.
pub trait UniformData {
    /// Return the type that this uniform data should be treated as.
    fn uniform_datum_type() -> UniformDatumType;

    /// Create a byte slice of uniform data from `self`.
    fn uniform_bytes(&self) -> &[u8];

    /// Return the number of uniform data elements that `self` contains.
    fn uniform_elements(&self) -> usize;
}



/// A single uniform value, which corresponds to a single
/// primitive GLSL uniform type.
///
/// # Safety
/// This type will be transmuted to a slice according to the value
/// returned by the [`uniform_datum_type`]
/// (trait.UniformDatum.html#tymethod.uniform_datum_type)
/// method. Implementing this method incorrectly will lead to memory
/// unsafety.
pub unsafe trait UniformDatum {
    /// Return the data type this datum corresponds to.
    ///
    /// # Safety
    /// An instance of this type must match the size and memory layout
    /// specified by the returned [`UniformDatumType`]
    /// (enum.UniformDatumType.html).
    fn uniform_datum_type() -> UniformDatumType;
}


/// A single, basic value that can be composed to make a [`UniformDatum`]
/// (trait.UniformDatum.html). Scalar values are an example of a `UniformPrimitive`.
///
/// # Safety
/// This type will be transmuted to a slice according to the value returned
/// by the [`uniform_primitive_type`]
/// (trait.UniformPrimitive.html#tymethod.uniform_primitive_type) method.
/// Implementing this method incorrectly will lead to memory unsafety.
pub unsafe trait UniformPrimitive {
    /// Return the data type this primitive corresponds to.
    ///
    /// # Safety
    /// An instance of this type must be the size of the
    /// [`UniformPrimitiveType`](enum.UniformPrimitiveType.html)
    /// returned by this function.
    fn uniform_primitive_type() -> UniformPrimitiveType;
}

unsafe impl UniformPrimitive for f32 {
    fn uniform_primitive_type() -> UniformPrimitiveType {
        UniformPrimitiveType::Float
    }
}

unsafe impl UniformPrimitive for i32 {
    fn uniform_primitive_type() -> UniformPrimitiveType {
        UniformPrimitiveType::Int
    }
}



unsafe impl<T: UniformPrimitive> UniformDatum for T {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

unsafe impl<T: UniformPrimitive> UniformDatum for [T; 1] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

unsafe impl<T: UniformPrimitive> UniformDatum for [T; 2] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

unsafe impl<T: UniformPrimitive> UniformDatum for [T; 3] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

unsafe impl<T> UniformDatum for [T; 4] where T: UniformPrimitive {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(T::uniform_primitive_type())
    }
}

unsafe impl UniformDatum for [[f32; 2]; 2] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Matrix2x2
    }
}

unsafe impl UniformDatum for [[f32; 3]; 3] {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Matrix3x3
    }
}

unsafe impl UniformDatum for [[f32; 4]; 4] {
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

/// Transpose a static matrix (of form `[[a1, b1, ...], [a2, b2, ...]]`
/// into `[[a1, a2, ...], [[b1, b2, ...]]]`).
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
