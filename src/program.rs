//! Exposes the OpenGL [`Program`](struct.Program.html) object and related types.

use std::marker::PhantomData;
use gl;
use gl::types::*;
use types::GLObject;

/// An OpenGL program object.
///
/// A program is used in OpenGL to customize the render pipeline. A program is
/// composed of multiple [`Shader`](../shader/struct.Shader.html) objects, each
/// of which computes one step in the render pipeline, and feed data to the
/// next program.
///
/// A program will automatically be deleted after going out of scope.
///
/// # See also
/// [`gl.build_shader`](../context/program_context/trait.ContextProgramBuilderExt.html#tymethod.build_program):
/// Build and link a new program object
///
/// [`gl.create_program`](../context/program_context/trait.ContextProgramExt.html#method.create_program):
/// Create a new, empty program object.
///
/// [`gl.use_program`](../context/program_context/trait.ProgramContext.html#method.use_program):
/// Bind a program to use for drawing calls, returning a [`ProgramBinding`]
/// (../context/program_context/struct.ProgramBinding.html) type.
pub struct Program {
    gl_id: GLuint,
    _phantom: PhantomData<*mut ()>
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.gl_id);
        }
    }
}

impl GLObject for Program {
    type Id = GLuint;

    unsafe fn from_raw(id: Self::Id) -> Self {
        Program {
            gl_id: id,
            _phantom: PhantomData
        }
    }

    fn id(&self) -> Self::Id {
        self.gl_id
    }
}


/// An OpenGL generic vertex attribute.
///
/// This type is used for getting and setting up the vertex attributes of a
/// program, which describes how a program should treat the raw data that
/// is rendered with a draw call such as [`gl.draw_arrays_range`]
/// (context/buffer_context/trait.ContextBufferExt.html#method.draw_arrays_range).
///
/// # See also
/// [`gl.get_attrib_location`](context/program_context/trait.ContextProgramExt.html#method.get_attrib_location):
/// Get a `ProgramAttrib` from an attribute's name within a program.
#[derive(Debug, Clone, Copy)]
pub struct ProgramAttrib {
    /// The index of the program attribute.
    pub gl_index: GLuint
}

/// An OpenGL program uniform.
///
/// This type is used for getting and setting a program's uniform variables,
/// which are constants used during a single draw call, but can be changed
/// between draw calls.
///
/// # See also
/// [`gl.get_uniform_location`](context/program_context/trait.ContextProgramExt.html#method.get_uniform_location):
/// Get a `ProgramUnfirom` from a uniform's name within a program.
///
/// [`gl.set_uniform`](context/program_context/trait.ContextProgramExt.html#method.set_uniform):
/// Set or change the value of a program's uniform.
#[derive(Debug, Clone, Copy)]
pub struct ProgramUniform {
    /// The index of the the program uniform.
    pub gl_index: GLuint
}
