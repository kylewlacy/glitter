//! Contains types related to vertex data, which are used for [`VertexBuffers`]
//! (../vertex_buffer/struct.VertexBuffer.html).

use std::mem;
use std::slice;
use types::DataType;

/// A type that has vertex data.
///
/// # Safety
/// Implementors must properly implement the [`visit_attributes`]
/// (trait.VertexData.html#tymethod.visit_attributes) method. Implementing
/// this method incorrectly will lead to memory unsafety in some safe
/// API's in glitter.
///
/// # Example
/// ```
/// #[macro_use] extern crate glitter;
/// use glitter::prelude::*;
///
/// #[derive(Clone, Copy)] // `VertexData` types must implement `Copy`
/// #[repr(C)] // Use a predictable data layout (for our field offsets)
/// struct MyVertex {
///     position: [f32; 3],
///     color: [f32; 3]
/// }
///
/// // Manually implement the `VertexData` trait
/// unsafe impl glitter::VertexData for MyVertex {
///     fn visit_attributes<F>(mut f: F)
///         where F: FnMut(glitter::VertexAttribute)
///     {
///         use std::mem;
///         use glitter::{VertexAttribute, VertexAttributeType, VertexDatum};
///
///         // The vertex attribute type for an `[f32; 3]`, or `vec3`
///         let vec3 = <[f32; 3] as VertexDatum>::attrib_type();
///
///         // let vec3 = VertexAttributeType {
///         //     data: glitter::FLOAT,
///         //     components: 3,
///         //     normalize: false
///         // };
///
///         let stride = mem::size_of::<MyVertex>();
///         let position_offset = 0;
///         let position_size = mem::size_of::<[f32; 3]>();
///         let color_offset = position_offset + position_size;
///
///         let position = VertexAttribute {
///             ty: vec3.clone(),
///             name: "position".into(),
///             offset: position_offset,
///             stride: stride
///         };
///         let color = VertexAttribute {
///             ty: vec3,
///             name: "color".into(),
///             offset: color_offset,
///             stride: stride
///         };
///
///         // Call the function with the "position" and "color" attributes
///         f(position);
///         f(color);
///     }
/// }
/// # fn main() {
/// #     // TODO: Add `VertexData` implementation test
/// # }
/// ```
///
/// # See also
/// [`impl_vertx_data!`](../macro.impl_vertex_data!.html): A macro that
/// automatically implements `VertexData` for structs.
pub unsafe trait VertexData: Copy {
    /// Call the given function with each attribute that this
    /// vertex data is composed of.
    ///
    /// # Safety
    /// The function must only be called once per attribute, and
    /// the provided values of the [`VertexAttribute`]
    /// (struct.VertexAttribute.html) passed to the function *must*
    /// be correct. See the [`VertexAttribute`](struct.VertexAttribute.html)
    /// docs for more details on what each field means.
    fn visit_attributes<F>(&f: F) where F: FnMut(VertexAttribute);
}

/// A single value that can be treated as a part of a vertex. Implementors
/// should map to a GLSL primitive that can be used as a vertex attribute.
/// For example, `[f32; 2]` corresponds to `vec2` in memory layout
/// and structure.
///
/// # Safety
/// This type will be transmuted to a slice according to the value returned
/// by the [`attrib_type`](trait.VertexDatum.html#tymethod.attrib_type) method.
/// Implementing this method incorrectly will lead to memory unsafety.
pub unsafe trait VertexDatum: Copy {
    /// Return the data type this datum corresponds to.
    ///
    /// # Safety
    /// An instance of this type must match the size and memory layout
    /// specified by the returned [`VertexAttributeType`]
    /// (struct.VertexAttributeType.html).
    fn attrib_type() -> VertexAttributeType;
}

/// A single, basic value that can be composed to make a [`VertexDatum`]
/// (trait.VertexDatum.html). Scalar values are an example of a
/// `VertexPrimitive`.
///
/// # Safety
/// This type will be transmuted to a slice according to the value returned
/// by the [`data_type`](trait.VertexPrimitive.html#tymethod.data_type) method.
/// Implementing this method incorrectly will lead to memory unsafety.
pub unsafe trait VertexPrimitive: Copy {
    /// Return the data type this primitive corresponds to.
    ///
    /// # Safety
    /// An instance of this type must be the size of the [`DataType`]
    /// (../types/enum.DataType.html) returned by this function.
    fn data_type() -> DataType;
}

/// Specifies the type, name, and memory layout of a vertex attribute.
/// Generally, a "vertex attribute" corresponds to a field in a [`VertexData`]
/// (trait.VertexData.html) struct.
#[derive(Clone)]
pub struct VertexAttribute {
    /// The type of the vertex attribute. This also specifies the number
    /// of bytes that make up a vertex attribute.
    pub ty: VertexAttributeType,

    /// The name of the vertex attribute. This value is used
    /// as the `$field_name` when binding vertex attribute
    /// pointers with the [`attrib_pointers!`](macro.attrib_pointers!.html)
    /// macro.
    pub name: String,

    /// The number of bytes to "move" from the start of the vertex data
    /// to reach this vertex attribute.
    pub offset: usize,

    /// The number of bytes between consecutive vertex attributes. 0 indicates
    /// that the vertex data is tightly packed.
    pub stride: usize
}

/// Used to specify type of a vertex attribute. The size of the vertex
/// attribute is `size_of(data) * components`.
#[derive(Clone)]
pub struct VertexAttributeType {
    /// The type of data that makes up this vertex attribute.
    pub data: DataType,

    /// The number of `data` components that make up this vertex attribute.
    pub components: i8,

    /// If the `data` type is fixed-point data, indicates if the data
    /// should be normalized when being accessed. `true` indicates
    /// that the vertex attribute **should** be normalized when being
    /// accessed.
    pub normalize: bool
}



unsafe impl VertexPrimitive for i8 {
    fn data_type() -> DataType { DataType::Byte }
}

unsafe impl VertexPrimitive for u8 {
    fn data_type() -> DataType { DataType::UnsignedByte }
}

unsafe impl VertexPrimitive for i16 {
    fn data_type() -> DataType { DataType::Short }
}

unsafe impl VertexPrimitive for u16 {
    fn data_type() -> DataType { DataType::UnsignedShort }
}

unsafe impl VertexPrimitive for f32 {
    fn data_type() -> DataType { DataType::Float }
}

unsafe impl<T: VertexPrimitive> VertexDatum for T {
    fn attrib_type() -> VertexAttributeType {
        VertexAttributeType {
            data: T::data_type(),
            components: 1,
            normalize: false
        }
    }
}

unsafe impl<T: VertexPrimitive> VertexDatum for [T; 1] {
    fn attrib_type() -> VertexAttributeType {
        VertexAttributeType {
            data: T::data_type(),
            components: 1,
            normalize: false
        }
    }
}

unsafe impl<T: VertexPrimitive> VertexDatum for [T; 2] {
    fn attrib_type() -> VertexAttributeType {
        VertexAttributeType {
            data: T::data_type(),
            components: 2,
            normalize: false
        }
    }
}

unsafe impl<T: VertexPrimitive> VertexDatum for [T; 3] {
    fn attrib_type() -> VertexAttributeType {
        VertexAttributeType {
            data: T::data_type(),
            components: 3,
            normalize: false
        }
    }
}

unsafe impl<T: VertexPrimitive> VertexDatum for [T; 4] {
    fn attrib_type() -> VertexAttributeType {
        VertexAttributeType {
            data: T::data_type(),
            components: 4,
            normalize: false
        }
    }
}



/// Indicates that a type can be coerced to a `u8` slice that can
/// then be treated as a stream of vertex data.
pub trait VertexBytes {
    /// Create a byte slice of vertex data from `self`.
    fn vertex_bytes(&self) -> &[u8];
}

impl<T> VertexBytes for T where T: VertexData {
    fn vertex_bytes(&self) -> &[u8] {
        unsafe {
            slice::from_raw_parts(mem::transmute(self), mem::size_of::<Self>())
        }
    }
}

impl<T> VertexBytes for [T] where T: VertexData {
    fn vertex_bytes(&self) -> &[u8] {
        let size = mem::size_of::<T>() * self.len();
        unsafe {
            slice::from_raw_parts(mem::transmute(&self[0]), size)
        }
    }
}

// TODO: Use a proper compiler intrinsic/macro (when available)
// see: https://github.com/rust-lang/rust/issues/24111
/// Compute the offset of a field within a struct type.
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate glitter;
/// # fn main() {
/// #[repr(C, packed)]
/// struct MyStruct {
///     foo: u32,
///     bar: u32,
///     baz: u32
/// }
///
/// assert_eq!(offset_of!(MyStruct, foo), 0);
/// assert_eq!(offset_of!(MyStruct, bar), 4);
/// assert_eq!(offset_of!(MyStruct, baz), 8);
/// # }
/// ```
#[macro_export]
macro_rules! offset_of {
    ($T:ty, $field:ident) => {
        unsafe {
            let obj: $T = ::std::mem::uninitialized();
            let obj_ptr: *const u8 = ::std::mem::transmute(&obj);
            let member_ptr: *const u8 = ::std::mem::transmute(&obj.$field);

            ::std::mem::forget(obj);

            (member_ptr as usize) - (obj_ptr as usize)
        }
    }
}

/// Implement the [`VertexData`](vertex_data/trait.VertexData.html) trait
/// for a struct. Each field of the struct must that is part of the
/// `VertexData` implementation must be a type that implements [`VertexDatum`]
/// (vertex_data/trait.VertexDatum.html).
///
/// # Examples
///
/// ```
/// #[macro_use] extern crate glitter;
///
/// # fn main() {
/// // A type must implement `Copy` to implement `VertexData`
/// #[derive(Clone, Copy)]
/// struct MyVertex {
///     position: [f32; 3],
///     color: [f32; 3]
/// }
///
/// // Implement `VertexData`, using the "position" and "color" fields
/// // as vertex attributes.
/// impl_vertex_data!(MyVertex, position, color);
/// # }
/// ```
#[macro_export]
macro_rules! impl_vertex_data {
    ($name:ty, $($field_name:ident),*) => {
        unsafe impl $crate::VertexData for $name {
            fn visit_attributes<F>(mut f: F)
                where F: FnMut($crate::VertexAttribute)
            {
                // TODO: A better way of iterating over field types
                let _data: $name = unsafe { ::std::mem::uninitialized() };
                fn get_attribute_type<T: $crate::VertexDatum>(_: &T)
                    -> $crate::VertexAttributeType
                {
                    T::attrib_type()
                }
                $(
                    f($crate::VertexAttribute {
                        ty: get_attribute_type(&_data.$field_name),
                        name: stringify!($field_name).into(),
                        stride: ::std::mem::size_of::<$name>(),
                        offset: offset_of!($name, $field_name)
                    });
                )*

                ::std::mem::forget(_data);
            }
        }
    };
}
