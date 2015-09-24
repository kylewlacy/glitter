use std::ptr;
use std::marker::PhantomData;
use gl;
use gl::types::*;
use context::Context;
use image_data::{Image2d, TextelFormat, ImageFormat};
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
    pub unsafe fn gen_texture<T: TextureType>(&self) -> Texture<T> {
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

gl_enum! {
    pub gl_enum TextureWrapMode {
        ClampToEdge as CLAMP_TO_EDGE = gl::CLAMP_TO_EDGE,
        MirroredRepeat as MIRRORED_REPEAT = gl::MIRRORED_REPEAT,
        Repeat as REPEAT = gl::REPEAT
    }
}



unsafe fn _tex_parameter_iv(target: TextureBindingTarget,
                            pname: GLenum,
                            params: *const GLint)
{
    gl::TexParameteriv(target.gl_enum(), pname, params);
    dbg_gl_sanity_check! {
        GLError::InvalidEnum => "`target` or `pname` is not an accepted defined value, or `params` should have defined a symbolic constant and does not",
        _ => "Unknown error"
    }
}

unsafe fn _tex_image_2d<T: ImageTargetType>(target: T,
                                            level: u32,
                                            internal_format: TextelFormat,
                                            width: u32,
                                            height: u32,
                                            border: u32,
                                            format: ImageFormat,
                                            image_ptr: *const u8) {
    debug_assert!(internal_format == format.textel_format);
    gl::TexImage2D(target.gl_enum(),
                   level as GLint,
                   internal_format.gl_enum() as GLint,
                   width as GLint,
                   height as GLint,
                   border as GLint,
                   format.textel_format.gl_enum(),
                   format.textel_type.gl_enum(),
                   image_ptr as *const GLvoid);
    dbg_gl_sanity_check! {
        GLError::InvalidEnum => "`target`, `format`, or `type` is not an accepted value",
        GLError::InvalidValue => "`target`, `level`, `internalformat`, `width`, `height`, or `border` is an invalid value",
        GLError::InvalidOperation => "`format` conflicts with either `internalformat` or `type`",
        _ => "Unknown error"
    }
}

pub trait TextureBinding {
    type TextureType: TextureType;

    fn target() -> TextureBindingTarget {
        Self::TextureType::target()
    }

    fn set_min_filter<F: Into<TextureMipmapFilter>>(&mut self, filter: F) {
        let gl_int = filter.into().gl_enum() as GLint;
        unsafe {
            _tex_parameter_iv(Self::target(),
                              gl::TEXTURE_MIN_FILTER,
                              &gl_int as *const GLint);
        }
    }

    fn set_mag_filter(&mut self, filter: TextureFilter) {
        let gl_int = filter.gl_enum() as GLint;
        unsafe {
            _tex_parameter_iv(Self::target(),
                              gl::TEXTURE_MAG_FILTER,
                              &gl_int as *const GLint);
        }
    }

    fn set_wrap_s(&mut self, wrap_mode: TextureWrapMode) {
        let gl_int = wrap_mode.gl_enum() as GLint;
        unsafe {
            _tex_parameter_iv(Self::target(),
                              gl::TEXTURE_WRAP_S,
                              &gl_int as *const GLint);
        }
    }

    fn set_wrap_t(&mut self, wrap_mode: TextureWrapMode) {
        let gl_int = wrap_mode.gl_enum() as GLint;
        unsafe {
            _tex_parameter_iv(Self::target(),
                              gl::TEXTURE_WRAP_T,
                              &gl_int as *const GLint);
        }
    }

    fn image_2d<T, I>(&mut self,
                      target: T,
                      level: u32,
                      img: &I)
        where T: Into<<Self::TextureType as TextureType>::ImageTargetType>,
              I: Image2d
    {
        unsafe {
            _tex_image_2d(target.into(),
                          level,
                          img.format().textel_format,
                          img.width() as u32,
                          img.height() as u32,
                          0,
                          img.format(),
                          img.textel_bytes().as_ptr());
        }
    }

    fn image_2d_empty<T: ImageTargetType>(&mut self,
                                          target: T,
                                          level: u32,
                                          format: ImageFormat,
                                          width: u32,
                                          height: u32)
    {
        unsafe {
            _tex_image_2d(target,
                          level,
                          format.textel_format,
                          width,
                          height,
                          0,
                          format,
                          ptr::null());
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
    pub fn bind(&mut self, texture: &mut Texture2d) -> Texture2dBinding {
        unsafe {
            _bind_texture(texture);
        }
        Texture2dBinding { phantom: PhantomData }
    }
}

pub struct TextureCubeMapBinder;
impl TextureCubeMapBinder {
    pub fn bind(&mut self, texture: &mut TextureCubeMap)
        -> TextureCubeMapBinding
    {
        unsafe {
            _bind_texture(texture);
        }
        TextureCubeMapBinding { phantom: PhantomData }
    }
}
