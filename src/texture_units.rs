use std::borrow::{Borrow, BorrowMut};
use gl;
use gl::types::*;
use context::{AContext, ContextOf};
use texture::{Texture2dBinder, TextureCubeMapBinder,
              Texture2dBinding, TextureCubeMapBinding,
              Texture2d, TextureCubeMap};
use uniform_data::{UniformDatum, UniformDatumType, UniformPrimitiveType};
use types::GLError;
use to_ref::{ToRef, ToMut};

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

    fn borrowed<'a,
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

    fn borrowed_mut<'a,
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

    pub fn swap_0<N0>(self, new_unit: N0)
        -> (T0, TextureUnitsOf<N0, T1, T2, T3, T4, T5, T6, T7>)
    {
        (
            self.0,
            TextureUnitsOf(new_unit,
                           self.1,
                           self.2,
                           self.3,
                           self.4,
                           self.5,
                           self.6,
                           self.7)
        )
    }



    pub fn swap_1<N1>(self, new_unit: N1)
        -> (T1, TextureUnitsOf<T0, N1, T2, T3, T4, T5, T6, T7>)
    {
        (
            self.1,
            TextureUnitsOf(self.0,
                           new_unit,
                           self.2,
                           self.3,
                           self.4,
                           self.5,
                           self.6,
                           self.7)
        )
    }

    pub fn swap_2<N2>(self, new_unit: N2)
        -> (T2, TextureUnitsOf<T0, T1, N2, T3, T4, T5, T6, T7>)
    {
        (
            self.2,
            TextureUnitsOf(self.0,
                           self.1,
                           new_unit,
                           self.3,
                           self.4,
                           self.5,
                           self.6,
                           self.7)
        )
    }

    pub fn swap_3<N3>(self, new_unit: N3)
        -> (T3, TextureUnitsOf<T0, T1, T2, N3, T4, T5, T6, T7>)
    {
        (
            self.3,
            TextureUnitsOf(self.0,
                           self.1,
                           self.2,
                           new_unit,
                           self.4,
                           self.5,
                           self.6,
                           self.7)
        )
    }

    pub fn swap_4<N4>(self, new_unit: N4)
        -> (T4, TextureUnitsOf<T0, T1, T2, T3, N4, T5, T6, T7>)
    {
        (
            self.4,
            TextureUnitsOf(self.0,
                           self.1,
                           self.2,
                           self.3,
                           new_unit,
                           self.5,
                           self.6,
                           self.7)
        )
    }

    pub fn swap_5<N5>(self, new_unit: N5)
        -> (T5, TextureUnitsOf<T0, T1, T2, T3, T4, N5, T6, T7>)
    {
        (
            self.5,
            TextureUnitsOf(self.0,
                           self.1,
                           self.2,
                           self.3,
                           self.4,
                           new_unit,
                           self.6,
                           self.7)
        )
    }

    pub fn swap_6<N6>(self, new_unit: N6)
        -> (T6, TextureUnitsOf<T0, T1, T2, T3, T4, T5, N6, T7>)
    {
        (
            self.6,
            TextureUnitsOf(self.0,
                           self.1,
                           self.2,
                           self.3,
                           self.4,
                           self.5,
                           new_unit,
                           self.7)
        )
    }

    pub fn swap_7<N7>(self, new_unit: N7)
        -> (T7, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, N7>)
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
                           new_unit)
        )
    }

    pub unsafe fn active_nth(&self, idx: u32) -> TextureUnitBinding {
        _active_texture(idx);
        TextureUnitBinding::current_at_idx(idx)
    }
}

impl<'a, T0, T1, T2, T3, T4, T5, T6, T7> ToRef<'a>
    for TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>
    where T0: 'a + ToRef<'a>,
          T1: 'a + ToRef<'a>,
          T2: 'a + ToRef<'a>,
          T3: 'a + ToRef<'a>,
          T4: 'a + ToRef<'a>,
          T5: 'a + ToRef<'a>,
          T6: 'a + ToRef<'a>,
          T7: 'a + ToRef<'a>
{
    type Ref = TextureUnitsOf<T0::Ref,
                              T1::Ref,
                              T2::Ref,
                              T3::Ref,
                              T4::Ref,
                              T5::Ref,
                              T6::Ref,
                              T7::Ref>;

    fn to_ref(&'a self) -> Self::Ref {
        TextureUnitsOf(
            self.0.to_ref(),
            self.1.to_ref(),
            self.2.to_ref(),
            self.3.to_ref(),
            self.4.to_ref(),
            self.5.to_ref(),
            self.6.to_ref(),
            self.7.to_ref()
        )
    }
}

impl<'a, T0, T1, T2, T3, T4, T5, T6, T7> ToMut<'a>
    for TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>
    where T0: 'a + ToMut<'a>,
          T1: 'a + ToMut<'a>,
          T2: 'a + ToMut<'a>,
          T3: 'a + ToMut<'a>,
          T4: 'a + ToMut<'a>,
          T5: 'a + ToMut<'a>,
          T6: 'a + ToMut<'a>,
          T7: 'a + ToMut<'a>
{
    type Mut = TextureUnitsOf<T0::Mut,
                              T1::Mut,
                              T2::Mut,
                              T3::Mut,
                              T4::Mut,
                              T5::Mut,
                              T6::Mut,
                              T7::Mut>;

    fn to_mut(&'a mut self) -> Self::Mut {
        TextureUnitsOf(
            self.0.to_mut(),
            self.1.to_mut(),
            self.2.to_mut(),
            self.3.to_mut(),
            self.4.to_mut(),
            self.5.to_mut(),
            self.6.to_mut(),
            self.7.to_mut()
        )
    }
}



pub trait TextureUnit0Context: AContext {
    type Unit: BorrowMut<TextureUnit0>;
    type Rest: AContext;

    fn split_tex_unit_0(self) -> (Self::Unit, Self::Rest);

    fn active_texture_0(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_0();
        (unit.borrow_mut().active(), rest)
    }
}

pub trait TextureUnit1Context: AContext {
    type Unit: BorrowMut<TextureUnit1>;
    type Rest: AContext;

    fn split_tex_unit_1(self) -> (Self::Unit, Self::Rest);

    fn active_texture_1(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_1();
        (unit.borrow_mut().active(), rest)
    }
}

pub trait TextureUnit2Context: AContext {
    type Unit: BorrowMut<TextureUnit2>;
    type Rest: AContext;

    fn split_tex_unit_2(self) -> (Self::Unit, Self::Rest);

    fn active_texture_2(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_2();
        (unit.borrow_mut().active(), rest)
    }
}

pub trait TextureUnit3Context: AContext {
    type Unit: BorrowMut<TextureUnit3>;
    type Rest: AContext;

    fn split_tex_unit_3(self) -> (Self::Unit, Self::Rest);

    fn active_texture_3(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_3();
        (unit.borrow_mut().active(), rest)
    }
}

pub trait TextureUnit4Context: AContext {
    type Unit: BorrowMut<TextureUnit4>;
    type Rest: AContext;

    fn split_tex_unit_4(self) -> (Self::Unit, Self::Rest);

    fn active_texture_4(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_4();
        (unit.borrow_mut().active(), rest)
    }
}

pub trait TextureUnit5Context: AContext {
    type Unit: BorrowMut<TextureUnit5>;
    type Rest: AContext;

    fn split_tex_unit_5(self) -> (Self::Unit, Self::Rest);

    fn active_texture_5(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_5();
        (unit.borrow_mut().active(), rest)
    }
}

pub trait TextureUnit6Context: AContext {
    type Unit: BorrowMut<TextureUnit6>;
    type Rest: AContext;

    fn split_tex_unit_6(self) -> (Self::Unit, Self::Rest);

    fn active_texture_6(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_6();
        (unit.borrow_mut().active(), rest)
    }
}

pub trait TextureUnit7Context: AContext {
    type Unit: BorrowMut<TextureUnit7>;
    type Rest: AContext;

    fn split_tex_unit_7(self) -> (Self::Unit, Self::Rest);

    fn active_texture_7(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_7();
        (unit.borrow_mut().active(), rest)
    }
}

impl<B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit0Context
    for ContextOf<B, F, P, R, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>>
    where T0: BorrowMut<TextureUnit0>
{
    type Unit = T0;
    type Rest = ContextOf<B, F, P, R, TextureUnitsOf<(),
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>;

    fn split_tex_unit_0(self) -> (Self::Unit, Self::Rest) {
        let (tex_units, gl) = self.swap_tex_units(());
        let (unit, rest_tex_units) = tex_units.swap_0(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit0Context
    for &'a mut ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>
    where T0: BorrowMut<TextureUnit0>
{
    type Unit = &'a mut TextureUnit0;
    type Rest = ContextOf<&'a mut B,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          TextureUnitsOf<(),
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_0(self) -> (Self::Unit, Self::Rest) {
        let gl = self.borrowed_mut();
        let (tex_units, gl) = gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (tex_unit, rest_tex_units) = tex_units.swap_0(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (tex_unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit0Context
    for &'a mut ContextOf<B, F, P, R, &'a mut TextureUnitsOf<T0,
                                                             T1,
                                                             T2,
                                                             T3,
                                                             T4,
                                                             T5,
                                                             T6,
                                                             T7>>
    where T0: BorrowMut<TextureUnit0>,
          B: ToMut<'a>, F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>
{
    type Unit = &'a mut TextureUnit0;
    type Rest = ContextOf<B::Mut,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          TextureUnitsOf<(),
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_0(self) -> (Self::Unit, Self::Rest) {
        let gl = self.to_mut();
        let (tex_units, gl): (&mut TextureUnitsOf<_, _, _, _, _, _, _, _>, _) =
            gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (unit, rest_tex_units) = tex_units.swap_0(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit1Context
    for ContextOf<B, F, P, R, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>>
    where T1: BorrowMut<TextureUnit1>
{
    type Unit = T1;
    type Rest = ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     (),
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>;

    fn split_tex_unit_1(self) -> (Self::Unit, Self::Rest) {
        let (tex_units, gl) = self.swap_tex_units(());
        let (unit, rest_tex_units) = tex_units.swap_1(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit1Context
    for &'a mut ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>
    where T1: BorrowMut<TextureUnit1>
{
    type Unit = &'a mut TextureUnit1;
    type Rest = ContextOf<&'a mut B,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          TextureUnitsOf<&'a mut T0,
                                         (),
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_1(self) -> (Self::Unit, Self::Rest) {
        let gl = self.borrowed_mut();
        let (tex_units, gl) = gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (tex_unit, rest_tex_units) = tex_units.swap_1(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (tex_unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit1Context
    for &'a mut ContextOf<B, F, P, R, &'a mut TextureUnitsOf<T0,
                                                             T1,
                                                             T2,
                                                             T3,
                                                             T4,
                                                             T5,
                                                             T6,
                                                             T7>>
    where T1: BorrowMut<TextureUnit1>,
          B: ToMut<'a>, F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>
{
    type Unit = &'a mut TextureUnit1;
    type Rest = ContextOf<B::Mut,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          TextureUnitsOf<&'a mut T0,
                                         (),
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_1(self) -> (Self::Unit, Self::Rest) {
        let gl = self.to_mut();
        let (tex_units, gl): (&mut TextureUnitsOf<_, _, _, _, _, _, _, _>, _) =
            gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (unit, rest_tex_units) = tex_units.swap_1(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit2Context
    for ContextOf<B, F, P, R, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>>
    where T2: BorrowMut<TextureUnit2>
{
    type Unit = T2;
    type Rest = ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     (),
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>;

    fn split_tex_unit_2(self) -> (Self::Unit, Self::Rest) {
        let (tex_units, gl) = self.swap_tex_units(());
        let (unit, rest_tex_units) = tex_units.swap_2(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit2Context
    for &'a mut ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>
    where T2: BorrowMut<TextureUnit2>
{
    type Unit = &'a mut TextureUnit2;
    type Rest = ContextOf<&'a mut B,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         (),
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_2(self) -> (Self::Unit, Self::Rest) {
        let gl = self.borrowed_mut();
        let (tex_units, gl) = gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (tex_unit, rest_tex_units) = tex_units.swap_2(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (tex_unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit2Context
    for &'a mut ContextOf<B, F, P, R, &'a mut TextureUnitsOf<T0,
                                                             T1,
                                                             T2,
                                                             T3,
                                                             T4,
                                                             T5,
                                                             T6,
                                                             T7>>
    where T2: BorrowMut<TextureUnit2>,
          B: ToMut<'a>, F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>
{
    type Unit = &'a mut TextureUnit2;
    type Rest = ContextOf<B::Mut,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         (),
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_2(self) -> (Self::Unit, Self::Rest) {
        let gl = self.to_mut();
        let (tex_units, gl): (&mut TextureUnitsOf<_, _, _, _, _, _, _, _>, _) =
            gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (unit, rest_tex_units) = tex_units.swap_2(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit3Context
    for ContextOf<B, F, P, R, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>>
    where T3: BorrowMut<TextureUnit3>
{
    type Unit = T3;
    type Rest = ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     (),
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>;

    fn split_tex_unit_3(self) -> (Self::Unit, Self::Rest) {
        let (tex_units, gl) = self.swap_tex_units(());
        let (unit, rest_tex_units) = tex_units.swap_3(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit3Context
    for &'a mut ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>
    where T3: BorrowMut<TextureUnit3>
{
    type Unit = &'a mut TextureUnit3;
    type Rest = ContextOf<&'a mut B,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         (),
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_3(self) -> (Self::Unit, Self::Rest) {
        let gl = self.borrowed_mut();
        let (tex_units, gl) = gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (tex_unit, rest_tex_units) = tex_units.swap_3(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (tex_unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit3Context
    for &'a mut ContextOf<B, F, P, R, &'a mut TextureUnitsOf<T0,
                                                             T1,
                                                             T2,
                                                             T3,
                                                             T4,
                                                             T5,
                                                             T6,
                                                             T7>>
    where T3: BorrowMut<TextureUnit3>,
          B: ToMut<'a>, F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>
{
    type Unit = &'a mut TextureUnit3;
    type Rest = ContextOf<B::Mut,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         (),
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_3(self) -> (Self::Unit, Self::Rest) {
        let gl = self.to_mut();
        let (tex_units, gl): (&mut TextureUnitsOf<_, _, _, _, _, _, _, _>, _) =
            gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (unit, rest_tex_units) = tex_units.swap_3(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit4Context
    for ContextOf<B, F, P, R, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>>
    where T4: BorrowMut<TextureUnit4>
{
    type Unit = T4;
    type Rest = ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     (),
                                                     T5,
                                                     T6,
                                                     T7>>;

    fn split_tex_unit_4(self) -> (Self::Unit, Self::Rest) {
        let (tex_units, gl) = self.swap_tex_units(());
        let (unit, rest_tex_units) = tex_units.swap_4(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit4Context
    for &'a mut ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>
    where T4: BorrowMut<TextureUnit4>
{
    type Unit = &'a mut TextureUnit4;
    type Rest = ContextOf<&'a mut B,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         (),
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_4(self) -> (Self::Unit, Self::Rest) {
        let gl = self.borrowed_mut();
        let (tex_units, gl) = gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (tex_unit, rest_tex_units) = tex_units.swap_4(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (tex_unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit4Context
    for &'a mut ContextOf<B, F, P, R, &'a mut TextureUnitsOf<T0,
                                                             T1,
                                                             T2,
                                                             T3,
                                                             T4,
                                                             T5,
                                                             T6,
                                                             T7>>
    where T4: BorrowMut<TextureUnit4>,
          B: ToMut<'a>, F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>
{
    type Unit = &'a mut TextureUnit4;
    type Rest = ContextOf<B::Mut,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         (),
                                         &'a mut T5,
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_4(self) -> (Self::Unit, Self::Rest) {
        let gl = self.to_mut();
        let (tex_units, gl): (&mut TextureUnitsOf<_, _, _, _, _, _, _, _>, _) =
            gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (unit, rest_tex_units) = tex_units.swap_4(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit5Context
    for ContextOf<B, F, P, R, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>>
    where T5: BorrowMut<TextureUnit5>
{
    type Unit = T5;
    type Rest = ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     (),
                                                     T6,
                                                     T7>>;

    fn split_tex_unit_5(self) -> (Self::Unit, Self::Rest) {
        let (tex_units, gl) = self.swap_tex_units(());
        let (unit, rest_tex_units) = tex_units.swap_5(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit5Context
    for &'a mut ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>
    where T5: BorrowMut<TextureUnit5>
{
    type Unit = &'a mut TextureUnit5;
    type Rest = ContextOf<&'a mut B,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         (),
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_5(self) -> (Self::Unit, Self::Rest) {
        let gl = self.borrowed_mut();
        let (tex_units, gl) = gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (tex_unit, rest_tex_units) = tex_units.swap_5(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (tex_unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit5Context
    for &'a mut ContextOf<B, F, P, R, &'a mut TextureUnitsOf<T0,
                                                             T1,
                                                             T2,
                                                             T3,
                                                             T4,
                                                             T5,
                                                             T6,
                                                             T7>>
    where T5: BorrowMut<TextureUnit5>,
          B: ToMut<'a>, F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>
{
    type Unit = &'a mut TextureUnit5;
    type Rest = ContextOf<B::Mut,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         (),
                                         &'a mut T6,
                                         &'a mut T7>>;

    fn split_tex_unit_5(self) -> (Self::Unit, Self::Rest) {
        let gl = self.to_mut();
        let (tex_units, gl): (&mut TextureUnitsOf<_, _, _, _, _, _, _, _>, _) =
            gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (unit, rest_tex_units) = tex_units.swap_5(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit6Context
    for ContextOf<B, F, P, R, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>>
    where T6: BorrowMut<TextureUnit6>
{
    type Unit = T6;
    type Rest = ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     (),
                                                     T7>>;

    fn split_tex_unit_6(self) -> (Self::Unit, Self::Rest) {
        let (tex_units, gl) = self.swap_tex_units(());
        let (unit, rest_tex_units) = tex_units.swap_6(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit6Context
    for &'a mut ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>
    where T6: BorrowMut<TextureUnit6>
{
    type Unit = &'a mut TextureUnit6;
    type Rest = ContextOf<&'a mut B,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         (),
                                         &'a mut T7>>;

    fn split_tex_unit_6(self) -> (Self::Unit, Self::Rest) {
        let gl = self.borrowed_mut();
        let (tex_units, gl) = gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (tex_unit, rest_tex_units) = tex_units.swap_6(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (tex_unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit6Context
    for &'a mut ContextOf<B, F, P, R, &'a mut TextureUnitsOf<T0,
                                                             T1,
                                                             T2,
                                                             T3,
                                                             T4,
                                                             T5,
                                                             T6,
                                                             T7>>
    where T6: BorrowMut<TextureUnit6>,
          B: ToMut<'a>, F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>
{
    type Unit = &'a mut TextureUnit6;
    type Rest = ContextOf<B::Mut,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         (),
                                         &'a mut T7>>;

    fn split_tex_unit_6(self) -> (Self::Unit, Self::Rest) {
        let gl = self.to_mut();
        let (tex_units, gl): (&mut TextureUnitsOf<_, _, _, _, _, _, _, _>, _) =
            gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (unit, rest_tex_units) = tex_units.swap_6(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit7Context
    for ContextOf<B, F, P, R, TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>>
    where T7: BorrowMut<TextureUnit7>
{
    type Unit = T7;
    type Rest = ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     ()>>;

    fn split_tex_unit_7(self) -> (Self::Unit, Self::Rest) {
        let (tex_units, gl) = self.swap_tex_units(());
        let (unit, rest_tex_units) = tex_units.swap_7(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit7Context
    for &'a mut ContextOf<B, F, P, R, TextureUnitsOf<T0,
                                                     T1,
                                                     T2,
                                                     T3,
                                                     T4,
                                                     T5,
                                                     T6,
                                                     T7>>
    where T7: BorrowMut<TextureUnit7>
{
    type Unit = &'a mut TextureUnit7;
    type Rest = ContextOf<&'a mut B,
                          &'a mut F,
                          &'a mut P,
                          &'a mut R,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         ()>>;

    fn split_tex_unit_7(self) -> (Self::Unit, Self::Rest) {
        let gl = self.borrowed_mut();
        let (tex_units, gl) = gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (tex_unit, rest_tex_units) = tex_units.swap_7(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (tex_unit, gl)
    }
}

impl<'a, B, F, P, R, T0, T1, T2, T3, T4, T5, T6, T7> TextureUnit7Context
    for &'a mut ContextOf<B, F, P, R, &'a mut TextureUnitsOf<T0,
                                                             T1,
                                                             T2,
                                                             T3,
                                                             T4,
                                                             T5,
                                                             T6,
                                                             T7>>
    where T7: BorrowMut<TextureUnit7>,
          B: ToMut<'a>, F: ToMut<'a>, P: ToMut<'a>, R: ToMut<'a>
{
    type Unit = &'a mut TextureUnit7;
    type Rest = ContextOf<B::Mut,
                          F::Mut,
                          P::Mut,
                          R::Mut,
                          TextureUnitsOf<&'a mut T0,
                                         &'a mut T1,
                                         &'a mut T2,
                                         &'a mut T3,
                                         &'a mut T4,
                                         &'a mut T5,
                                         &'a mut T6,
                                         ()>>;

    fn split_tex_unit_7(self) -> (Self::Unit, Self::Rest) {
        let gl = self.to_mut();
        let (tex_units, gl): (&mut TextureUnitsOf<_, _, _, _, _, _, _, _>, _) =
            gl.swap_tex_units(());
        let tex_units = tex_units.borrowed_mut();
        let (unit, rest_tex_units) = tex_units.swap_7(());
        let ((), gl) = gl.swap_tex_units(rest_tex_units);

        (unit, gl)
    }
}



// TODO: Make `idx` a type-level integer parameter
pub struct TextureUnitBindingOf<T2, TC> {
    idx: u32,
    pub texture_2d: T2,
    pub texture_cube_map: TC
}

pub type TextureUnitBinding = TextureUnitBindingOf<Texture2dBinder,
                                                   TextureCubeMapBinder>;

impl<T2, TC> TextureUnitBindingOf<T2, TC> {
    unsafe fn current_at_idx(idx: u32) -> TextureUnitBinding {
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

    fn split_texture_2d(self) -> (T2, TextureUnitBindingOf<(), TC>) {
        (
            self.texture_2d,
            TextureUnitBindingOf {
                idx: self.idx,
                texture_2d: (),
                texture_cube_map: self.texture_cube_map
            }
        )
    }

    fn split_texture_cube_map(self) -> (TC, TextureUnitBindingOf<T2, ()>) {
        (
            self.texture_cube_map,
            TextureUnitBindingOf {
                idx: self.idx,
                texture_2d: self.texture_2d,
                texture_cube_map: ()
            }
        )
    }

    fn borrowed<'a, B2 = T2, BC = TC>(&'a self)
        -> TextureUnitBindingOf<&'a B2, &'a BC>
        where T2: Borrow<B2>,
              TC: Borrow<BC>
    {
        TextureUnitBindingOf {
            idx: self.idx,
            texture_2d: self.texture_2d.borrow(),
            texture_cube_map: self.texture_cube_map.borrow()
        }
    }

    fn borrowed_mut<'a, B2 = T2, BC = TC>(&'a mut self)
        -> TextureUnitBindingOf<&'a mut B2, &'a mut BC>
        where T2: BorrowMut<B2>,
              TC: BorrowMut<BC>
    {
        TextureUnitBindingOf {
            idx: self.idx,
            texture_2d: self.texture_2d.borrow_mut(),
            texture_cube_map: self.texture_cube_map.borrow_mut()
        }
    }
}



pub unsafe trait ATextureUnitBinding {

}

unsafe impl<T2, TC> ATextureUnitBinding for TextureUnitBindingOf<T2, TC> {

}

unsafe impl<'a, T2, TC> ATextureUnitBinding
    for &'a mut TextureUnitBindingOf<T2, TC>
{

}

pub trait TextureUnitBinding2d: ATextureUnitBinding {
    type Binder: BorrowMut<Texture2dBinder>;
    type Rest: ATextureUnitBinding;

    fn split_texture_2d(self) -> (Self::Binder, Self::Rest);

    fn bind_texture_2d<'a>(self, tex: &'a mut Texture2d)
        -> (Texture2dBinding<'a>, Self::Rest)
        where Self: Sized
    {
        let (mut binder, rest) = self.split_texture_2d();
        (binder.borrow_mut().bind(tex), rest)
    }
}

pub trait TextureUnitBindingCubeMap: ATextureUnitBinding {
    type Binder: BorrowMut<TextureCubeMapBinder>;
    type Rest: ATextureUnitBinding;

    fn split_texture_cube_map(self) -> (Self::Binder, Self::Rest);

    fn bind_texture_cube_map<'a>(self, tex: &'a mut TextureCubeMap)
        -> (TextureCubeMapBinding<'a>, Self::Rest)
        where Self: Sized
    {
        let (mut binder, rest) = self.split_texture_cube_map();
        (binder.borrow_mut().bind(tex), rest)
    }
}

impl<T2, TC> TextureUnitBinding2d for TextureUnitBindingOf<T2, TC>
    where T2: BorrowMut<Texture2dBinder>
{
    type Binder = T2;
    type Rest = TextureUnitBindingOf<(), TC>;

    fn split_texture_2d(self) -> (Self::Binder, Self::Rest) {
        self.split_texture_2d()
    }
}

impl<'a, T2, TC> TextureUnitBinding2d
    for &'a mut TextureUnitBindingOf<T2, TC>
    where T2: BorrowMut<Texture2dBinder>
{
    type Binder = &'a mut Texture2dBinder;
    type Rest = TextureUnitBindingOf<(), &'a mut TC>;

    fn split_texture_2d(self) -> (Self::Binder, Self::Rest) {
        let gl_tex_unit = self.borrowed_mut();
        gl_tex_unit.split_texture_2d()
    }
}

impl<T2, TC> TextureUnitBindingCubeMap for TextureUnitBindingOf<T2, TC>
    where TC: BorrowMut<TextureCubeMapBinder>
{
    type Binder = TC;
    type Rest = TextureUnitBindingOf<T2, ()>;

    fn split_texture_cube_map(self) -> (Self::Binder, Self::Rest) {
        self.split_texture_cube_map()
    }
}

impl<'a, T2, TC> TextureUnitBindingCubeMap
    for &'a mut TextureUnitBindingOf<T2, TC>
    where TC: BorrowMut<TextureCubeMapBinder>
{
    type Binder = &'a mut TextureCubeMapBinder;
    type Rest = TextureUnitBindingOf<&'a mut T2, ()>;

    fn split_texture_cube_map(self) -> (Self::Binder, Self::Rest) {
        let gl_tex_unit = self.borrowed_mut();
        gl_tex_unit.split_texture_cube_map()
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
