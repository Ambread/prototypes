use luminance::{
    context::GraphicsContext,
    tess::{Interleaved, Mode, Tess, TessError},
};
use luminance_derive::{Semantics, Vertex};
use luminance_gl::GL33;
use luminance_glfw::GlfwSurface;

/// Quad that covers the entire viewport
pub type Quad = Tess<GL33, Vertex, (), (), Interleaved>;

/// Quad that covers the entire viewport
pub fn create_quad(surface: &mut GlfwSurface) -> Result<Quad, TessError> {
    surface
        .context
        .new_tess()
        .set_vertices(&VERTICES[..])
        .set_mode(Mode::Triangle)
        .build()
}

// Vertices for a quad that covers the entire viewport
const VERTICES: [Vertex; 6] = [
    Vertex::new(VertexPosition::new([1.0, 1.0])),
    Vertex::new(VertexPosition::new([1.0, -1.0])),
    Vertex::new(VertexPosition::new([-1.0, 1.0])),
    Vertex::new(VertexPosition::new([-1.0, -1.0])),
    Vertex::new(VertexPosition::new([-1.0, 1.0])),
    Vertex::new(VertexPosition::new([1.0, -1.0])),
];

#[derive(Debug, Clone, Copy, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    position: VertexPosition,
}

#[derive(Debug, Clone, Copy, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
}
