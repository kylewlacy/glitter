use std::mem;
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

pub trait GLObject {
    type Id;

    unsafe fn from_raw(id: Self::Id) -> Self;

    fn id(&self) -> Self::Id;

    fn into_raw(self) -> Self::Id
        where Self: Sized
    {
        let id = self.id();
        mem::forget(self);
        id
    }
}

bitflags! {
    flags BufferBits: gl::types::GLbitfield {
        const COLOR_BUFFER_BIT = gl::COLOR_BUFFER_BIT,
        const DEPTH_BUFFER_BIT = gl::DEPTH_BUFFER_BIT,
        const STENCIL_BUFFER_BIT = gl::STENCIL_BUFFER_BIT
    }
}

gl_enum! {
    pub gl_enum Capability {
        pub const Blend as BLEND
            = gl::BLEND,
        pub const CullFace as CULL_FACE
            = gl::CULL_FACE,
        pub const DepthTest as DEPTH_TEST
            = gl::DEPTH_TEST,
        pub const Dither as DITHER
            = gl::DITHER,
        pub const PolygonOffsetFill as POLYGON_OFFSET_FILL
            = gl::POLYGON_OFFSET_FILL,
        pub const SampleAlphaToCoverage as SAMPLE_ALPHA_TO_COVERAGE =
            gl::SAMPLE_ALPHA_TO_COVERAGE,
        pub const SampleCoverage as SAMPLE_COVERAGE =
            gl::SAMPLE_COVERAGE,
        pub const ScisscorTest as SCISSCOR_TEST =
            gl::SCISSOR_TEST,
        pub const StencilTest as STENCIL_TEST =
            gl::STENCIL_TEST
    }
}



#[derive(Debug)]
pub enum GLError {
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    FramebufferError(GLFramebufferError),
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
            GLError::FramebufferError(ref e) => {
                write!(f, "{:?}", e)
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
            GLError::FramebufferError(ref e) => {
                error::Error::description(e)
            },
            GLError::Message(ref s) => &s
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            GLError::FramebufferError(ref e) => { Some(e) },
            _ => { None }
        }
    }
}

#[derive(Debug)]
pub enum GLFramebufferError {
    IncompleteAttachment,
    IncompleteDimensions,
    IncompleteMissingAttachment,
    Unsupported
}

impl fmt::Display for GLFramebufferError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            GLFramebufferError::IncompleteAttachment => {
                write!(f, "Incomplete attachment")
            },
            GLFramebufferError::IncompleteDimensions => {
                write!(f, "Incomplete dimensions")
            },
            GLFramebufferError::IncompleteMissingAttachment => {
                write!(f, "Missing attachments")
            },
            GLFramebufferError::Unsupported => {
                write!(f, "Unsupported")
            }
        }
    }
}

impl error::Error for GLFramebufferError {
    fn description(&self) -> &str {
        match *self {
            GLFramebufferError::IncompleteAttachment => "One or more framebuffer attachments are not complete",
            GLFramebufferError::IncompleteDimensions => "Not all images attached to the framebuffer have the same width and height",
            GLFramebufferError::IncompleteMissingAttachment => "The framebuffer has no images attached",
            GLFramebufferError::Unsupported => "The framebuffer contains an unsupported combination of attachments",
        }
    }
}

impl From<GLFramebufferError> for GLError {
    fn from(e: GLFramebufferError) -> GLError {
        GLError::FramebufferError(e)
    }
}



gl_enum! {
    pub gl_enum DrawingMode {
        pub const Points as POINTS = gl::POINTS,
        pub const LineStrip as LINE_STRIP = gl::LINE_STRIP,
        pub const LineLoop as LINE_LOOP = gl::LINE_LOOP,
        pub const Lines as LINES = gl::LINES,
        pub const TriangleStrip as TRIANGLE_STRIP = gl::TRIANGLE_STRIP,
        pub const TriangleFan as TRIANGLE_FAN = gl::TRIANGLE_FAN,
        pub const Triangles as TRIANGLES = gl::TRIANGLES
    }
}

gl_enum! {
    pub gl_enum DataType {
        pub const Byte as BYTE = gl::BYTE,
        pub const UnsignedByte as UNSIGNED_BYTE = gl::UNSIGNED_BYTE,
        pub const Short as SHORT = gl::SHORT,
        pub const UnsignedShort as UNSIGNED_SHORT = gl::UNSIGNED_SHORT,
        pub const Fixed as FIXED = gl::FIXED,
        pub const Float as FLOAT = gl::FLOAT
    }
}
