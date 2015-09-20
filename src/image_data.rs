use gl;

gl_enum! {
    pub gl_enum TextelType {
        UnsignedByte as UNSIGNED_BYTE_TEXTEL = gl::UNSIGNED_BYTE,
        UnsignedShort565 as UNSIGNED_SHORT_5_6_5 = gl::UNSIGNED_SHORT_5_6_5,
        UnsignedShort4444 as UNSIGNED_SHORT_4_4_4_4 =
            gl::UNSIGNED_SHORT_4_4_4_4,
        UnsignedShort5551 as UNSIGNED_SHORT_5_5_5_1 =
            gl::UNSIGNED_SHORT_5_5_5_1
    }
}

gl_enum! {
    pub gl_enum TextelFormat {
        Alpha as ALPHA = gl::ALPHA,
        RGB as RGB = gl::RGB,
        RGBA as RGBA = gl::RGBA
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ImageFormat {
    pub textel_type: TextelType,
    pub textel_format: TextelFormat
}
