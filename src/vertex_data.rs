use super::gl;
use std::mem;
use std::raw;

pub trait VertexData: Copy {
    type Binder;
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum DataType {
    BYTE = gl::BYTE as isize,
    UNSIGNED_BYTE = gl::UNSIGNED_BYTE as isize,
    SHORT = gl::SHORT as isize,
    UNSIGNED_SHORT = gl::UNSIGNED_SHORT as isize,
    FIXED = gl::FIXED as isize,
    FLOAT = gl::FLOAT as isize
}
pub use self::DataType::*;



pub trait VertexPrimitive {
    fn gl_type() -> DataType;
}

impl VertexPrimitive for i8 {
    fn gl_type() -> DataType { self::BYTE }
}

impl VertexPrimitive for u8 {
    fn gl_type() -> DataType { self::UNSIGNED_BYTE }
}

impl VertexPrimitive for i16 {
    fn gl_type() -> DataType { self::SHORT }
}

impl VertexPrimitive for u16 {
    fn gl_type() -> DataType { self::UNSIGNED_SHORT }
}

impl VertexPrimitive for f32 {
    fn gl_type() -> DataType { self::FLOAT }
}

pub trait VertexDatum {
    fn gl_type() -> DataType;
    fn components() -> i8;
    fn normalized() -> bool { false }
}



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



#[macro_export]
macro_rules! vertex_data {
    ($(
        struct $name:ident {
            $($field_name:ident: $field_type:ty),*
        }
    )+) => {
        mod _glitter_vertex_data {
            $(
                #[allow(non_snake_case)]
                pub mod $name {
                    #[repr(C)]
                    #[derive(Debug, Clone, Copy)]
                    pub struct $name {
                        $($field_name: $field_type),*
                    }

                    pub struct Binder;

                    impl $crate::VertexData for $name {
                        type Binder = Binder;
                    }
                }
            )+
        }


        $(
            #[allow(unused_imports)]
            use self::_glitter_vertex_data::$name::$name;
        )+
    }
}
