use gl::types::*;

pub struct Framebuffer {
    gl_id: GLuint
}

impl Framebuffer {
    pub fn gl_id(&self) -> GLuint {
        self.gl_id
    }
}
