use std::borrow::{Borrow, BorrowMut};
use gl;
use gl::types::*;
use texture::{Texture2dBinder, TextureCubeMapBinder};
use uniform_data::{UniformDatum, UniformDatumType, UniformPrimitiveType};
use types::GLError;

unsafe fn _active_texture(idx: u32) {
    gl::ActiveTexture(gl::TEXTURE0 + (idx as GLenum));
    dbg_gl_error! {
        GLError::InvalidEnum => "`texture` is out of bounds (expected to be GL_TEXTUREi, 0 <= i < GL_MAX_TEXTURE_IMAGE_UNITS)",
        _ => "Unknown error"
    }
}

pub trait TextureUnit {
    fn idx(&self) -> u32;

    fn active(&mut self) -> TextureUnitBinding {
        let idx = self.idx();
        unsafe {
            _active_texture(idx);
            TextureUnitBinding::current_at_idx(idx)
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

impl TextureUnit for TextureUnit0 { fn idx(&self) -> u32 { 0 } }
impl TextureUnit for TextureUnit1 { fn idx(&self) -> u32 { 1 } }
impl TextureUnit for TextureUnit2 { fn idx(&self) -> u32 { 2 } }
impl TextureUnit for TextureUnit3 { fn idx(&self) -> u32 { 3 } }
impl TextureUnit for TextureUnit4 { fn idx(&self) -> u32 { 4 } }
impl TextureUnit for TextureUnit5 { fn idx(&self) -> u32 { 5 } }
impl TextureUnit for TextureUnit6 { fn idx(&self) -> u32 { 6 } }
impl TextureUnit for TextureUnit7 { fn idx(&self) -> u32 { 7 } }

// NOTE: Ensure the number of each texture unit matches its index in the tuple
// TODO: Use macros + integer-level types to refactor this
pub struct TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>(pub T0,
                                                          pub T1,
                                                          pub T2,
                                                          pub T3,
                                                          pub T4,
                                                          pub T5,
                                                          pub T6,
                                                          pub T7);

pub type TextureUnits = TextureUnitsOf<TextureUnit0,
                                       TextureUnit1,
                                       TextureUnit2,
                                       TextureUnit3,
                                       TextureUnit4,
                                       TextureUnit5,
                                       TextureUnit6,
                                       TextureUnit7>;

pub type TextureUnitsRef<'a> = TextureUnitsOf<&'a TextureUnit0,
                                              &'a TextureUnit1,
                                              &'a TextureUnit2,
                                              &'a TextureUnit3,
                                              &'a TextureUnit4,
                                              &'a TextureUnit5,
                                              &'a TextureUnit6,
                                              &'a TextureUnit7>;

pub type TextureUnitsMut<'a> = TextureUnitsOf<&'a mut TextureUnit0,
                                              &'a mut TextureUnit1,
                                              &'a mut TextureUnit2,
                                              &'a mut TextureUnit3,
                                              &'a mut TextureUnit4,
                                              &'a mut TextureUnit5,
                                              &'a mut TextureUnit6,
                                              &'a mut TextureUnit7>;

impl<T0, T1, T2, T3, T4, T5, T6, T7> TextureUnitsOf<T0,
                                                    T1,
                                                    T2,
                                                    T3,
                                                    T4,
                                                    T5,
                                                    T6,
                                                    T7>
{
    pub unsafe fn current() -> TextureUnits {
        TextureUnitsOf(TextureUnit0,
                       TextureUnit1,
                       TextureUnit2,
                       TextureUnit3,
                       TextureUnit4,
                       TextureUnit5,
                       TextureUnit6,
                       TextureUnit7)
    }

    pub fn borrowed<'a,
                    B0 = T0,
                    B1 = T1,
                    B2 = T2,
                    B3 = T3,
                    B4 = T4,
                    B5 = T5,
                    B6 = T6,
                    B7 = T7>
                   (&'a self)
        -> TextureUnitsOf<&'a B0,
                          &'a B1,
                          &'a B2,
                          &'a B3,
                          &'a B4,
                          &'a B5,
                          &'a B6,
                          &'a B7>
        where T0: Borrow<B0>,
              T1: Borrow<B1>,
              T2: Borrow<B2>,
              T3: Borrow<B3>,
              T4: Borrow<B4>,
              T5: Borrow<B5>,
              T6: Borrow<B6>,
              T7: Borrow<B7>
    {
        TextureUnitsOf(self.0.borrow(),
                       self.1.borrow(),
                       self.2.borrow(),
                       self.3.borrow(),
                       self.4.borrow(),
                       self.5.borrow(),
                       self.6.borrow(),
                       self.7.borrow())
    }

    pub fn borrowed_mut<'a,
                        B0 = T0,
                        B1 = T1,
                        B2 = T2,
                        B3 = T3,
                        B4 = T4,
                        B5 = T5,
                        B6 = T6,
                        B7 = T7>
                       (&'a mut self)
        -> TextureUnitsOf<&'a mut B0,
                          &'a mut B1,
                          &'a mut B2,
                          &'a mut B3,
                          &'a mut B4,
                          &'a mut B5,
                          &'a mut B6,
                          &'a mut B7>
        where T0: BorrowMut<B0>,
              T1: BorrowMut<B1>,
              T2: BorrowMut<B2>,
              T3: BorrowMut<B3>,
              T4: BorrowMut<B4>,
              T5: BorrowMut<B5>,
              T6: BorrowMut<B6>,
              T7: BorrowMut<B7>
    {
        TextureUnitsOf(self.0.borrow_mut(),
                       self.1.borrow_mut(),
                       self.2.borrow_mut(),
                       self.3.borrow_mut(),
                       self.4.borrow_mut(),
                       self.5.borrow_mut(),
                       self.6.borrow_mut(),
                       self.7.borrow_mut())
    }

    pub fn split_0(self)
        -> (T0, TextureUnitsOf<(), T1, T2, T3, T4, T5, T6, T7>)
    {
        (
            self.0,
            TextureUnitsOf((),
                           self.1,
                           self.2,
                           self.3,
                           self.4,
                           self.5,
                           self.6,
                           self.7)
        )
    }



    pub fn split_1(self)
        -> (T1, TextureUnitsOf<T0, (), T2, T3, T4, T5, T6, T7>)
    {
        (
            self.1,
            TextureUnitsOf(self.0,
                           (),
                           self.2,
                           self.3,
                           self.4,
                           self.5,
                           self.6,
                           self.7)
        )
    }

    pub fn split_2(self)
        -> (T2, TextureUnitsOf<T0, T1, (), T3, T4, T5, T6, T7>)
    {
        (
            self.2,
            TextureUnitsOf(self.0,
                           self.1,
                           (),
                           self.3,
                           self.4,
                           self.5,
                           self.6,
                           self.7)
        )
    }

    pub fn split_3(self)
        -> (T3, TextureUnitsOf<T0, T1, T2, (), T4, T5, T6, T7>)
    {
        (
            self.3,
            TextureUnitsOf(self.0,
                           self.1,
                           self.2,
                           (),
                           self.4,
                           self.5,
                           self.6,
                           self.7)
        )
    }

    pub fn split_4(self)
        -> (T4, TextureUnitsOf<T0, T1, T2, T3, (), T5, T6, T7>)
    {
        (
            self.4,
            TextureUnitsOf(self.0,
                           self.1,
                           self.2,
                           self.3,
                           (),
                           self.5,
                           self.6,
                           self.7)
        )
    }

    pub fn split_5(self)
        -> (T5, TextureUnitsOf<T0, T1, T2, T3, T4, (), T6, T7>)
    {
        (
            self.5,
            TextureUnitsOf(self.0,
                           self.1,
                           self.2,
                           self.3,
                           self.4,
                           (),
                           self.6,
                           self.7)
        )
    }

    pub fn split_6(self)
        -> (T6, TextureUnitsOf<T0, T1, T2, T3, T4, T5, (), T7>)
    {
        (
            self.6,
            TextureUnitsOf(self.0,
                           self.1,
                           self.2,
                           self.3,
                           self.4,
                           self.5,
                           (),
                           self.7)
        )
    }

    pub fn split_7(self)
        -> (T7, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, ()>)
    {
        (
            self.7,
            TextureUnitsOf(self.0,
                           self.1,
                           self.2,
                           self.3,
                           self.4,
                           self.5,
                           self.6,
                           ())
        )
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

    pub fn sampler(&self) -> TextureSampler {
        TextureSampler { idx: self.idx as i32 }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextureSampler { idx: i32 }

unsafe impl UniformDatum for TextureSampler {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(UniformPrimitiveType::Int)
    }
}


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
