extern crate gl as gl_lib;

mod context;
mod types;

pub mod gl {
    pub use super::gl_lib::*;
}

pub use context::Context;
