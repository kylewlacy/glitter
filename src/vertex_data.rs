use std::mem;
use std::raw;
use super::gl_lib as gl;
use super::context::Context;

pub trait VertexData: Copy {
    type Binder: VertexAttribBinder;

    fn build_attrib_binder() -> <Self::Binder as VertexAttribBinder>::Builder;
}

pub trait VertexAttribBinder {
    type Builder;

    fn bind(&self, gl: &Context);
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

impl<T> VertexDatum for T where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 1 }
}

impl<T> VertexDatum for [T; 1] where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 1 }
}

impl<T> VertexDatum for [T; 2] where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 2 }
}

impl<T> VertexDatum for [T; 3] where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 3 }
}

impl<T> VertexDatum for [T; 4] where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 4 }
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

impl<T> VertexBytes for [T] where T: VertexData {
    fn vertex_bytes(&self) -> &[u8] {
        unsafe {
            mem::transmute(raw::Slice::<T> {
                data: &self[0],
                len: mem::size_of::<T>() * self.len()
            })
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


#[macro_use(offset_of)]
#[macro_export]
macro_rules! vertex_data {
    ($(
        struct $name:ident {
            $($field_name:ident: $field_type:ty),*
        }
    )+) => {
        mod _glitter_vertex_data {$(
            #[allow(non_snake_case)]
            pub mod $name {
                #[repr(C)]
                #[derive(Debug, Clone, Copy)]
                pub struct $name {
                    $(pub $field_name: $field_type),*
                }

                impl $crate::VertexData for $name {
                    type Binder = Binder;

                    fn build_attrib_binder() -> BinderBuilder {
                        BinderBuilder::new()
                    }
                }

                #[allow(dead_code)]
                pub struct Binder {
                    $($field_name: $crate::ProgramAttrib),*
                }

                impl $crate::VertexAttribBinder for Binder {
                    type Builder = BinderBuilder;

                    fn bind(&self, gl: &$crate::Context) {
                        use std::mem;
                        use $crate::VertexDatum as Datum;

                        $({
                            let components =
                                <$field_type as Datum>::components();
                            let gl_type =
                              <$field_type as Datum>::gl_type();
                            let normalized =
                                <$field_type as Datum>::normalized();
                            let stride = mem::size_of::<$name>();
                            let offset = offset_of!($name, $field_name);

                            gl.vertex_attrib_pointer(self.$field_name,
                                                     components,
                                                     gl_type,
                                                     normalized,
                                                     stride,
                                                     offset);
                        };)*
                    }
                }

                #[allow(dead_code)]
                pub struct BinderBuilder {
                    $($field_name: Option<$crate::ProgramAttrib>),*
                }

                #[allow(dead_code)]
                impl BinderBuilder {
                    pub fn new() -> Self {
                        BinderBuilder {
                            $($field_name: None),*
                        }
                    }

                    pub fn unwrap(self, gl: &$crate::Context) -> self::Binder {
                        let binder = self::Binder {
                            $($field_name: self.$field_name.expect(
                                concat!("No attribute provided for ",
                                        stringify!($field_name))
                            )),*
                        };

                        $(gl.enable_vertex_attrib_array(binder.$field_name);)*

                        binder
                    }

                    $(
                        pub fn $field_name(mut self,
                                           attrib: $crate::ProgramAttrib)
                        -> Self
                        {
                            self.$field_name = Some(attrib);
                            self
                        }
                    )*
                }
            })+
        }


        $(
            #[allow(unused_imports)]
            use self::_glitter_vertex_data::$name::$name;
        )+
    }
}
