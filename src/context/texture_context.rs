use std::ptr;
use std::marker::PhantomData;
use gl;
use gl::types::*;
use context::{ContextOf, TextureUnit0Context, TextureUnitBinding2d};
use texture::{TextureMipmapFilter, TextureFilter, TextureWrapMode,
              Texture, Texture2d, TextureCubeMap,
              Tx2d, TxCubeMap, TextureType, Tx2dImageTarget,
              ImageTargetType, TextureBindingTarget};
use image_data::{Image2d, TextelFormat, ImageFormat};
use types::{GLObject, GLError};

pub struct Texture2dBuilder<'a, C>
    where C: 'a + TextureUnit0Context
{
    gl: C,
    min_filter: Option<TextureMipmapFilter>,
    mag_filter: Option<TextureFilter>,
    wrap_s: Option<TextureWrapMode>,
    wrap_t: Option<TextureWrapMode>,
    gen_mipmap: bool,
    image: Option<&'a Image2d>,
    empty_params: Option<(ImageFormat, u32, u32)>
}

impl<'a, C> Texture2dBuilder<'a, C>
    where C: TextureUnit0Context
{
    fn new(gl: C) -> Self {
        Texture2dBuilder {
            gl: gl,
            min_filter: None,
            mag_filter: None,
            wrap_s: None,
            wrap_t: None,
            gen_mipmap: false,
            image: None,
            empty_params: None
        }
    }

    pub fn image_2d(mut self, image: &'a Image2d) -> Self {
        self.image = Some(image);
        self
    }

    pub fn empty(mut self, format: ImageFormat, width: u32, height: u32)
        -> Self
    {
        self.empty_params = Some((format, width, height));
        self
    }

    pub fn generate_mipmap(mut self) -> Self {
        self.gen_mipmap = true;
        self
    }

    pub fn min_filter<I>(mut self, filter: I) -> Self
        where I: Into<TextureMipmapFilter>
    {
        self.min_filter = Some(filter.into());
        self
    }

    pub fn mag_filter(mut self, filter: TextureFilter) -> Self {
        self.mag_filter = Some(filter);
        self
    }

    pub fn wrap_s(mut self, wrap: TextureWrapMode) -> Self {
        self.wrap_s = Some(wrap);
        self
    }

    pub fn wrap_t(mut self, wrap: TextureWrapMode) -> Self {
        self.wrap_t = Some(wrap);
        self
    }

    pub fn try_unwrap(self) -> Result<Texture2d, GLError> {
        use TextureMipmapFilter::MipmapFilter;

        let gl = self.gl;
        let mut texture = unsafe { gl.gen_texture() };

        {
            let (gl_tex_unit, gl) = gl.active_texture_0();
            let (mut gl_tex, _) = gl_tex_unit.bind_texture_2d(&mut texture);

            if let Some(min_filter) = self.min_filter {
                gl.set_min_filter(&mut gl_tex, min_filter);
            }
            if let Some(mag_filter) = self.mag_filter {
                gl.set_mag_filter(&mut gl_tex, mag_filter);
            }
            if let Some(wrap_s) = self.wrap_s {
                gl.set_wrap_s(&mut gl_tex, wrap_s);
            }
            if let Some(wrap_t) = self.wrap_t {
                gl.set_wrap_t(&mut gl_tex, wrap_t);
            }

            // TODO: Find out what conditions lead to a non-complete texture
            //       (e.g. if either width or height are 0)
            if let Some(image) = self.image {
                gl.image_2d(&mut gl_tex, Tx2dImageTarget::Texture2d, 0, image);
            }
            else if let Some((format, width, height)) = self.empty_params {
                gl.image_2d_empty(&mut gl_tex,
                                  Tx2dImageTarget::Texture2d,
                                  0,
                                  format,
                                  width,
                                  height);

                if !(width > 0 && height > 0) {
                    let msg = "Error building texture: texture must have positive dimensions";
                    return Err(GLError::Message(msg.to_owned()))
                }
            }
            else {
                let msg = "Error building texture: neither an image nor a format were provided";
                return Err(GLError::Message(msg.to_owned()))
            }

            if self.gen_mipmap {
                gl.generate_mipmap(&mut gl_tex);
            }
            else if let Some(MipmapFilter {..}) = self.min_filter {
                    let msg = "Error building texture: texture uses a mipmap filter but does not have a mipmap";
                    return Err(GLError::Message(msg.to_owned()));
            }
        }

        Ok(texture)
    }

    pub fn unwrap(self) -> Texture2d {
        self.try_unwrap().unwrap()
    }
}

// NOTE: There is currently no way to express "a context with
//       one free texure unit"; this design should be explored for
//       cases like this (where the actual unit number doesn't matter)
pub trait ContextTextureBuilderExt: TextureUnit0Context + Sized {
    fn build_texture_2d<'a>(self) -> Texture2dBuilder<'a, Self> {
        Texture2dBuilder::new(self)
    }
}

impl<'a, C: 'a> ContextTextureBuilderExt for &'a mut C
    where &'a mut C: TextureUnit0Context
{

}



pub unsafe trait ContextTextureExt {
    unsafe fn gen_texture<TX: TextureType>(&self) -> Texture<TX> {
        let mut id : GLuint =  0;

        gl::GenTextures(1, &mut id as *mut GLuint);
        dbg_gl_sanity_check! {
            GLError::InvalidValue => "`n` is negative",
            _ => "Unknown error"
        }

        Texture::from_raw(id)
    }

    fn set_min_filter<T, F>(&self, gl_texture: &mut T, filter: F)
        where T: TextureBinding, F: Into<TextureMipmapFilter>
    {
        let gl_int = filter.into().gl_enum() as GLint;
        unsafe {
            _tex_parameter_iv(gl_texture.target(),
                              gl::TEXTURE_MIN_FILTER,
                              &gl_int as *const GLint);
        }
    }

    fn set_mag_filter<T>(&self, gl_texture: &mut T, filter: TextureFilter)
        where T: TextureBinding
    {
        let gl_int = filter.gl_enum() as GLint;
        unsafe {
            _tex_parameter_iv(gl_texture.target(),
                              gl::TEXTURE_MAG_FILTER,
                              &gl_int as *const GLint);
        }
    }

    fn set_wrap_s<T>(&self, gl_texture: &mut T, wrap_mode: TextureWrapMode)
        where T: TextureBinding
    {
        let gl_int = wrap_mode.gl_enum() as GLint;
        unsafe {
            _tex_parameter_iv(gl_texture.target(),
                              gl::TEXTURE_WRAP_S,
                              &gl_int as *const GLint);
        }
    }

    fn set_wrap_t<T>(&self, gl_texture: &mut T, wrap_mode: TextureWrapMode)
        where T: TextureBinding
    {
        let gl_int = wrap_mode.gl_enum() as GLint;
        unsafe {
            _tex_parameter_iv(gl_texture.target(),
                              gl::TEXTURE_WRAP_T,
                              &gl_int as *const GLint);
        }
    }

    fn generate_mipmap<T>(&self, gl_texture: &mut T)
        where T: TextureBinding
    {
        unsafe {
            gl::GenerateMipmap(gl_texture.target().gl_enum())
        }
    }

    fn image_2d<T, U, I: ?Sized>(&self,
                                 _gl_texture: &mut T,
                                 target: U,
                                 level: u32,
                                 img: &I)
        where T: TextureBinding,
              U: Into<<T::TextureType as TextureType>::ImageTargetType>,
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

    fn image_2d_empty<T, I>(&self,
                            _gl_texture: &mut T,
                            target: I,
                            level: u32,
                            format: ImageFormat,
                            width: u32,
                            height: u32)
        where T: TextureBinding, I: ImageTargetType
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

unsafe impl<B, F, P, R, T> ContextTextureExt for ContextOf<B, F, P, R, T> {

}

unsafe impl<'a, B, F, P, R, T> ContextTextureExt
    for &'a mut ContextOf<B, F, P, R, T>
{

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

    fn target(&self) -> TextureBindingTarget;
}

pub struct Texture2dBinding<'a> {
    phantom: PhantomData<&'a mut Texture2d>
}

impl<'a> TextureBinding for Texture2dBinding<'a> {
    type TextureType = Tx2d;

    fn target(&self) -> TextureBindingTarget {
        Tx2d::target()
    }
}

pub struct TextureCubeMapBinding<'a> {
    phantom: PhantomData<&'a mut TextureCubeMap>
}

impl<'a> TextureBinding for TextureCubeMapBinding<'a> {
    type TextureType = TxCubeMap;

    fn target(&self) -> TextureBindingTarget {
        TxCubeMap::target()
    }
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
    pub fn bind<'a>(&mut self, texture: &mut Texture2d)
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
    pub fn bind<'a>(&mut self, texture: &'a mut TextureCubeMap)
        -> TextureCubeMapBinding<'a>
    {
        unsafe {
            _bind_texture(texture);
        }
        TextureCubeMapBinding { phantom: PhantomData }
    }
}
