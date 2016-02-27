use std::ops;
use std::mem;
use std::slice;
use gl;

pub trait Image2d {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn format(&self) -> ImageFormat;
    fn texel_bytes(&self) -> &[u8];
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

impl Pixels {
    pub fn new(width: usize, height: usize) -> Self {
        Pixels {
            width: width,
            height: height,
            pixels: vec![Pixel::rgb(0x0000FF); width * height]
        }
    }
}

impl ops::Index<usize> for Pixels {
    type Output = [Pixel];

    fn index(&self, row: usize) -> &[Pixel] {
        debug_assert!(row < self.height);

        &self.pixels[(row*self.width)..((row+1)*self.width)]
    }
}

impl ops::IndexMut<usize> for Pixels {
    fn index_mut(&mut self, row: usize) -> &mut [Pixel] {
        debug_assert!(row < self.height);

        &mut self.pixels[(row*self.width)..((row+1)*self.width)]
    }
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

    fn texel_bytes(&self) -> &[u8] {
        let len = self.pixels.len() * mem::size_of::<Pixel>();
        unsafe {
            slice::from_raw_parts(mem::transmute(&self.pixels[0]), len)
        }
    }
}

gl_enum! {
    pub gl_enum TexelType {
        pub const UnsignedByte as UNSIGNED_BYTE_TEXEL =
            gl::UNSIGNED_BYTE,
        pub const UnsignedShort565 as UNSIGNED_SHORT_5_6_5 =
            gl::UNSIGNED_SHORT_5_6_5,
        pub const UnsignedShort4444 as UNSIGNED_SHORT_4_4_4_4 =
            gl::UNSIGNED_SHORT_4_4_4_4,
        pub const UnsignedShort5551 as UNSIGNED_SHORT_5_5_5_1 =
            gl::UNSIGNED_SHORT_5_5_5_1
    }
}

gl_enum! {
    pub gl_enum TexelFormat {
        pub const Alpha as ALPHA = gl::ALPHA,
        pub const RGB as RGB = gl::RGB,
        pub const RGBA as RGBA = gl::RGBA
    }
}

gl_enum! {
    pub gl_enum RenderbufferFormat {
        pub const RGBA4 as RGBA4 = gl::RGBA4,
        pub const RGB565 as RGB565 = gl::RGB565,
        pub const RGB5A1 as RGB5_A1 = gl::RGB5_A1,
        pub const DepthComponent16 as DEPTH_COMPONENT16 = gl::DEPTH_COMPONENT16,
        pub const StencilIndex8 as STENCIL_INDEX8 = gl::STENCIL_INDEX8
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageFormat {
    pub texel_type: TexelType,
    pub texel_format: TexelFormat
}

impl ImageFormat {
    pub fn rgba8() -> Self {
        ImageFormat {
            texel_type: TexelType::UnsignedByte,
            texel_format: TexelFormat::RGBA
        }
    }
}
