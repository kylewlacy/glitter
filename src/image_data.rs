//! Contains types related to 2D image data.

use std::ops;
use std::mem;
use std::slice;
use gl;

/// A trait for types that that contain 2D image data, which can
/// be uploaded to a texture using a [`Texture2dBuilder`]
/// (../context/texture_context/struct.Texture2dBuilder)
/// or using the [`gl.image_2d`]
/// (../context/texture_context/trait.ContextTextureExt.html#method.image_2d)
/// method.
///
/// See [`Pixels`](struct.Pixels.html) for a simple `Image2d`.
pub trait Image2d {
    /// Get the width of the image, in texels.
    fn width(&self) -> usize;

    /// Get the height of the image, in texels.
    fn height(&self) -> usize;

    /// Get the format of the image data that is returned by the
    /// [`texel_bytes`](trait.Image2d.html#tymethod.texel_bytes) method.
    fn format(&self) -> ImageFormat;

    /// Get the raw texel data of the image data, as a `u8` slice.
    fn texel_bytes(&self) -> &[u8];
}

/// A single OpenGL color value, with `u8` components laid out
/// as a C struct in RGBA order. This type is the simplest implementation
/// of [`Image2d`](trait.Image2d.html), which allows it to be uploaded
/// to a texture.
#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Pixel {
    /// The red component.
    pub r: u8,

    /// The green component.
    pub g: u8,

    /// The blue component.
    pub b: u8,

    /// The alpha component.
    pub a: u8
}

impl Pixel {
    /// Create an RGBA color, provided each color component.
    ///
    /// # Examples
    /// ```
    /// let pixel = glitter::Pixel::r_g_b_a(0xAA, 0xBB, 0xCC, 0xDD);
    /// assert_eq!(pixel.r, 0xAA);
    /// assert_eq!(pixel.g, 0xBB);
    /// assert_eq!(pixel.b, 0xCC);
    /// assert_eq!(pixel.a, 0xDD);
    /// ```
    pub fn r_g_b_a(r: u8, g: u8, b: u8, a: u8) -> Self {
        Pixel { r: r, g: g, b: b, a: a }
    }

    /// Create an RGBA color, provided the RGB components and using
    /// `0xFF` as the A value.
    ///
    /// # Examples
    /// ```
    /// let pixel = glitter::Pixel::r_g_b(0xAA, 0xBB, 0xCC);
    /// assert_eq!(pixel.r, 0xAA);
    /// assert_eq!(pixel.g, 0xBB);
    /// assert_eq!(pixel.b, 0xCC);
    /// assert_eq!(pixel.a, 0xFF);
    /// ```
    pub fn r_g_b(r: u8, g: u8, b: u8) -> Self {
        Pixel::r_g_b_a(r, g, b, 0xFF)
    }

    /// Create an RGBA color, provided the RGB components as a packed
    /// `u32` value. The `u32` value will be read as `0x00RRGGBB`.
    ///
    /// # Examples
    /// ```
    /// // The `0x55` component gets discared
    /// let pixel = glitter::Pixel::rgb(0x55AABBCC);
    /// assert_eq!(pixel.r, 0xAA);
    /// assert_eq!(pixel.g, 0xBB);
    /// assert_eq!(pixel.b, 0xCC);
    /// assert_eq!(pixel.a, 0xFF);
    /// ```
    pub fn rgb(rgb: u32) -> Self {
        Pixel::rgb_a(rgb, 0xFF)
    }

    /// Create an RGBA color value, provided the components as a packed
    /// `u32` value. The `u32` value will be read as `0xAARRGGBB`.
    ///
    /// # Examples
    /// ```
    /// let pixel = glitter::Pixel::argb(0xDDAABBCC);
    /// assert_eq!(pixel.r, 0xAA);
    /// assert_eq!(pixel.g, 0xBB);
    /// assert_eq!(pixel.b, 0xCC);
    /// assert_eq!(pixel.a, 0xDD);
    /// ```
    pub fn argb(argb: u32) -> Self {
        let a = (argb & 0xFF000000) >> 24;
        Pixel::rgb_a(argb, a as u8)
    }

    /// Create an RGBA color, provided the RGB components as a packed
    /// `u32` value. The `u32` value will be read as `0xRRGGBBAA`.
    ///
    /// # Examples
    /// ```
    /// let pixel = glitter::Pixel::rgba(0xAABBCCDD);
    /// assert_eq!(pixel.r, 0xAA);
    /// assert_eq!(pixel.g, 0xBB);
    /// assert_eq!(pixel.b, 0xCC);
    /// assert_eq!(pixel.a, 0xDD);
    /// ```
    pub fn rgba(rgba: u32) -> Self {
        let r = (rgba & 0xFF000000) >> 24;
        let g = (rgba & 0x00FF0000) >> 16;
        let b = (rgba & 0x0000FF00) >> 8;
        let a =  rgba & 0x000000FF;
        Pixel::r_g_b_a(r as u8, g as u8, b as u8, a as u8)
    }

    /// Create an RGBA color, provided the RGB components as a packed
    /// `u32` value, and a separate A component. The `u32` value
    /// will be read as `0x00RRGGBB`.
    ///
    /// # Examples
    /// ```
    /// // The `0x55` component gets discarded
    /// let pixel = glitter::Pixel::rgb_a(0x55AABBCC, 0xDD);
    /// assert_eq!(pixel.r, 0xAA);
    /// assert_eq!(pixel.g, 0xBB);
    /// assert_eq!(pixel.b, 0xCC);
    /// assert_eq!(pixel.a, 0xDD);
    /// ```
    pub fn rgb_a(rgb: u32, a: u8) -> Self {
        let r = (rgb & 0xFF0000) >> 16;
        let g = (rgb & 0x00FF00) >> 8;
        let b =  rgb & 0x0000FF;
        Pixel::r_g_b_a(r as u8, g as u8, b as u8, a)
    }
}

/// A (heap-allocated) 2D image composed of a list of pixels.
///
/// # Example
///
/// ```
/// // Draw a red circle with a 50px radius in a 100px * 100px black image.
/// extern crate glitter;
///
/// let width = 100;
/// let height = 100;
/// let radius = 50;
///
// let (center_x, center_y) = (width as f32/2.0, height as f32/2.0);
//
// let mut pixels = glitter::Pixels::new(width, height);
// for x in 0..width {
//     for y in 0..height {
//         let dx = center_x - x as f32;
//         let dy = center_y - y as f32;
//         let distance = (dx*dx + dy*dy).sqrt();
//
//         let color = if distance < radius {
//             // The point is within the circle, so it should be red
//             glitter::Pixel::rgb(0xFF0000)
//         }
//         else {
//             // The point is outside the circle, so it should be black
//             glitter::Pixel::rgb(0x000000)
//         };
//         pixels[y][x] = color;
//     }
// }
// ```
pub struct Pixels {
    width: usize,
    height: usize,
    pixels: Vec<Pixel>
}

impl Pixels {
    /// Create a new image with the desired width and height.
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
    /// The data types for a texel.
    pub gl_enum TexelType {
        /// Each texel is 4 * 8 bits.
        pub const UnsignedByte as UNSIGNED_BYTE_TEXEL =
            gl::UNSIGNED_BYTE,

        /// Each texel is 16 bits, with 5-bit/6-bit/5-bit components.
        pub const UnsignedShort565 as UNSIGNED_SHORT_5_6_5 =
            gl::UNSIGNED_SHORT_5_6_5,

        /// Each texel is 16 bits, with 4, 4-bit components.
        pub const UnsignedShort4444 as UNSIGNED_SHORT_4_4_4_4 =
            gl::UNSIGNED_SHORT_4_4_4_4,

        /// Each texel is 16 bits, with 5-bit/5-bit/5-bit/1-bit components.
        pub const UnsignedShort5551 as UNSIGNED_SHORT_5_5_5_1 =
            gl::UNSIGNED_SHORT_5_5_5_1
    }
}

gl_enum! {
    /// The different texel formats.
    pub gl_enum TexelFormat {
        /// A texel contains only an alpha component.
        pub const Alpha as ALPHA = gl::ALPHA,

        /// A texel contains red, green, and blue components.
        pub const RGB as RGB = gl::RGB,

        /// A texel contains red, green, blue, and alpha components.
        pub const RGBA as RGBA = gl::RGBA
    }
}

gl_enum! {
    /// The various image formats of a renderbuffer.
    pub gl_enum RenderbufferFormat {
        /// The red, green, blue, and alpha channels are all stored with 4 bits.
        pub const RGBA4 as RGBA4 = gl::RGBA4,

        /// The red, green, and blue channels are stored with 5 bits, 6 bits,
        /// and 5 bits, respectively.
        pub const RGB565 as RGB565 = gl::RGB565,

        /// The red, green, and blue channels are stored with 5 bits, and the
        /// alpha channel is stored with 1 bit.
        pub const RGB5A1 as RGB5_A1 = gl::RGB5_A1,

        /// The renderbuffer stores a 16-bit depth component.
        pub const DepthComponent16 as DEPTH_COMPONENT16 = gl::DEPTH_COMPONENT16,

        /// The renderbuffer stores an 8-bit stencil component.
        pub const StencilIndex8 as STENCIL_INDEX8 = gl::STENCIL_INDEX8
    }
}

/// A type that contains the texel type and format that make up a 2D image.
#[derive(Debug, Clone, Copy)]
pub struct ImageFormat {
    /// The texel type of the image.
    pub texel_type: TexelType,

        /// The texel format of the image.
    pub texel_format: TexelFormat
}

impl ImageFormat {
    /// Returns the RGBA image format with 8 bits per component.
    pub fn rgba8() -> Self {
        ImageFormat {
            texel_type: TexelType::UnsignedByte,
            texel_format: TexelFormat::RGBA
        }
    }
}
