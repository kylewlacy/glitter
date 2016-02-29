//! Contains miscellaneous general-purpose OpenGL types.

use std::mem;
use std::fmt;
use std::error;
use gl;

/// A color, with floating-point RGBA components.
pub struct Color {
    /// The color's red component.
    pub r: f32,

    /// The color's green component.
    pub g: f32,

    /// The color's blue component.
    pub b: f32,

    /// The color's alpha component.
    pub a: f32
}

impl Color {
    /// Create a new color, with the specified RGBA values.
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r: r, g: g, b: b, a: a}
    }
}

/// An OpenGL viewport, with an origin and size, with integer components.
#[derive(Clone, Copy)]
pub struct Viewport {
    /// The X coordinate of the viewport's origin
    pub x: u32,

    /// The Y coordinate of the viewport's origin
    pub y: u32,

    /// The viewport's width
    pub width: u32,

    /// The viewport's height
    pub height: u32
}

impl Viewport {
    /// Create a new viewport, with the given X and Y coordinates as the origin
    /// and the given width and height as the size.
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Viewport {
            x: x,
            y: y,
            width: width,
            height: height
        }
    }
}

/// An OpenGL object.
pub trait GLObject {
    /// The type of the object's internal ID.
    type Id;

    /// Get a new object from an OpenGL ID.
    ///
    /// # Safety
    /// The provided ID must correspond to an instance of the target
    /// type. Providing an invalid object ID is considered undefined
    /// behavior in glitter.
    ///
    /// Additionally, using `from_raw` should not be used to create two
    /// objects that contain the same ID. Doing so will likely lead to
    /// the object being freed twice (which may not be catastrophic in
    /// OpenGL).
    unsafe fn from_raw(id: Self::Id) -> Self;

    /// Get the object's ID.
    fn id(&self) -> Self::Id;

    /// Consume the object without cleaning up its resources, returning the
    /// object's ID.
    ///
    /// # Note
    /// To properly clean up the object's resources, the returned ID
    /// should be passed to [`GLObject::from_raw`]
    /// (trait.GLObject.html#tymethod.from_raw) so that the object
    /// will be properly destroyed.
    fn into_raw(self) -> Self::Id
        where Self: Sized
    {
        let id = self.id();
        mem::forget(self);
        id
    }
}

bitflags! {
    /// The possible buffers that the active framebuffer may contain.
    pub flags BufferBits: ::gl::types::GLbitfield {
        /// The color buffer, which stores color information
        /// for each fragment (or pixel).
        const COLOR_BUFFER_BIT = ::gl::COLOR_BUFFER_BIT,

        /// The depth buffer, which stores distance information
        /// for each fragment when depth testing is enabled.
        const DEPTH_BUFFER_BIT = ::gl::DEPTH_BUFFER_BIT,

        /// The stencil buffer, which stores information about
        /// which fragments should be kept or discarded when
        /// stencil testing is enabled.
        const STENCIL_BUFFER_BIT = ::gl::STENCIL_BUFFER_BIT
    }
}

gl_enum! {
    /// The OpenGL drawing capabilities that can be enabled or disabled.
    pub gl_enum Capability {
        /// Blend newly-computed fragment colors with the current
        /// values in the color buffer.
        pub const Blend as BLEND
            = gl::BLEND,

        /// Cull polygons, based on their winding in window coordinates.
        pub const CullFace as CULL_FACE
            = gl::CULL_FACE,

        /// Perform a depth test for each fragment, only drawing
        /// fragments that are not obscured by other geometry.
        /// Also updates the depth buffer appropriately.
        pub const DepthTest as DEPTH_TEST
            = gl::DEPTH_TEST,

        /// Dither color components or indices.
        pub const Dither as DITHER
            = gl::DITHER,

        /// When filling a polygon, add an offset to
        /// each fragment's depth value.
        pub const PolygonOffsetFill as POLYGON_OFFSET_FILL
            = gl::POLYGON_OFFSET_FILL,

        // TODO: Is this correct?
        /// When multisampling, use the alpha value from the sample
        /// location.
        pub const SampleAlphaToCoverage as SAMPLE_ALPHA_TO_COVERAGE =
            gl::SAMPLE_ALPHA_TO_COVERAGE,

        // TODO: Is this correct?
        // TODO: Link to glSampleCoverage
        /// When multisampling, use the preset sample coverage value
        /// as the alpha value.
        pub const SampleCoverage as SAMPLE_COVERAGE =
            gl::SAMPLE_COVERAGE,

        /// Only draw fragments within the scissor rectangle.
        pub const ScisscorTest as SCISSCOR_TEST =
            gl::SCISSOR_TEST,

        /// Perform a stencil test for each fragment, only drawing
        /// fragments that pass the currently-set stencil operation.
        /// Also updates the stencil buffer appropriately.
        pub const StencilTest as STENCIL_TEST =
            gl::STENCIL_TEST
    }
}



/// The various possible OpenGL errors.
#[derive(Debug)]
pub enum GLError {
    /// Indicates that an unexpected enum value was passed to a function.
    InvalidEnum,

    /// Indicates that a function was passed an argument with a
    /// value that had an unexpected value (such as passing a negative
    /// value to a function that only expects positive arguments).
    InvalidValue,

    /// Indicates that a particular operation was attempted that is
    /// not allowed, often resulting from an unexpected pairing
    /// of arguments.
    InvalidOperation,

    /// Indicates that a particular operation attempted to use a
    /// framebuffer that is not framebuffer-complete.
    InvalidFramebufferOperation,

    /// Indicates that the OpenGL driver could not allocate
    /// enough memory to satisfy a request.
    OutOfMemory,

    /// Indicates a framebuffer-related error.
    FramebufferError(GLFramebufferError),

    /// Indicates an error with a message attached (such as
    /// a message from an info log, or an error message
    /// originating from glitter).
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

/// The possible framebuffer-incomplete errors.
#[derive(Debug)]
pub enum GLFramebufferError {
    /// Not all framebuffer attachments are [attachment-complete]
    /// (https://www.opengl.org/wiki/Framebuffer_Object#Attachment_Completeness).
    IncompleteAttachment,

    /// Not all attachments have the same dimensions.
    IncompleteDimensions,

    /// The framebuffer has no attachments.
    IncompleteMissingAttachment,

    /// The combination of attachment formats is unsupported by the current
    /// OpenGL implementation.
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
    /// The primitive drawing modes for drawing raw vertex data.
    pub gl_enum DrawingMode {
        /// Draw each vertex as a single point.
        pub const Points as POINTS = gl::POINTS,

        /// Draw a connected line segment, where each vertex is connected
        /// to the next. The first and last vertex are treated as the
        /// start and end points.
        pub const LineStrip as LINE_STRIP = gl::LINE_STRIP,

        /// Draw a self-connected line segment, where each vertex is connected
        /// to the next, and the last vertex connects to the first.
        pub const LineLoop as LINE_LOOP = gl::LINE_LOOP,

        /// Draw each pair of vertices as individual line segments.
        pub const Lines as LINES = gl::LINES,

        /// Draw the vertices as a strip of triangles.
        /// The first three vertices form the first triangle, then
        /// the next vertex plus the previous two vertices form the next
        /// triangle, and so on. For example, vertices v1, v2, and v3 form
        /// the first triangle, then vertices v2, v3, and v4 form the next,
        /// and so on.
        pub const TriangleStrip as TRIANGLE_STRIP = gl::TRIANGLE_STRIP,

        /// Draw the vertices as a triangle fan. The first vertex, v1
        /// is the fan's 'center'. Vertices v2 and v3 form the first triangle
        /// with the center, v1. Then vertices v3, v4, and v1 form the next
        /// triangle, then vertices v4, v5, and v1, and so on.
        pub const TriangleFan as TRIANGLE_FAN = gl::TRIANGLE_FAN,

        /// Draw each group of three vertices as a triangle.
        pub const Triangles as TRIANGLES = gl::TRIANGLES
    }
}

gl_enum! {
    /// The different OpenGL data types.
    pub gl_enum DataType {
        /// A signed 8-bit byte.
        pub const Byte as BYTE = gl::BYTE,

        /// An unsigned 8-bit byte.
        pub const UnsignedByte as UNSIGNED_BYTE = gl::UNSIGNED_BYTE,

        /// A signed 16-bit short.
        pub const Short as SHORT = gl::SHORT,

        /// An unsigned 16-bit short.
        pub const UnsignedShort as UNSIGNED_SHORT = gl::UNSIGNED_SHORT,

        /// A signed 32-bit, fixed-point number in 16.16 form.
        pub const Fixed as FIXED = gl::FIXED,

        /// A 32-bit, IEEE floating-point number.
        pub const Float as FLOAT = gl::FLOAT
    }
}
