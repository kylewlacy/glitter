use std::mem;
use std::slice;
use types::DataType;

pub unsafe trait VertexData: Copy {
    fn visit_attributes<F>(&f: F) where F: FnMut(VertexAttribute);
}

pub unsafe trait VertexDatum: Copy {
    fn attrib_type() -> VertexAttributeType;
}

pub unsafe trait VertexPrimitive: Copy {
    fn data_type() -> DataType;
}

#[derive(Clone)]
pub struct VertexAttribute {
    pub ty: VertexAttributeType,
    pub name: String,
    pub offset: usize,
    pub stride: usize
}

#[derive(Clone)]
pub struct VertexAttributeType {
    pub data: DataType,
    pub components: i8,
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



pub trait VertexBytes {
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
#[macro_export]
macro_rules! offset_of {
    ($T:ty, $field:ident) => {
        unsafe {
            use std::mem::{uninitialized, transmute};

            let obj : $T = uninitialized();
            let obj_ptr : *const u8 = transmute(&obj);
            let member_ptr : *const u8 = transmute(&obj.$field);

            (member_ptr as usize) - (obj_ptr as usize)
        }
    }
}

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
            }
        }
    };
}
