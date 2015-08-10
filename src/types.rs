use super::gl;

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
