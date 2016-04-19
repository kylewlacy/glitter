//! Exposes the OpenGL [`Texture`](struct.Texture.html) family of objects and
//! related types.

use std::marker::PhantomData;
use gl;
use gl::types::*;
use types::GLObject;

/// A type of OpenGL texture.
///
/// OpenGL supports a multitude of different types of textures. The simplest
/// of these is a [`Texture2d`](type.Texture2d.html), which is, as the name
/// suggests, a flat, 2-dimensional image. Textures serve two primary purposes
/// in OpenGL: to be used in a [`Shader`](../shader/struct.Shader.html) to
/// be used while drawing (such as to paint a polygon with an image), or
/// to be used as an attachment for a [`Framebuffer`]
/// (../framebuffer/struct.Framebuffer.html), which can be used for
/// postprocessing effects (such as applying a gaussian blur to a scene).
///
/// In glitter, the `Texture` type has a generic type parameter, which is used
/// to represent, at the type level, what type of image data a specific
/// texture contains. For simplicity, the [`Texture2d`](type.Texture2d.html)
/// and [`TextureCubeMap`](type.TextureCubeMap.html) type aliases are provided.
///
/// All textures will be automatically deleted after going out of scope.
///
/// For details about the different types of textures
/// read the documentation for the [`TextureBindingTarget`]
/// (enum.TextureBindingTarget.html) enum. For more details about the
/// generic type parameter of the `Texture` type, read the documentation for
/// the [`TextureType`](trait.TextureType.html) trait.
///
/// # See also
/// [`gl.build_texture_2d`](trait.ContextTextureBuilderExt.html#method.build_texture_2d):
/// Build a new [`Texture2d`](type.Texture2d.html).
///
/// [`gl.gen_texture`](../context/texture_context/trait.ContextTextureExt.html#method.gen_texture):
/// Create a new, uninitialized texture. Note that this is currently the only
/// way to create textures that aren't 2D.
///
/// [`context::texture_units`](../context/texture_units/index.html): The module
/// with details details about binding a texture in a context.
pub struct Texture<T: TextureType> {
    gl_id: GLuint,
    phantom: PhantomData<*mut T>
}

/// An OpenGL texture with 2-dimensional image data.
///
/// See the documentation for [`Texture`](struct.Texture.html) for
/// more details about textures in glitter, and [`TextureBindingTarget`]
/// (enum.TextureBindingTarget) for details about the different types
/// of textures.
pub type Texture2d = Texture<Tx2d>;

/// An OpenGL texture used to hold a cubemap texture, made up of 6
/// 2-dimensional images (one for each face of a cube).
///
/// See the documentation for [`Texture`](struct.Texture.html) for
/// more details about textures in glitter, and [`TextureBindingTarget`]
/// (enum.TextureBindingTarget) for details about the different types
/// of textures.
pub type TextureCubeMap = Texture<TxCubeMap>;

impl<T: TextureType> Drop for Texture<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.gl_id as *const GLuint);
        }
    }
}

impl<T: TextureType> GLObject for Texture<T> {
    type Id = GLuint;

    unsafe fn from_raw(id: Self::Id) -> Self {
        Texture {
            gl_id: id,
            phantom: PhantomData
        }
    }

    fn id(&self) -> Self::Id {
        self.gl_id
    }
}



/// A trait implemented for types that are used to represent all of the
/// possible 2D images that make up a specific implementation of
/// [`TextureType`](trait.TextureType.html). For more details, read the
/// [`TextureType`](trait.TextureType.html) documentation.
pub trait ImageTargetType {
    /// Get the raw OpenGL enum value for an image target.
    fn gl_enum(&self) -> GLenum;
}

/// A trait implemented for a type that represent a type of texture (such
/// as 2D textures or cube map textures).  For example, [`TxCubeMap`]
/// (struct.TxCubeMap.html) is a type that implements `TextureType`, and
/// it represents cube map textures.
pub trait TextureType {
    /// The type that is used to indicate all of the possible target 2D images
    /// for this type of texture. The associated `ImageTargetType` in the impl
    /// for [`TxCubeMap`](struct.TxCubeMap.html), for example, is
    /// [`TxCubeMapImageTarget`] (enum.TxCubeMapImageTarget.html), which is an
    /// enum with six variants, one for each of the six 2-dimensional images
    /// that make up a cube map.
    type ImageTargetType: ImageTargetType;

    /// The actual variant that represents this type of texture. The
    /// `target()` method impl for [`TxCubeMap`](struct.TxCubeMap.html), for
    /// example, returns `TextureBindingTarget::CubeMap`.
    fn target() -> TextureBindingTarget;
}

/// The [`TextureType`](trait.TextureType.html) for 2-dimensional textures.
pub struct Tx2d;

/// The possible image targets for `GL_TEXTURE_2D` (only one variant,
/// since this *is* the 2D texture).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tx2dImageTarget {
    /// The only possible target for a 2-dimensional texture.
    Texture2d = gl::TEXTURE_2D as isize
}

impl ImageTargetType for Tx2dImageTarget {
    fn gl_enum(&self) -> GLenum {
        *self as GLenum
    }
}

impl TextureType for Tx2d {
    type ImageTargetType = Tx2dImageTarget;

    fn target() -> TextureBindingTarget {
        TextureBindingTarget::Texture2d
    }
}

/// The [`TextureType`](trait.TextureType.html) for cubemap textures.
pub struct TxCubeMap;

gl_enum! {
    /// The possible 2D image targets for a cubemap texture.
    pub gl_enum TxCubeMapImageTarget {
        /// The positive-X image target face of a cubemap.
        pub const CubeMapPositiveX as TEXTURE_CUBE_MAP_POSITIVE_X =
            gl::TEXTURE_CUBE_MAP_POSITIVE_X,

        /// The negative-X image target face of a cubemap.
        pub const CubeMapNegativeX as TEXTURE_CUBE_MAP_NEGATIVE_X =
            gl::TEXTURE_CUBE_MAP_NEGATIVE_X,

        /// The positive-Y image target face of a cubemap.
        pub const CubeMapPositiveY as TEXTURE_CUBE_MAP_POSITIVE_Y =
            gl::TEXTURE_CUBE_MAP_POSITIVE_Y,

        /// The negative-Y image target face of a cubemap.
        pub const CubeMapNegativeY as TEXTURE_CUBE_MAP_NEGATIVE_Y =
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,

        /// The positive-Z image target face of a cubemap.
        pub const CubeMapPositiveZ as TEXTURE_CUBE_MAP_POSITIVE_Z =
            gl::TEXTURE_CUBE_MAP_POSITIVE_Z,

        /// The negative-Z image target face of a cubemap.
        pub const CubeMapNegativeZ as TEXTURE_CUBE_MAP_NEGATIVE_Z =
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Z
    }
}

impl ImageTargetType for TxCubeMapImageTarget {
    fn gl_enum(&self) -> GLenum {
        self.gl_enum()
    }
}

impl TextureType for TxCubeMap {
    type ImageTargetType = TxCubeMapImageTarget;

    fn target() -> TextureBindingTarget {
        TextureBindingTarget::TextureCubeMap
    }
}



/// Represents all of the possible types of OpenGL textures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureBindingTarget {
    /// A 2-dimensional texture, which can be thought of as a 2D grid of colors.
    Texture2d = gl::TEXTURE_2D as isize,

    /// A cubemap texture, which is a texture made up of six 2-dimensional
    /// images, each of which represent a face of a cube. This type of texture
    /// is especially useful for skyboxes.
    TextureCubeMap = gl::TEXTURE_CUBE_MAP as isize
}

impl TextureBindingTarget {
    /// Convert a `TextureBindingTarget` into a raw OpenGL enum value.
    pub fn gl_enum(&self) -> GLenum {
        *self as GLenum
    }
}

/// This is a unit type that is used to be coerced into select enum variants.
///
/// In OpenGL, the `GL_TEXTURE_2D` enum value is used for multiple purposes. In
/// particular, it can be used as *both* a valid argument for [`glBindTexture`]
/// (http://docs.gl/es2/glBindTexture) (where it represents a possible texture
/// binding, which maps to [`TextureBindingTarget::Texture2d`]
/// (enum.TextureBindingTarget.html) in glitter) *and* for [`glTexImage2D`]
/// (http://docs.gl/es2/glTexImage2D) (where it represents the possible
/// *2D images* that make up a particular texture type, which maps to
/// the [`ImageTargetType`](trait.ImageTargetType.html) trait and
/// the [`Tx2dImageTarget`](enum.Tx2dImageTarget.html) enum in glitter).
///
/// Thus, in order to be able to use the constant `glitter::TEXTURE_2D` as an
/// argument to the [`image_2d`](../context/texture_context/trait.ContextTextureExt.html#method.image_2d)
/// method, `glitter::TEXTURE_2D` must be of type [`TextureBindingTarget`]
/// (enum.TextureBindingTarget.html). But, in order to be able to it as an
/// argument to a method that takes [`Tx2dImageTarget`]
/// (enum.Tx2dImageTarget.html), it must be of type [`Tx2dImageTarget`]
// / (enum.Tx2dImageTarget.html). Obviously, glitter::TEXTURE_2D` can't be
/// *both* types, so this type exists solely to be the type of
/// `glitter::TEXTURE_2D`, and it implements `From` so that it can be converted
/// to either type freely.

// HACK: Done to allow glitter::TEXTURE_2D as both a
//       TextureBindingTarget and Tx2dImageTarget
pub struct VariantTexture2d;

impl From<VariantTexture2d> for TextureBindingTarget {
    fn from(_: VariantTexture2d) -> TextureBindingTarget {
        TextureBindingTarget::Texture2d
    }
}

impl From<VariantTexture2d> for Tx2dImageTarget {
    fn from(_: VariantTexture2d) -> Tx2dImageTarget {
        Tx2dImageTarget::Texture2d
    }
}

impl ImageTargetType for VariantTexture2d {
    fn gl_enum(&self) -> GLenum {
        gl::TEXTURE_2D
    }
}

/// This constant is designed to be used in glitter wherever the constant
/// `GL_TEXTURE_CUBE_MAP` is used in plain OpenGL code.
pub const TEXTURE_CUBE_MAP : TextureBindingTarget =
    TextureBindingTarget::TextureCubeMap;


/// This constant is designed to be used in glitter wherever the constant
/// `GL_TEXTURE_2D` is used in plain OpenGL code.
///
/// See the documentation for [`VariantTexture2d`](struct.VariantTexture2d.html)
/// for details about how this works.
pub const TEXTURE_2D : VariantTexture2d = VariantTexture2d;


/// Represents the different forms of texture filtering, which determines
/// how a texture will be sampled when drawn.

// TODO: Use type refinements someday...
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureFilter {
    /// When texturing a pixel, return the texel that is nearest to the center
    /// of the pixel.
    Nearest,

    /// When texturing a pixel, return a weighted average of the four texels
    /// nearest to center of the pixel.
    Linear
}

/// Represents the different forms of texture filtering when using mipmaps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureMipmapFilter {
    /// Ignore mipmap values, and texture a pixel using a standard
    /// [`TextureFilter`](enum.TextureFilter.html).
    Filter(TextureFilter),

    /// Select the two mipmaps that are closest to the size of pixel
    /// being filled, and sample each one according to `criterion`.
    /// Finally, the result will be computed either by taking the
    /// weighted average of each texel, or by selecting the value
    /// from the closer texel, according to `mipmap`.
    MipmapFilter {
        /// The method to use to select the texels from a mipmap.
        criterion: TextureFilter,

        /// The method to use to select the mipmaps.
        mipmap: TextureFilter
    }
}

/// When texturing a pixel, select the texel that is closest
/// to the center of the pixel.
pub const NEAREST : TextureFilter = TextureFilter::Nearest;

/// When texturing a pixel, select the four texels that
/// are closest to the center of the pixel, and compute the
/// result by taking a weighted average of each texel.
pub const LINEAR : TextureFilter = TextureFilter::Linear;

/// When texturing a pixel, select the mipmap that is nearest
/// in size to the pixel, and select the texel that is
/// closest to the center of the pixel.
pub const NEAREST_MIPMAP_NEAREST : TextureMipmapFilter =
    TextureMipmapFilter::MipmapFilter {
        criterion: TextureFilter::Nearest,
        mipmap: TextureFilter::Nearest
    };

/// When texturing a pixel, select the mipmap that is nearest
/// in size to the pixel, select the four texels that are closest
/// to the center of the pixel, and compute the result by taking
/// the weighted average of each texel.
pub const LINEAR_MIPMAP_NEAREST : TextureMipmapFilter =
    TextureMipmapFilter::MipmapFilter {
        criterion: TextureFilter::Linear,
        mipmap: TextureFilter::Nearest
    };

/// When texturing a pixel, select the two mipmaps that are nearest
/// in size to the pixel, select the texel in each that is closest
/// to the center of the pixel, and compute the result by taking
/// the weighted average of each texel.
pub const NEAREST_MIPMAP_LINEAR : TextureMipmapFilter =
    TextureMipmapFilter::MipmapFilter {
        criterion: TextureFilter::Nearest,
        mipmap: TextureFilter::Linear
    };

/// When texturing a pixel, select the two mipmaps that are nearest
/// in size to the pixel. For each, select the four texels that are
/// closest to the center of the pixel, and compute the weighted average.
/// Finally, take the resulting two weighted averages of the texels,
/// and take the weighted average of both based on the mipmaps.
pub const LINEAR_MIPMAP_LINEAR : TextureMipmapFilter =
    TextureMipmapFilter::MipmapFilter {
        criterion: TextureFilter::Linear,
        mipmap: TextureFilter::Linear
    };

#[allow(dead_code)]
impl TextureFilter {
    fn from_gl(gl_enum: GLenum) -> Result<Self, ()> {
        match gl_enum {
            gl::NEAREST => { Ok(self::NEAREST) },
            gl::LINEAR => { Ok(self::LINEAR) },
            _ => { Err(()) }
        }
    }

    /// Convert a `TextureFilter` into a raw OpenGL enum value
    pub fn gl_enum(&self) -> GLenum {
        match *self {
            self::NEAREST => gl::NEAREST,
            self::LINEAR => gl::LINEAR
        }
    }
}

#[allow(dead_code)]
impl TextureMipmapFilter {
    fn from_gl(gl_enum: GLenum) -> Result<Self, ()> {
        match gl_enum {
            gl::NEAREST => { Ok(TextureMipmapFilter::Filter(self::NEAREST)) },
            gl::LINEAR => { Ok(TextureMipmapFilter::Filter(self::LINEAR)) },
            gl::NEAREST_MIPMAP_NEAREST => { Ok(self::NEAREST_MIPMAP_NEAREST) },
            gl::LINEAR_MIPMAP_NEAREST => { Ok(self::LINEAR_MIPMAP_NEAREST) },
            gl::NEAREST_MIPMAP_LINEAR => { Ok(self::NEAREST_MIPMAP_LINEAR) },
            gl::LINEAR_MIPMAP_LINEAR => { Ok(self::LINEAR_MIPMAP_LINEAR) },
            _ => { Err(()) }
        }
    }

    /// Convert a `TextureMipmapFilter` into a raw OpenGL enum value
    pub fn gl_enum(&self) -> GLenum {
        match *self {
            TextureMipmapFilter::Filter(self::LINEAR) => { gl::LINEAR },
            TextureMipmapFilter::Filter(self::NEAREST) => { gl::NEAREST },
            self::NEAREST_MIPMAP_NEAREST => { gl::NEAREST_MIPMAP_NEAREST },
            self::LINEAR_MIPMAP_NEAREST => { gl::LINEAR_MIPMAP_NEAREST },
            self::NEAREST_MIPMAP_LINEAR => { gl::NEAREST_MIPMAP_LINEAR },
            self::LINEAR_MIPMAP_LINEAR => { gl::LINEAR_MIPMAP_LINEAR }
        }
    }
}

impl From<TextureFilter> for TextureMipmapFilter {
    fn from(filter: TextureFilter) -> TextureMipmapFilter {
        TextureMipmapFilter::Filter(filter)
    }
}

gl_enum! {
    /// The wrapping modes when drawing a texture.
    pub gl_enum TextureWrapMode {
        /// Wrap a texture by clamping it within the range `[1/2x, 1 - 1/2x]`,
        /// where `x` is the dimension of the texture being clamped.
        pub const ClampToEdge as CLAMP_TO_EDGE = gl::CLAMP_TO_EDGE,

        /// Wrap a texture by repeating it front-to-back, then back-to-front,
        /// then repeating.
        pub const MirroredRepeat as MIRRORED_REPEAT = gl::MIRRORED_REPEAT,

        /// Wrap a texture by repeating it over and over again.
        pub const Repeat as REPEAT = gl::REPEAT
    }
}
