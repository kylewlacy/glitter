use std::marker::PhantomData;
use gl;
use gl::types::*;
use types::GLObject;

pub struct Texture<T: TextureType> {
    gl_id: GLuint,
    phantom: PhantomData<*mut T>
}
pub type Texture2d = Texture<Tx2d>;
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



pub trait ImageTargetType {
    fn gl_enum(&self) -> GLenum;
}

pub trait TextureType {
    type ImageTargetType: ImageTargetType;

    fn target() -> TextureBindingTarget;
}

pub struct Tx2d;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tx2dImageTarget {
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

pub struct TxCubeMap;

gl_enum! {
    pub gl_enum TxCubeMapImageTarget {
        pub const CubeMapPositiveX as TEXTURE_CUBE_MAP_POSITIVE_X =
            gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        pub const CubeMapNegativeX as TEXTURE_CUBE_MAP_NEGATIVE_X =
            gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        pub const CubeMapPositiveY as TEXTURE_CUBE_MAP_POSITIVE_Y =
            gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        pub const CubeMapNegativeY as TEXTURE_CUBE_MAP_NEGATIVE_Y =
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        pub const CubeMapPositiveZ as TEXTURE_CUBE_MAP_POSITIVE_Z =
            gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
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



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextureBindingTarget {
    Texture2d = gl::TEXTURE_2D as isize,
    TextureCubeMap = gl::TEXTURE_CUBE_MAP as isize
}

impl TextureBindingTarget {
    pub fn gl_enum(&self) -> GLenum {
        *self as GLenum
    }
}

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

pub const TEXTURE_CUBE_MAP : TextureBindingTarget =
    TextureBindingTarget::TextureCubeMap;
pub const TEXTURE_2D : VariantTexture2d = VariantTexture2d;



// TODO: Use type refinements someday...
#[derive(Debug, Clone, Copy)]
pub enum TextureFilter {
    Nearest,
    Linear
}

#[derive(Debug, Clone, Copy)]
pub enum TextureMipmapFilter {
    Filter(TextureFilter),
    MipmapFilter { criterion: TextureFilter, mipmap: TextureFilter }
}

pub const NEAREST : TextureFilter = TextureFilter::Nearest;
pub const LINEAR : TextureFilter = TextureFilter::Linear;
pub const NEAREST_MIPMAP_NEAREST : TextureMipmapFilter =
    TextureMipmapFilter::MipmapFilter {
        criterion: TextureFilter::Nearest,
        mipmap: TextureFilter::Nearest
    };
pub const LINEAR_MIPMAP_NEAREST : TextureMipmapFilter =
    TextureMipmapFilter::MipmapFilter {
        criterion: TextureFilter::Linear,
        mipmap: TextureFilter::Nearest
    };
pub const NEAREST_MIPMAP_LINEAR : TextureMipmapFilter =
    TextureMipmapFilter::MipmapFilter {
        criterion: TextureFilter::Nearest,
        mipmap: TextureFilter::Linear
    };
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
    pub gl_enum TextureWrapMode {
        pub const ClampToEdge as CLAMP_TO_EDGE = gl::CLAMP_TO_EDGE,
        pub const MirroredRepeat as MIRRORED_REPEAT = gl::MIRRORED_REPEAT,
        pub const Repeat as REPEAT = gl::REPEAT
    }
}
