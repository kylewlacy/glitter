use std::mem;
use std::marker::PhantomData;
use gl;
use gl::types::*;
use context::Context;
use image_data::Image2d;
use types::GLError;

pub struct Texture<T: TextureType> {
    gl_id: GLuint,
    phantom: PhantomData<*mut T>
}
pub type Texture2d = Texture<Tx2d>;
pub type TextureCubeMap = Texture<TxCubeMap>;

impl<T: TextureType> Texture<T> {
    pub fn gl_id(&self) -> GLuint {
        self.gl_id
    }
}

impl<T: TextureType> Drop for Texture<T> {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.gl_id as *const GLuint);
        }
    }
}

impl Context {
    pub fn gen_texture<T: TextureType>(&self) -> Texture<T> {
        unsafe {
            let mut id : GLuint =  0;

            gl::GenTextures(1, &mut id as *mut GLuint);
            dbg_gl_sanity_check! {
                GLError::InvalidValue => "`n` is negative",
                _ => "Unknown error"
            }

            Texture {
                gl_id: id,
                phantom: PhantomData
            }
        }
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

gl_enum! {
    pub gl_enum Tx2dImageTarget {
        Texture2d as TEXTURE_2D_TARGET = gl::TEXTURE_2D
    }
}

impl ImageTargetType for Tx2dImageTarget {
    fn gl_enum(&self) -> GLenum {
        self.gl_enum()
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
        CubeMapPositiveX as TEXTURE_CUBE_MAP_POSITIVE_X =
            gl::TEXTURE_CUBE_MAP_POSITIVE_X,
        CubeMapNegativeX as TEXTURE_CUBE_MAP_NEGATIVE_X =
            gl::TEXTURE_CUBE_MAP_NEGATIVE_X,
        CubeMapPositiveY as TEXTURE_CUBE_MAP_POSITIVE_Y =
            gl::TEXTURE_CUBE_MAP_POSITIVE_Y,
        CubeMapNegativeY as TEXTURE_CUBE_MAP_NEGATIVE_Y =
            gl::TEXTURE_CUBE_MAP_NEGATIVE_Y,
        CubeMapPositiveZ as TEXTURE_CUBE_MAP_POSITIVE_Z =
            gl::TEXTURE_CUBE_MAP_POSITIVE_Z,
        CubeMapNegativeZ as TEXTURE_CUBE_MAP_NEGATIVE_Z =
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



gl_enum! {
    pub gl_enum TextureBindingTarget {
        Texture2d as TEXTURE_2D = gl::TEXTURE_2D,
        TextureCubeMap as TEXTURE_CUBE_MAP = gl::TEXTURE_CUBE_MAP
    }
}

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

    fn gl_enum(&self) -> GLenum {
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

    fn gl_enum(&self) -> GLenum {
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

pub trait TextureBinding {
    type TextureType: TextureType;

    fn target() -> TextureBindingTarget {
        Self::TextureType::target()
    }

    fn image_2d<I>(&mut self,
                   level: i32,
                   target: <Self::TextureType as TextureType>::ImageTargetType,
                   img: &I)
        where I: Image2d
    {
        unsafe {
            let ptr = mem::transmute(img.textel_bytes().as_ptr());
            gl::TexImage2D(target.gl_enum(),
                           level as GLint,
                           img.format().textel_format.gl_enum() as GLint,
                           img.width() as i32,
                           img.height() as i32,
                           0,
                           img.format().textel_format.gl_enum(),
                           img.format().textel_type.gl_enum(),
                           ptr);
            dbg_gl_error! {
                GLError::InvalidEnum => "`target`, `format`, or `type` is not an accepted value",
                GLError::InvalidValue => "`target`, `level`, `internalformat`, `width`, `height`, or `border` is an invalid value",
                GLError::InvalidOperation => "`format` conflicts with either `internalformat` or `type`",
                _ => "Unknown error"
            }
        }
    }
}

pub struct Texture2dBinding<'a> {
    phantom: PhantomData<&'a mut Texture2d>
}

impl<'a> TextureBinding for Texture2dBinding<'a> {
    type TextureType = Tx2d;
}

pub struct TextureCubeMapBinding<'a> {
    phantom: PhantomData<&'a mut TextureCubeMap>
}

impl<'a> TextureBinding for TextureCubeMapBinding<'a> {
    type TextureType = TxCubeMap;
}



unsafe fn _bind_texture<T: TextureType>(texture: &mut Texture<T>) {
    gl::BindTexture(T::target().gl_enum(), texture.gl_id());
    dbg_gl_error! {
        GLError::InvalidEnum => "`target` is not one of the allowed values",
        GLError::InvalidOperation => "`texture` was created with a target that doesn't match `target`",
        _ => "Unknown error"
    }
}

pub struct Texture2dBinder;
impl Texture2dBinder {
    pub fn bind<'a>(&'a mut self, texture: &mut Texture2d)
        -> Texture2dBinding<'a>
    {
        unsafe {
            _bind_texture(texture);
        }
        Texture2dBinding { phantom: PhantomData }
    }
}

pub struct TextureCubeMapBinder;
impl TextureCubeMapBinder {
    pub fn bind<'a>(&'a mut self, texture: &mut TextureCubeMap)
        -> TextureCubeMapBinding<'a>
    {
        unsafe {
            _bind_texture(texture);
        }
        TextureCubeMapBinding { phantom: PhantomData }
    }
}
