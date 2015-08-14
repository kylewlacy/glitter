use super::gl;
use std::mem;
use std::raw;

pub trait VertexData: Copy {
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

                    impl $crate::VertexData for $name {

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
