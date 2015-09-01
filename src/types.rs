use std::fmt;
use std::error;
use gl;

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

#[derive(Clone, Copy)]
pub struct Viewport {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32
}

impl Viewport {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Viewport {
            x: x,
            y: y,
            width: width,
            height: height
        }
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
pub enum GLError {
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    Message(String)
}

impl fmt::Display for GLError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GLError::InvalidEnum => {
                write!(f, "Invalid enum")
            },
            GLError::InvalidValue => {
                write!(f, "Invalid value")
            },
            GLError::InvalidOperation => {
                write!(f, "Invalid operation")
            },
            GLError::InvalidFramebufferOperation => {
                write!(f, "Invalid framebuffer operation")
            },
            GLError::OutOfMemory => {
                write!(f, "Out of memory")
            },
            GLError::Message(ref s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl error::Error for GLError {
    fn description(&self) -> &str {
        match *self {
            GLError::InvalidEnum => "Invalid enum variant in OpenGL call.",
            GLError::InvalidValue => "Out-of-range value in OpenGL call.",
            GLError::InvalidOperation => "The specified OpenGL operation is not allowed in the current state.",
            GLError::InvalidFramebufferOperation => "OpenGL command tried to read or write to an incomplete framebuffer.",
            GLError::OutOfMemory => "There is not enough memory left to execute the specified OpenGL command.",
            GLError::Message(ref s) => &s
        }
    }
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

#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub enum DataType {
    BYTE = gl::BYTE as isize,
    UNSIGNED_BYTE = gl::UNSIGNED_BYTE as isize,
    SHORT = gl::SHORT as isize,
    UNSIGNED_SHORT = gl::UNSIGNED_SHORT as isize,
    FIXED = gl::FIXED as isize,
    FLOAT = gl::FLOAT as isize
}
pub use self::DataType::*;
