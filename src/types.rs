use super::gl_lib as gl;

pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

impl Color {
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r: r, g: g, b: b, a: a}
    }
}

bitflags! {
    flags BufferBits: gl::types::GLbitfield {
        const COLOR_BUFFER_BIT = gl::COLOR_BUFFER_BIT,
        const DEPTH_BUFFER_BIT = gl::DEPTH_BUFFER_BIT,
        const STENCIL_BUFFER_BIT = gl::STENCIL_BUFFER_BIT
    }
}

#[derive(Debug)]
pub struct GLError {
    pub message: String
}

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum DrawingMode {
    POINTS = gl::POINTS as isize,
    LINE_STRIP = gl::LINE_STRIP as isize,
    LINE_LOOP = gl::LINE_LOOP as isize,
    LINES = gl::LINES as isize,
    TRIANGLE_STRIP = gl::TRIANGLE_STRIP as isize,
    TRIANGLE_FAN = gl::TRIANGLE_FAN as isize,
    TRIANGLES = gl::TRIANGLES as isize
}
pub use self::DrawingMode::*;
