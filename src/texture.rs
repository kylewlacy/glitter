use gl;
use gl::types::*;

pub trait ImageTargetType {
    fn gl_enum(&self) -> GLenum;
}

gl_enum! {
    pub gl_enum TextureBindingTarget {
        Texture2d as TEXTURE_2D = gl::TEXTURE_2D,
        TextureCubeMap as TEXTURE_CUBE_MAP = gl::TEXTURE_CUBE_MAP
    }
}
