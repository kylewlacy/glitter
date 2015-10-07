use std::mem;
use std::slice;
use buffer::ArrayBufferBinding;
use types::DataType;

pub unsafe trait VertexData: Copy {
    type Binder: VertexAttribBinder;

    fn build_attrib_binder() -> <Self::Binder as VertexAttribBinder>::Builder;
}

pub trait VertexAttribBinder {
    type Builder;

    fn bind(&self, gl_buffer: &ArrayBufferBinding);
}



pub unsafe trait VertexPrimitive {
    fn gl_type() -> DataType;
}

unsafe impl VertexPrimitive for i8 {
    fn gl_type() -> DataType { DataType::Byte }
}

unsafe impl VertexPrimitive for u8 {
    fn gl_type() -> DataType { DataType::UnsignedByte }
}

unsafe impl VertexPrimitive for i16 {
    fn gl_type() -> DataType { DataType::Short }
}

unsafe impl VertexPrimitive for u16 {
    fn gl_type() -> DataType { DataType::UnsignedShort }
}

unsafe impl VertexPrimitive for f32 {
    fn gl_type() -> DataType { DataType::Float }
}

pub unsafe trait VertexDatum {
    fn gl_type() -> DataType;
    fn components() -> i8;
    fn normalized() -> bool { false }
}

unsafe impl<T> VertexDatum for T where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 1 }
}

unsafe impl<T> VertexDatum for [T; 1] where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 1 }
}

unsafe impl<T> VertexDatum for [T; 2] where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 2 }
}

unsafe impl<T> VertexDatum for [T; 3] where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 3 }
}

unsafe impl<T> VertexDatum for [T; 4] where T: VertexPrimitive {
    fn gl_type() -> DataType { <T as VertexPrimitive>::gl_type() }
    fn components() -> i8 { 4 }
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
macro_rules! _glitter_vertex_data_reexport {
    ({ }, $name:ident) => {
        #[allow(unused_imports)]
        use self::_glitter_vertex_data::$name::$name;
    };
    ({ pub }, $name:ident) => {
        #[allow(unused_imports)]
        pub use self::_glitter_vertex_data::$name::$name;
    }
}

#[macro_use(offset_of, _glitter_vertex_data_reexport)]
#[macro_export]
macro_rules! vertex_data {
    (
        $(
            $(#[$($attrs:tt)*])*
            struct $name:ident {
                $($field_name:ident: $field_type:ty),*
            }
        )+
    ) => {
        vertex_data! {
            $(
                $(#[$($attrs)*])*,
                { },
                @struct $name { $($field_name: $field_type),* }
            )+,
        }
    };
    (
        $(
            $(#[$($attrs:tt)*])*
            pub struct $name:ident {
                $($field_name:ident: $field_type:ty),*
            }
        )+
    ) => {
        vertex_data! {
            $(
                $(#[$($attrs)*])*,
                { pub },
                @struct $name { $($field_name: $field_type),* }
            )+,
        }
    };
    (
        $(
            $(#[$($attrs:tt)*])*,
            { $($modifiers:tt)* },
            @struct $name:ident {
                $($field_name:ident: $field_type:ty),*
            }
        )+,
    ) => {
        mod _glitter_vertex_data {$(
            #[allow(non_snake_case)]
            pub mod $name {
                #[repr(C)]
                #[derive(Debug, Clone, Copy)]
                $(#[$($attrs)*])*
                pub struct $name {
                    $(pub $field_name: $field_type),*
                }

                unsafe impl $crate::VertexData for $name {
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

                    fn bind(&self, gl_buffer: &$crate::ArrayBufferBinding) {
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

                            unsafe {
                                gl_buffer.vertex_attrib_pointer(
                                    self.$field_name,
                                    components,
                                    gl_type,
                                    normalized,
                                    stride,
                                    offset);
                            }
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
            _glitter_vertex_data_reexport!({ $($modifiers)* }, $name);
        )+
    }
}
