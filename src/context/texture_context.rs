//! Contains all of the OpenGL state types related to texture bindings.

use std::ptr;
use std::marker::PhantomData;
use gl;
use gl::types::*;
use context::{BaseContext, TextureUnit0Context, TextureUnitBinding2d};
use texture::{TextureMipmapFilter, TextureFilter, TextureWrapMode,
              Texture, Texture2d, TextureCubeMap,
              Tx2d, TxCubeMap, TextureType, Tx2dImageTarget,
              ImageTargetType, TextureBindingTarget};
use image_data::{Image2d, TexelFormat, ImageFormat};
use types::{GLObject, GLError};

/// Provide a safe interface for building a 2D texture
/// object that is checked to be complete. A `Texture2dBuilder`
/// can be created using the [`gl.build_texture_2d`]
/// (trait.ContextTextureBuilderExt.html#method.build_texture_2d)
/// method.
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

    /// Set the 2D image data to fill the texture with.
    pub fn image_2d(mut self, image: &'a Image2d) -> Self {
        self.image = Some(image);
        self
    }

    /// Set the parameters for creating an empty texture.
    pub fn empty(mut self, format: ImageFormat, width: u32, height: u32)
        -> Self
    {
        self.empty_params = Some((format, width, height));
        self
    }

    /// Automatically generate mipamps for the texture.
    pub fn generate_mipmap(mut self) -> Self {
        self.gen_mipmap = true;
        self
    }

    /// Set the texture's minifying filter.
    pub fn min_filter<I>(mut self, filter: I) -> Self
        where I: Into<TextureMipmapFilter>
    {
        self.min_filter = Some(filter.into());
        self
    }

    /// Set the texture's magnifying filter.
    pub fn mag_filter(mut self, filter: TextureFilter) -> Self {
        self.mag_filter = Some(filter);
        self
    }

    /// Set the texture's wrap mode for the s-coordinate.
    pub fn wrap_s(mut self, wrap: TextureWrapMode) -> Self {
        self.wrap_s = Some(wrap);
        self
    }

    /// Set the texture's wrap mode for the t-coordinate.
    pub fn wrap_t(mut self, wrap: TextureWrapMode) -> Self {
        self.wrap_t = Some(wrap);
        self
    }

    /// Create and return a texture with the specified options,
    /// or return an error.
    ///
    /// # Failures
    /// If any of the following conditions are met, an error
    /// will be returned:
    ///
    /// - The texture is not complete.
    /// - The texture was set to be empty, but either the width
    ///   or height were 0.
    /// - The texture was neither set to be empty with [`empty`]
    ///   (struct.Texture2dBuilder.html#method.empty), nor had
    ///   any image data supplied with [`image_2d`]
    ///   (struct.Texture2dBuilder.html#method.image_2d).
    /// - The texture had a mipmap filter set for the [`min_filter`]
    ///   (struct.Texture2dBuilder.html#method.min_filter), but
    ///   mimaps were not generated using [`generate_mipmaps`]
    ///   (struct.Texture2dBuilder.html#method.generate_mipmap).
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
                gl.tex_image_2d(&mut gl_tex,
                                Tx2dImageTarget::Texture2d,
                                0,
                                image);
            }
            else if let Some((format, width, height)) = self.empty_params {
                gl.tex_image_2d_empty(&mut gl_tex,
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

    /// Create a texture with the specified options, or panic.
    ///
    /// # Panic
    /// See the [`try_unwrap`](struct.Texture2dBuilder.html#method.try_unwrap)
    /// method docs for all of the possible failure cases when building
    /// a texture.
    pub fn unwrap(self) -> Texture2d {
        self.try_unwrap().unwrap()
    }
}

// NOTE: There is currently no way to express "a context with
//       one free texure unit"; this design should be explored for
//       cases like this (where the actual unit number doesn't matter)
/// The extension trait for contexts that adds the `build_texture_2d` method.
///
/// # Note
/// Currently, this trait is only implemented for contexts where the
/// 0th texture unit is free.
pub trait ContextTextureBuilderExt: TextureUnit0Context + Sized {
    /// Create a new 2D texture builder, providing a safe interface
    /// for constructing a 2D texture object. See the [`Texture2dBuilder`]
    /// (struct.Texture2dBuilder.html) docs for more details.
    fn build_texture_2d<'a>(self) -> Texture2dBuilder<'a, Self> {
        Texture2dBuilder::new(self)
    }
}

impl<'a, C: 'a> ContextTextureBuilderExt for &'a mut C
    where &'a mut C: TextureUnit0Context
{

}



/// An extension trait that includes texture-related OpenGL methods.
pub trait ContextTextureExt: BaseContext {
    /// Create a new texture with no storage or image data.
    ///
    /// # Safety
    /// Many OpenGL functions assume a texture object will be [texture-complete]
    /// (https://www.opengl.org/wiki/Texture_Storage#Texture_completeness).
    /// Violating this invariant is considered undefined behavior.
    ///
    /// # See also
    /// [`gl.build_texture_2d`](trait.ContextTextureBuilderExt.html#method.build_texture_2d):
    /// A safe wrapper for building 2D textures.
    ///
    /// [`glGenTextures`](http://docs.gl/es2/glGenTextures) OpenGL docs
    unsafe fn gen_texture<TX: TextureType>(&self) -> Texture<TX> {
        let mut id : GLuint =  0;

        gl::GenTextures(1, &mut id as *mut GLuint);
        dbg_gl_sanity_check! {
            GLError::InvalidValue => "`n` is negative",
            _ => "Unknown error"
        }

        Texture::from_raw(id)
    }

    /// Set a texture's minifying filter.
    ///
    /// # See also
    /// [`glTexParameter`](http://docs.gl/es2/glTexParameter) OpenGL docs
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

    /// Set a texture's magnifying filter.
    ///
    /// # See also
    /// [`glTexParameter`](http://docs.gl/es2/glTexParameter) OpenGL docs
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

    /// Set a texture's wrap mode for the s-coordinate.
    ///
    /// # See also
    /// [`glTexParameter`](http://docs.gl/es2/glTexParameter) OpenGL docs
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

    /// Set a texture's wrap mode for the t-coordinate.
    ///
    /// # See also
    /// [`glTexParameter`](http://docs.gl/es2/glTexParameter) OpenGL docs
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

    /// Generate a set of mipmaps for a texture object.
    ///
    /// # See also
    /// [`glGenerateMipmap`](http://docs.gl/es2/glGenerateMipmap) OpenGL docs
    fn generate_mipmap<T>(&self, gl_texture: &mut T)
        where T: TextureBinding
    {
        unsafe {
            gl::GenerateMipmap(gl_texture.target().gl_enum())
        }
    }

    /// Upload 2D image data to a texture object's image target.
    ///
    /// - `_gl_texture`: The binding of the texture object.
    /// - `target`: The texture's 2D image target to upload the image data to.
    /// - `level`: The mipmap level to upload the image data to.
    /// - `img`: The image data to upload.
    fn tex_image_2d<T, U, I: ?Sized>(&self,
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
                          img.format().texel_format,
                          img.width() as u32,
                          img.height() as u32,
                          0,
                          img.format(),
                          img.texel_bytes().as_ptr());
        }
    }

    /// Set a texture object's image target to an empty image
    /// with the specified parameters.
    ///
    /// - `_gl_texture`: The binding of the texture object.
    /// - `target`: The texture's 2D image target to set.
    /// - `level`: The mipmap level to set.
    /// - `format`: The image format to use to use for the
    ///             texture's data store.
    /// - `width`: The width to set for the texture's data store.
    /// - `height`: The height to set for the texture's data store.
    fn tex_image_2d_empty<T, I>(&self,
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
                          format.texel_format,
                          width,
                          height,
                          0,
                          format,
                          ptr::null());
        }
    }
}

impl<C: BaseContext> ContextTextureExt for C {

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
                                            internal_format: TexelFormat,
                                            width: u32,
                                            height: u32,
                                            border: u32,
                                            format: ImageFormat,
                                            image_ptr: *const u8) {
    debug_assert!(internal_format == format.texel_format);
    gl::TexImage2D(target.gl_enum(),
                   level as GLint,
                   internal_format.gl_enum() as GLint,
                   width as GLint,
                   height as GLint,
                   border as GLint,
                   format.texel_format.gl_enum(),
                   format.texel_type.gl_enum(),
                   image_ptr as *const GLvoid);
    dbg_gl_sanity_check! {
        GLError::InvalidEnum => "`target`, `format`, or `type` is not an accepted value",
        GLError::InvalidValue => "`target`, `level`, `internalformat`, `width`, `height`, or `border` is an invalid value",
        GLError::InvalidOperation => "`format` conflicts with either `internalformat` or `type`",
        _ => "Unknown error"
    }
}

/// Represents a texture that has been bound to a texture unit.
pub trait TextureBinding {
    /// The type of texture that this binding represents.
    type TextureType: TextureType;

    /// The OpenGL texture target of this binding.
    fn target(&self) -> TextureBindingTarget;
}

/// Represents a texture that has been bound to the `GL_TEXTURE_2D` binding
/// target of a texture unit.
pub struct Texture2dBinding<'a> {
    _phantom_ref: PhantomData<&'a mut Texture2d>,
    _phantom_ptr: PhantomData<*mut ()>
}

impl<'a> TextureBinding for Texture2dBinding<'a> {
    type TextureType = Tx2d;

    fn target(&self) -> TextureBindingTarget {
        Tx2d::target()
    }
}

/// Represents a texture that has been bound to the `GL_TEXTURE_CUBE_MAP`
/// binding target of a texture unit.
pub struct TextureCubeMapBinding<'a> {
    _phantom_ref: PhantomData<&'a mut TextureCubeMap>,
    _phantom_ptr: PhantomData<*mut ()>
}

impl<'a> TextureBinding for TextureCubeMapBinding<'a> {
    type TextureType = TxCubeMap;

    fn target(&self) -> TextureBindingTarget {
        TxCubeMap::target()
    }
}



unsafe fn _bind_texture<T: TextureType>(texture: &mut Texture<T>) {
    gl::BindTexture(T::target().gl_enum(), texture.id());
    dbg_gl_error! {
        GLError::InvalidEnum => "`target` is not one of the allowed values",
        GLError::InvalidOperation => "`texture` was created with a target that doesn't match `target`",
        _ => "Unknown error"
    }
}

/// The OpenGL texture unit state that represents the `GL_TEXTURE_2D`
/// target.
pub struct Texture2dBinder {
    _phantom: PhantomData<*mut ()>
}

impl Texture2dBinder {
    /// Get the current `GL_TEXTURE_2D` binder.
    ///
    /// # Safety
    /// The same rules apply to this method as the
    /// [`ContextOf::current_context()`]
    /// (../struct.ContextOf.html#method.current_context) method.
    pub unsafe fn current() -> Self {
        Texture2dBinder {
            _phantom: PhantomData
        }
    }

    /// Bind a texture to the `GL_TEXTURE_2D` target,
    /// returning a binding.
    pub fn bind<'a>(&mut self, texture: &mut Texture2d)
        -> Texture2dBinding<'a>
    {
        unsafe {
            _bind_texture(texture);
        }
        Texture2dBinding {
            _phantom_ref: PhantomData,
            _phantom_ptr: PhantomData
        }
    }
}

/// The OpenGL texture unit state that represents the `GL_TEXTURE_CUBE_MAP`
/// target.
pub struct TextureCubeMapBinder {
    _phantom: PhantomData<*mut ()>
}

impl TextureCubeMapBinder {
    /// Get the current `GL_TEXTURE_CUBE_MAP` binder.
    ///
    /// # Safety
    /// The same rules apply to this method as the
    /// [`ContextOf::current_context()`]
    /// (../struct.ContextOf.html#method.current_context) method.
    pub unsafe fn current() -> Self {
        TextureCubeMapBinder {
            _phantom: PhantomData
        }
    }

    /// Bind a texture to the `GL_TEXTURE_CUBE_MAP` target,
    /// returning a binding.
    pub fn bind<'a>(&mut self, texture: &'a mut TextureCubeMap)
        -> TextureCubeMapBinding<'a>
    {
        unsafe {
            _bind_texture(texture);
        }
        TextureCubeMapBinding {
            _phantom_ref: PhantomData,
            _phantom_ptr: PhantomData
        }
    }
}
