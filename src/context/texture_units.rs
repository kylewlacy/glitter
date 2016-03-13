//! Contains all of the OpenGL state types related texture units.

use std::borrow::BorrowMut;
use std::marker::PhantomData;
use gl;
use gl::types::*;
use context::{AContext, ContextOf,
              Texture2dBinder, TextureCubeMapBinder,
              Texture2dBinding, TextureCubeMapBinding};
use texture::{Texture2d, TextureCubeMap};
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

/// A trait that represents a 'texture unit', which is a piece of OpenGL state
/// that contains its own independent texture bindings.
pub trait TextureUnit {
    /// Get the index of the texture unit.
    fn idx(&self) -> u32;

    /// Make the current texture unit active, returning a binding.
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
/// The 0th texture unit.
pub struct TextureUnit0 {
    _phantom: PhantomData<*mut ()>
}

/// The 1st texture unit.
pub struct TextureUnit1 {
    _phantom: PhantomData<*mut ()>
}

/// The 2nd texture unit.
pub struct TextureUnit2 {
    _phantom: PhantomData<*mut ()>
}

/// The 3rd texture unit.
pub struct TextureUnit3 {
    _phantom: PhantomData<*mut ()>
}

/// The 4th texture unit.
pub struct TextureUnit4 {
    _phantom: PhantomData<*mut ()>
}

/// The 5th texture unit.
pub struct TextureUnit5 {
    _phantom: PhantomData<*mut ()>
}

/// The 6th texture unit.
pub struct TextureUnit6 {
    _phantom: PhantomData<*mut ()>
}

/// The 7th texture unit.
pub struct TextureUnit7 {
    _phantom: PhantomData<*mut ()>
}


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
/// This type holds all of the OpenGL textrure units. Each type parameter
/// is the current type of a texture unit. See the [`ContextOf`]
/// (../struct.ContextOf.html) docs for more details.
pub struct TextureUnitsOf<T0, T1, T2, T3, T4, T5, T6, T7>(pub T0,
                                                          pub T1,
                                                          pub T2,
                                                          pub T3,
                                                          pub T4,
                                                          pub T5,
                                                          pub T6,
                                                          pub T7);

/// A part of the OpenGL context that has all free texture units.
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
    /// Get the current texture units.
    ///
    /// # Safety
    /// The same rules apply to this method as the
    /// [`ContextOf::current_context()` method]
    /// (../struct.ContextOf.html#method.current_context).
    pub unsafe fn current() -> TextureUnits {
        TextureUnitsOf(TextureUnit0 { _phantom: PhantomData },
                       TextureUnit1 { _phantom: PhantomData },
                       TextureUnit2 { _phantom: PhantomData },
                       TextureUnit3 { _phantom: PhantomData },
                       TextureUnit4 { _phantom: PhantomData },
                       TextureUnit5 { _phantom: PhantomData },
                       TextureUnit6 { _phantom: PhantomData },
                       TextureUnit7 { _phantom: PhantomData })
    }

    fn borrowed_mut<'a, B0, B1, B2, B3, B4, B5, B6, B7>(&'a mut self)
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

    /// Replace the 0th texture unit context with a new value, returning the
    /// old value and a new set of texture units
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

    /// Replace the 1st texture unit context with a new value, returning the
    /// old value and a new set of texture units
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

    /// Replace the 2nd texture unit context with a new value, returning the
    /// old value and a new set of texture units
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

    /// Replace the 3rd texture unit context with a new value, returning the
    /// old value and a new set of texture units
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

    /// Replace the 4th texture unit context with a new value, returning the
    /// old value and a new set of texture units
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

    /// Replace the 5th texture unit context with a new value, returning the
    /// old value and a new set of texture units
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

    /// Replace the 6th texture unit context with a new value, returning the
    /// old value and a new set of texture units
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

    /// Replace the 7th texture unit context with a new value, returning the
    /// old value and a new set of texture units
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

    /// Make the `idx`th texture unit the active one, returning a new binding.
    ///
    /// # Safety
    /// For convenience, this function takes `self` by shared reference, not
    /// motable reference. Thus, this function can be to create multiple
    /// live bindings. Special care must be taken to ensure that two bindings
    /// do not conflict; since there can only ever be one active texture unit
    /// in OpenGL, using this function may result in unexpected or undefined
    /// behavior, and it should only be used as a fallback when glitter's
    /// safe texture unit interface is not sufficient.
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



/// An OpenGL context with the 0th texture unit free.
pub trait TextureUnit0Context: AContext {
    /// The type of unit this context contains.
    type Unit: BorrowMut<TextureUnit0>;

    /// The OpenGL context that will be returned after making the
    /// texture unit active.
    type Rest: AContext;

    /// Split the 0th texture unit from the context, returning the unit
    /// and the remaining context.
    fn split_tex_unit_0(self) -> (Self::Unit, Self::Rest);

    /// Make the 0th texture unit active, returning a binding and the
    /// remaining context
    fn active_texture_0(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_0();
        (unit.borrow_mut().active(), rest)
    }
}

/// An OpenGL context with the 1st texture unit free.
pub trait TextureUnit1Context: AContext {
    /// The type of unit this context contains.
    type Unit: BorrowMut<TextureUnit1>;

    /// The OpenGL context that will be returned after making the
    /// texture unit active.
    type Rest: AContext;

    /// Split the 1st texture unit from the context, returning the unit
    /// and the remaining context.
    fn split_tex_unit_1(self) -> (Self::Unit, Self::Rest);

    /// Make the 1st texture unit active, returning a binding and the
    /// remaining context
    fn active_texture_1(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_1();
        (unit.borrow_mut().active(), rest)
    }
}

/// An OpenGL context with the 2nd texture unit free.
pub trait TextureUnit2Context: AContext {
    /// The type of unit this context contains.
    type Unit: BorrowMut<TextureUnit2>;

    /// The OpenGL context that will be returned after making the
    /// texture unit active.
    type Rest: AContext;

    /// Split the 2nd texture unit from the context, returning the unit
    /// and the remaining context.
    fn split_tex_unit_2(self) -> (Self::Unit, Self::Rest);

    /// Make the 2nd texture unit active, returning a binding and the
    /// remaining context
    fn active_texture_2(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_2();
        (unit.borrow_mut().active(), rest)
    }
}

/// An OpenGL context with the 3rd texture unit free.
pub trait TextureUnit3Context: AContext {
    /// The type of unit this context contains.
    type Unit: BorrowMut<TextureUnit3>;

    /// The OpenGL context that will be returned after making the
    /// texture unit active.
    type Rest: AContext;

    /// Split the 3rd texture unit from the context, returning the unit
    /// and the remaining context.
    fn split_tex_unit_3(self) -> (Self::Unit, Self::Rest);

    /// Make the 3rd texture unit active, returning a binding and the
    /// remaining context
    fn active_texture_3(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_3();
        (unit.borrow_mut().active(), rest)
    }
}

/// An OpenGL context with the 4th texture unit free.
pub trait TextureUnit4Context: AContext {
    /// The type of unit this context contains.
    type Unit: BorrowMut<TextureUnit4>;

    /// The OpenGL context that will be returned after making the
    /// texture unit active.
    type Rest: AContext;

    /// Split the 4th texture unit from the context, returning the unit
    /// and the remaining context.
    fn split_tex_unit_4(self) -> (Self::Unit, Self::Rest);

    /// Make the 4th texture unit active, returning a binding and the
    /// remaining context
    fn active_texture_4(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_4();
        (unit.borrow_mut().active(), rest)
    }
}

/// An OpenGL context with the 5th texture unit free.
pub trait TextureUnit5Context: AContext {
    /// The type of unit this context contains.
    type Unit: BorrowMut<TextureUnit5>;

    /// The OpenGL context that will be returned after making the
    /// texture unit active.
    type Rest: AContext;

    /// Split the 5th texture unit from the context, returning the unit
    /// and the remaining context.
    fn split_tex_unit_5(self) -> (Self::Unit, Self::Rest);

    /// Make the 5th texture unit active, returning a binding and the
    /// remaining context
    fn active_texture_5(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_5();
        (unit.borrow_mut().active(), rest)
    }
}

/// An OpenGL context with the 6th texture unit free.
pub trait TextureUnit6Context: AContext {
    /// The type of unit this context contains.
    type Unit: BorrowMut<TextureUnit6>;

    /// The OpenGL context that will be returned after making the
    /// texture unit active.
    type Rest: AContext;

    /// Split the 6th texture unit from the context, returning the unit
    /// and the remaining context.
    fn split_tex_unit_6(self) -> (Self::Unit, Self::Rest);

    /// Make the 6th texture unit active, returning a binding and the
    /// remaining context
    fn active_texture_6(self) -> (TextureUnitBinding, Self::Rest)
        where Self: Sized
    {
        let (mut unit, rest) = self.split_tex_unit_6();
        (unit.borrow_mut().active(), rest)
    }
}

/// An OpenGL context with the 7th texture unit free.
pub trait TextureUnit7Context: AContext {
    /// The type of unit this context contains.
    type Unit: BorrowMut<TextureUnit7>;

    /// The OpenGL context that will be returned after making the
    /// texture unit active.
    type Rest: AContext;

    /// Split the 7th texture unit from the context, returning the unit
    /// and the remaining context.
    fn split_tex_unit_7(self) -> (Self::Unit, Self::Rest);

    /// Make the 7th texture unit active, returning a binding and the
    /// remaining context
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
/// A texture unit that has been made active, and can have textures
/// bound to it.
pub struct TextureUnitBindingOf<T2, TC> {
    idx: u32,
    texture_2d: T2,
    texture_cube_map: TC,
    _phantom: PhantomData<*mut ()>
}

/// A fresh texture unit binding, that has all free texture bindings.
pub type TextureUnitBinding = TextureUnitBindingOf<Texture2dBinder,
                                                   TextureCubeMapBinder>;

impl<T2, TC> TextureUnitBindingOf<T2, TC> {
    unsafe fn current_at_idx(idx: u32) -> TextureUnitBinding {
        TextureUnitBinding {
            idx: idx,
            texture_2d: Texture2dBinder::current(),
            texture_cube_map: TextureCubeMapBinder::current(),
            _phantom: PhantomData
        }
    }

    /// Get the index of the texture unit.
    pub fn gl_idx(&self) -> u32 {
        self.idx
    }

    /// Get the current texture unit as a [`TextureSampler`]
    /// (struct.TextureSampler.html), which can be used to set
    /// a uniform variable.
    pub fn sampler(&self) -> TextureSampler {
        TextureSampler { idx: self.idx as i32 }
    }

    fn split_texture_2d(self) -> (T2, TextureUnitBindingOf<(), TC>) {
        (
            self.texture_2d,
            TextureUnitBindingOf {
                idx: self.idx,
                texture_2d: (),
                texture_cube_map: self.texture_cube_map,
                _phantom: PhantomData
            }
        )
    }

    fn split_texture_cube_map(self) -> (TC, TextureUnitBindingOf<T2, ()>) {
        (
            self.texture_cube_map,
            TextureUnitBindingOf {
                idx: self.idx,
                texture_2d: self.texture_2d,
                texture_cube_map: (),
                _phantom: PhantomData
            }
        )
    }

    fn borrowed_mut<'a, B2, BC>(&'a mut self)
        -> TextureUnitBindingOf<&'a mut B2, &'a mut BC>
        where T2: BorrowMut<B2>,
              TC: BorrowMut<BC>
    {
        TextureUnitBindingOf {
            idx: self.idx,
            texture_2d: self.texture_2d.borrow_mut(),
            texture_cube_map: self.texture_cube_map.borrow_mut(),
            _phantom: PhantomData
        }
    }
}



/// A marker trait for types that represent an active texture unit binding.
///
/// # Safety
/// This type should only be implemented for types that can guarantee
/// that an active texture unit will be bound for the lifetime of an
/// instance of this type.
pub unsafe trait ATextureUnitBinding {

}

unsafe impl<T2, TC> ATextureUnitBinding for TextureUnitBindingOf<T2, TC> {

}

unsafe impl<'a, T2, TC> ATextureUnitBinding
    for &'a mut TextureUnitBindingOf<T2, TC>
{

}

/// A texture unit binding that has a free `GL_TEXTURE_2D` binding.
pub trait TextureUnitBinding2d: ATextureUnitBinding {
    /// The type of binder this texture unit contains.
    type Binder: BorrowMut<Texture2dBinder>;

    /// The texture unit that will be returned after binding the texture.
    type Rest: ATextureUnitBinding;

    /// Split the texture unit into a binder and the remaining texture unit.
    fn split_texture_2d(self) -> (Self::Binder, Self::Rest);

    /// Bind a 2D texture to this texture unit, returning a binding
    /// and the remaining texture unit.
    fn bind_texture_2d<'a>(self, tex: &'a mut Texture2d)
        -> (Texture2dBinding<'a>, Self::Rest)
        where Self: Sized
    {
        let (mut binder, rest) = self.split_texture_2d();
        (binder.borrow_mut().bind(tex), rest)
    }
}

/// A texture unit binding that has a free `GL_TEXTURE_CUBE_MAP` binding.
pub trait TextureUnitBindingCubeMap: ATextureUnitBinding {
    /// The type of binder this texture unit contains.
    type Binder: BorrowMut<TextureCubeMapBinder>;

    /// The texture unit that will be returned after binding the texture.
    type Rest: ATextureUnitBinding;

    /// Split the texture unit into a binder and the remaining texture unit.
    fn split_texture_cube_map(self) -> (Self::Binder, Self::Rest);

    /// Bind a cubemap texture to this texture unit, returning a binding
    /// and the remaining texture unit.
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

/// A newtype wrapper representing a texture sampler, which can be
/// used to set a uniform variable, using [`gl.set_uniform`]
/// (../program_context/trait.ContextProgramExt.html#method.set_uniform).
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct TextureSampler { idx: i32 }

unsafe impl UniformDatum for TextureSampler {
    fn uniform_datum_type() -> UniformDatumType {
        UniformDatumType::Vec1(UniformPrimitiveType::Int)
    }
}
