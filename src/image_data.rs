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

impl Pixel {
    pub fn r_g_b_a(r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel { r: r, g: g, b: b, a: a }
    }

    pub fn r_g_b(r: u8, g: u8, b: u8) -> Self {
        Pixel::r_g_b_a(r, g, b, 0xFF)
    }

    pub fn rgb(rgb: u32) -> Self {
        Pixel::rgb_a(rgb, 0xFF)
    }

    pub fn argb(argb: u32) -> Self {
        let a = (argb & 0xFF000000) >> 24;
        Pixel::rgb_a(argb, a as u8)
    }

    pub fn rgba(rgba: u32) -> Self {
        let r = (rgba & 0xFF000000) >> 24;
        let g = (rgba & 0x00FF0000) >> 16;
        let b = (rgba & 0x0000FF00) >> 8;
        let a =  rgba & 0x000000FF;
        Pixel::r_g_b_a(r as u8, g as u8, b as u8, a as u8)
    }

    pub fn rgb_a(rgb: u32, a: u8) -> Self {
        let r = (rgb & 0xFF0000) >> 16;
        let g = (rgb & 0x00FF00) >> 8;
        let b =  rgb & 0x0000FF;
        Pixel::r_g_b_a(r as u8, g as u8, b as u8, a)
    }
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
