mod quad;
mod texture;

use super::{
    shader::{FRAGMENT_SHADER, VERTEX_SHADER},
    uniforms::ShaderInterface,
};
use luminance::{
    context::GraphicsContext,
    framebuffer::{Framebuffer, FramebufferError},
    shader::{self, ProgramError},
    texture::Dim2,
};
use luminance_gl::GL33;
use luminance_glfw::GlfwSurface;
use quad::VertexSemantics;
pub use quad::{create_quad, Quad};
pub use texture::{create_tile_texture, create_world_texture, TileTexture, WorldTexture};

pub type Program = shader::Program<GL33, VertexSemantics, (), ShaderInterface>;

pub fn create_program(surface: &mut GlfwSurface) -> Result<Program, ProgramError> {
    Ok(surface
        .context
        .new_shader_program::<VertexSemantics, (), ShaderInterface>()
        .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)?
        .ignore_warnings())
}

pub type BackBuffer = Framebuffer<GL33, Dim2, (), ()>;

pub fn create_back_buffer(surface: &mut GlfwSurface) -> Result<BackBuffer, FramebufferError> {
    surface.context.back_buffer()
}
