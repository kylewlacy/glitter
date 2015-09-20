use std::mem;
use std::slice;
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

impl Image2d for Pixels {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn format(&self) -> ImageFormat {
        ImageFormat::rgba8()
    }

    fn textel_bytes(&self) -> &[u8] {
        let len = self.pixels.len() * mem::size_of::<Pixel>();
        unsafe {
            slice::from_raw_parts(mem::transmute(&self.pixels[0]), len)
        }
    }
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

impl ImageFormat {
    fn rgba8() -> Self {
        ImageFormat {
            textel_type: TextelType::UnsignedByte,
            textel_format: TextelFormat::RGBA
        }
    }
}
