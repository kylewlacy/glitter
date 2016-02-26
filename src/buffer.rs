use std::marker::PhantomData;
use gl;
use gl::types::*;
use types::GLObject;

pub struct Buffer {
    gl_id: GLuint,
    _phantom: PhantomData<*mut ()>
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.gl_id as *const GLuint);
        }
    }
}

impl GLObject for Buffer {
    type Id = GLuint;

    unsafe fn from_raw(id: Self::Id) -> Self {
        Buffer {
            gl_id: id,
            _phantom: PhantomData
        }
    }

    fn id(&self) -> Self::Id {
        self.gl_id
    }
}



gl_enum! {
    pub gl_enum BufferDataUsage {
        pub const StreamDraw as STREAM_DRAW = gl::STREAM_DRAW,
        pub const StaticDraw as STATIC_DRAW = gl::STATIC_DRAW,
        pub const DynamicDraw as DYNAMIC_DRAW = gl::DYNAMIC_DRAW
    }
}

gl_enum! {
    pub gl_enum BufferBindingTarget {
        pub const ArrayBuffer as ARRAY_BUFFER =
            gl::ARRAY_BUFFER,
        pub const ElementArrayBuffer as ELEMENT_ARRAY_BUFFER =
            gl::ELEMENT_ARRAY_BUFFER
    }
}
