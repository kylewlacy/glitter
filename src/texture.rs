use std::marker::PhantomData;
use gl;
use gl::types::*;

pub struct Texture<T: TextureType> {
    gl_id: GLuint,
    phantom: PhantomData<*mut T>
}

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
