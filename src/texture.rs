use std::marker::PhantomData;
use gl;
use gl::types::*;

pub struct Texture<T: TextureType> {
    gl_id: GLuint,
    phantom: PhantomData<*mut T>
}



pub trait ImageTargetType {
    fn gl_enum(&self) -> GLenum;
}

pub trait TextureType {
    type ImageTargetType: ImageTargetType;

    fn target() -> TextureBindingTarget;
}



gl_enum! {
    pub gl_enum TextureBindingTarget {
        Texture2d as TEXTURE_2D = gl::TEXTURE_2D,
        TextureCubeMap as TEXTURE_CUBE_MAP = gl::TEXTURE_CUBE_MAP
    }
}
