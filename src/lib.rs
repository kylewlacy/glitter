extern crate gl as gl_lib;

mod context;

pub mod gl {
    pub use super::gl_lib::*;
}

pub use context::Context;
