// Used to specify checks that shouldn't fail (but might in unsafe)
macro_rules! dbg_gl_error {
    ($($pat:pat => $msg:expr),*) => {
        if cfg!(debug_assertions) {
            let err = $crate::Context::get_error();
            match err {
                $(Some($pat) => {
                    panic!("OpenGL error {:?} - {}", err, $msg)
                }),*
                None => { }
            }
        }
    }
}

// Used to specify checks that should *never* be able to fail (even in unsafe!)
macro_rules! dbg_gl_sanity_check {
    ($($pat:pat => $msg:expr),*) => {
        dbg_gl_error! { $($pat => concat!("Sanity check failed: ", $msg)),* }
    }
}

// Create a Rust enum that maps to an OpenGL enum.
macro_rules! gl_enum {
    (
        $(#[$attr:meta])*
        pub gl_enum $name:ident {
            $(
                $(#[$variant_attr:meta])*
                pub const $variant:ident as $const_name:ident = $value:expr
            ),+
        }
    ) => {
        $(#[$attr])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $($(#[$variant_attr])* $variant = $value as isize),+
        }
        $($(#[$variant_attr])* pub const $const_name: $name = $name::$variant;)+

        #[allow(dead_code)]
        impl $name {
            /// Convert from a raw OpenGL integer value to an enum variant.
            /// Returns an error if the value is not a valid enum variant.
            pub fn from_gl(gl_enum: $crate::gl::types::GLenum)
                -> Result<Self, ()>
            {
                match gl_enum {
                    $(x if x == $value => { Ok($name::$variant) },)+
                    _ => { Err(()) }
                }
            }

            /// Return the OpenGL integer value for a given enum variant.
            pub fn gl_enum(&self) -> $crate::gl::types::GLenum {
                *self as $crate::gl::types::GLenum
            }
        }
    }
}
