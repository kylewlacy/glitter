use gl::types::*;

pub trait ImageTargetType {
    fn gl_enum(&self) -> GLenum;
}
