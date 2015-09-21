pub trait TextureUnit {
    fn idx() -> u32;
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
}
