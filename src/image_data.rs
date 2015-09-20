use gl;

pub trait Image2d {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn format(&self) -> ImageFormat;
    fn textel_bytes(&self) -> &[u8];
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

pub struct Pixels {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>
}

gl_enum! {
    pub gl_enum TextelType {
        UnsignedByte as UNSIGNED_BYTE_TEXTEL = gl::UNSIGNED_BYTE,
        UnsignedShort565 as UNSIGNED_SHORT_5_6_5 = gl::UNSIGNED_SHORT_5_6_5,
        UnsignedShort4444 as UNSIGNED_SHORT_4_4_4_4 =
            gl::UNSIGNED_SHORT_4_4_4_4,
        UnsignedShort5551 as UNSIGNED_SHORT_5_5_5_1 =
            gl::UNSIGNED_SHORT_5_5_5_1
    }
}

gl_enum! {
    pub gl_enum TextelFormat {
        Alpha as ALPHA = gl::ALPHA,
        RGB as RGB = gl::RGB,
        RGBA as RGBA = gl::RGBA
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageFormat {
    pub textel_type: TextelType,
    pub textel_format: TextelFormat
}
