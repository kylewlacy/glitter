use gl;
use gl::types::*;
use texture::{Texture2dBinder, TextureCubeMapBinder};
use types::GLError;

unsafe fn _active_texture(idx: u32) {
    gl::ActiveTexture(gl::TEXTURE0 + (idx as GLenum));
    dbg_gl_error! {
        GLError::InvalidEnum => "`texture` is out of bounds (expected to be GL_TEXTUREi, 0 <= i < GL_MAX_TEXTURE_IMAGE_UNITS)",
        _ => "Unknown error"
    }
}

pub trait TextureUnit {
    fn idx() -> u32;

    fn active(&mut self) -> TextureUnitBinding {
        unsafe {
            _active_texture(Self::idx());
            TextureUnitBinding::current_at_idx(Self::idx())
        }
    }
}

// TODO: Use a macro, or const generic parameters:
// https://github.com/rust-lang/rfcs/issues/273
// https://github.com/rust-lang/rfcs/issues/1038
pub struct TextureUnit0;
pub struct TextureUnit1;
pub struct TextureUnit2;
pub struct TextureUnit3;
pub struct TextureUnit4;
pub struct TextureUnit5;
pub struct TextureUnit6;
pub struct TextureUnit7;

impl TextureUnit for TextureUnit0 { fn idx() -> u32 { 0 } }
impl TextureUnit for TextureUnit1 { fn idx() -> u32 { 1 } }
impl TextureUnit for TextureUnit2 { fn idx() -> u32 { 2 } }
impl TextureUnit for TextureUnit3 { fn idx() -> u32 { 3 } }
impl TextureUnit for TextureUnit4 { fn idx() -> u32 { 4 } }
impl TextureUnit for TextureUnit5 { fn idx() -> u32 { 5 } }
impl TextureUnit for TextureUnit6 { fn idx() -> u32 { 6 } }
impl TextureUnit for TextureUnit7 { fn idx() -> u32 { 7 } }

// NOTE: Ensure the number of each texture unit matches its index in the tuple
pub struct TextureUnits(pub TextureUnit0, pub TextureUnit1, pub TextureUnit2,
                        pub TextureUnit3, pub TextureUnit4, pub TextureUnit5,
                        pub TextureUnit6, pub TextureUnit7);

impl TextureUnits {
    pub unsafe fn current() -> TextureUnits {
        TextureUnits(TextureUnit0, TextureUnit1, TextureUnit2, TextureUnit3,
                     TextureUnit4, TextureUnit5, TextureUnit6, TextureUnit7)
    }

    pub unsafe fn active_nth(&self, idx: u32) -> TextureUnitBinding {
        _active_texture(idx);
        TextureUnitBinding::current_at_idx(idx)
    }
}

pub struct TextureUnitBinding {
    idx: u32,
    pub texture_2d: Texture2dBinder,
    pub texture_cube_map: TextureCubeMapBinder
}

impl TextureUnitBinding {
    unsafe fn current_at_idx(idx: u32) -> Self {
        TextureUnitBinding {
            idx: idx,
            texture_2d: Texture2dBinder,
            texture_cube_map: TextureCubeMapBinder
        }
    }

    pub fn gl_idx(&self) -> u32 {
        self.idx
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextureSampler { idx: i32 }


#[macro_export]
macro_rules! bind_texture_cube_map {
    ($gl_tex_unit:expr, $texture:expr) => {
        $gl_tex_unit.texture_cube_map.bind($texture)
    }
}

#[macro_export]
macro_rules! bind_texture_2d {
    ($gl_tex_unit:expr, $texture:expr) => {
        $gl_tex_unit.texture_2d.bind($texture)
    }
}
