#[macro_use] extern crate bitflags;
extern crate gl as gl_lib;

#[macro_use] mod macros;
mod context;
mod buffer;
mod shader;
mod program;
mod framebuffer;
mod renderbuffer;
mod texture;
mod texture_units;
mod image_data;
mod vertex_data;
mod vertex_buffer;
mod index_data;
mod uniform_data;
mod types;

pub use gl_lib as gl;

pub use context::Context;
pub use buffer::{Buffer, BufferBinding, BufferDataUsage,
                 STREAM_DRAW, STATIC_DRAW, DYNAMIC_DRAW,
                 ArrayBufferBinder, ElementArrayBufferBinder,
                 ArrayBufferBinding, ElementArrayBufferBinding};
pub use shader::{Shader, ShaderType, VERTEX_SHADER, FRAGMENT_SHADER};
pub use program::{Program, ProgramBinder, ProgramBinding,
                  ProgramAttrib, ProgramUniform};
pub use framebuffer::{Framebuffer, FramebufferBinder, FramebufferBinding};
pub use renderbuffer::{Renderbuffer, RenderbufferBinder, RenderbufferBinding};
pub use texture::{Texture, TextureType, Tx2d, TxCubeMap,
                  Texture2dBinder, TextureCubeMapBinder,
                  TextureBinding, Texture2dBinding, TextureCubeMapBinding,
                  ImageTargetType, Tx2dImageTarget, TxCubeMapImageTarget,
                  Texture2d, TextureCubeMap, TextureBindingTarget,
                  TextureFilter, TextureMipmapFilter, TextureWrapMode,
                  TEXTURE_2D, TEXTURE_CUBE_MAP,
                  TEXTURE_CUBE_MAP_POSITIVE_X, TEXTURE_CUBE_MAP_NEGATIVE_X,
                  TEXTURE_CUBE_MAP_POSITIVE_Y, TEXTURE_CUBE_MAP_NEGATIVE_Y,
                  TEXTURE_CUBE_MAP_POSITIVE_Z, TEXTURE_CUBE_MAP_NEGATIVE_Z,
                  TEXTURE_2D_TARGET, LINEAR, NEAREST, LINEAR_MIPMAP_LINEAR,
                  LINEAR_MIPMAP_NEAREST, NEAREST_MIPMAP_LINEAR,
                  NEAREST_MIPMAP_NEAREST, CLAMP_TO_EDGE,
                  REPEAT, MIRRORED_REPEAT};
pub use texture_units::{TextureUnit, TextureUnits, TextureUnitBinding,
                        TextureUnit0, TextureUnit1, TextureUnit2, TextureUnit3,
                        TextureUnit4, TextureUnit5, TextureUnit6, TextureUnit7};
pub use image_data::{Image2d, Pixels, Pixel, ImageFormat,
                     TextelType, TextelFormat, RGB, RGBA, ALPHA,
                     UNSIGNED_BYTE_TEXTEL, UNSIGNED_SHORT_5_6_5,
                     UNSIGNED_SHORT_4_4_4_4, UNSIGNED_SHORT_5_5_5_1};
pub use uniform_data::{UniformData, UniformDatum, UniformPrimitive,
                       UniformPrimitiveType, UniformDatumType};
pub use vertex_data::{VertexData, VertexDatum,
                      VertexBytes, VertexAttribBinder};
pub use index_data::{IndexData, IndexDatum, IndexDatumType};
pub use vertex_buffer::{VertexBuffer, VertexBufferBinding,
                        IndexBuffer, IndexBufferBinding};
pub use types::{Color, Viewport, GLError, BufferBits,
                DrawingMode, DataType,
                COLOR_BUFFER_BIT, DEPTH_BUFFER_BIT, STENCIL_BUFFER_BIT,
                POINTS, LINE_STRIP, LINE_LOOP, LINES,
                TRIANGLE_STRIP, TRIANGLE_FAN, TRIANGLES,
                BYTE, UNSIGNED_BYTE, SHORT, UNSIGNED_SHORT,
                FIXED, FLOAT};
