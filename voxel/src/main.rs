pub mod camera;

use anyhow::Result;
use camera::Camera;
use cgmath::{Vector2, Vector3};
use glfw::{Action, Context, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _, pipeline::PipelineState, render_state::RenderState,
    shader::Uniform, tess::Mode,
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use std::collections::HashSet;

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    #[uniform(unbound)]
    projection: Uniform<[[f32; 4]; 4]>,
}

#[derive(Debug, Clone, Copy, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 3]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 3]", wrapper = "VertexRGB")]
    Color,
}

#[derive(Debug, Clone, Copy, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    position: VertexPosition,
    #[vertex(normalized = "true")]
    color: VertexRGB,
}

const COLORS: &[VertexRGB] = &[
    VertexRGB::new([255, 0, 0]),
    VertexRGB::new([0, 255, 0]),
    VertexRGB::new([0, 0, 255]),
    VertexRGB::new([255, 255, 0]),
    VertexRGB::new([0, 255, 255]),
    VertexRGB::new([255, 0, 255]),
];

const VERTICES: [Vertex; 36] = [
    // Face 1
    Vertex::new(VertexPosition::new([-0.5, -0.5, -0.5]), COLORS[0]),
    Vertex::new(VertexPosition::new([-0.5, 0.5, -0.5]), COLORS[0]),
    Vertex::new(VertexPosition::new([0.5, -0.5, -0.5]), COLORS[0]),
    Vertex::new(VertexPosition::new([-0.5, 0.5, -0.5]), COLORS[0]),
    Vertex::new(VertexPosition::new([0.5, 0.5, -0.5]), COLORS[0]),
    Vertex::new(VertexPosition::new([0.5, -0.5, -0.5]), COLORS[0]),
    // Face 2
    Vertex::new(VertexPosition::new([-0.5, -0.5, 0.5]), COLORS[1]),
    Vertex::new(VertexPosition::new([0.5, -0.5, 0.5]), COLORS[1]),
    Vertex::new(VertexPosition::new([-0.5, 0.5, 0.5]), COLORS[1]),
    Vertex::new(VertexPosition::new([-0.5, 0.5, 0.5]), COLORS[1]),
    Vertex::new(VertexPosition::new([0.5, -0.5, 0.5]), COLORS[1]),
    Vertex::new(VertexPosition::new([0.5, 0.5, 0.5]), COLORS[1]),
    // Face 3
    Vertex::new(VertexPosition::new([-0.5, 0.5, -0.5]), COLORS[2]),
    Vertex::new(VertexPosition::new([-0.5, 0.5, 0.5]), COLORS[2]),
    Vertex::new(VertexPosition::new([0.5, 0.5, -0.5]), COLORS[2]),
    Vertex::new(VertexPosition::new([-0.5, 0.5, 0.5]), COLORS[2]),
    Vertex::new(VertexPosition::new([0.5, 0.5, 0.5]), COLORS[2]),
    Vertex::new(VertexPosition::new([0.5, 0.5, -0.5]), COLORS[2]),
    // Face 4
    Vertex::new(VertexPosition::new([-0.5, -0.5, -0.5]), COLORS[3]),
    Vertex::new(VertexPosition::new([0.5, -0.5, -0.5]), COLORS[3]),
    Vertex::new(VertexPosition::new([-0.5, -0.5, 0.5]), COLORS[3]),
    Vertex::new(VertexPosition::new([-0.5, -0.5, 0.5]), COLORS[3]),
    Vertex::new(VertexPosition::new([0.5, -0.5, -0.5]), COLORS[3]),
    Vertex::new(VertexPosition::new([0.5, -0.5, 0.5]), COLORS[3]),
    // Face 5
    Vertex::new(VertexPosition::new([-0.5, -0.5, -0.5]), COLORS[4]),
    Vertex::new(VertexPosition::new([-0.5, -0.5, 0.5]), COLORS[4]),
    Vertex::new(VertexPosition::new([-0.5, 0.5, -0.5]), COLORS[4]),
    Vertex::new(VertexPosition::new([-0.5, -0.5, 0.5]), COLORS[4]),
    Vertex::new(VertexPosition::new([-0.5, 0.5, 0.5]), COLORS[4]),
    Vertex::new(VertexPosition::new([-0.5, 0.5, -0.5]), COLORS[4]),
    // Face 6
    Vertex::new(VertexPosition::new([0.5, -0.5, -0.5]), COLORS[5]),
    Vertex::new(VertexPosition::new([0.5, 0.5, -0.5]), COLORS[5]),
    Vertex::new(VertexPosition::new([0.5, -0.5, 0.5]), COLORS[5]),
    Vertex::new(VertexPosition::new([0.5, -0.5, 0.5]), COLORS[5]),
    Vertex::new(VertexPosition::new([0.5, 0.5, -0.5]), COLORS[5]),
    Vertex::new(VertexPosition::new([0.5, 0.5, 0.5]), COLORS[5]),
];

fn main() -> Result<()> {
    let surface = GlfwSurface::new_gl33(
        "Hello, world!",
        WindowOpt::default().set_dim(WindowDim::Windowed {
            width: 768,
            height: 768,
        }),
    )?;

    let mut window_size = Vector2::new(768, 768);

    let mut context = surface.context;
    let events = surface.events_rx;
    let mut back_buffer = context.back_buffer()?;

    const CHUNK_SIZE: u8 = 10;
    const SKY_BOX_SIZE: f32 = 40.0;

    let vertices: Vec<_> = (0..CHUNK_SIZE)
        .flat_map(|x| {
            (0..CHUNK_SIZE).flat_map(move |y| {
                (0..CHUNK_SIZE)
                    .map(move |z| Vector3::new(x as f32 * 2.0, y as f32 * 2.0, z as f32 * 2.0))
            })
        })
        .flat_map(|i| {
            std::array::IntoIter::new(VERTICES).map(move |mut it| {
                it.position.repr[0] += i.x;
                it.position.repr[1] += i.y;
                it.position.repr[2] += i.z;
                it
            })
        })
        .chain(std::array::IntoIter::new(VERTICES).map(move |mut it| {
            it.position.repr[0] *= SKY_BOX_SIZE;
            it.position.repr[1] *= SKY_BOX_SIZE;
            it.position.repr[2] *= SKY_BOX_SIZE;
            it
        }))
        .collect();

    let tess = context
        .new_tess()
        .set_vertices(vertices)
        .set_mode(Mode::Triangle)
        .build()?;

    let mut program = context
        .new_shader_program::<VertexSemantics, (), ShaderInterface>()
        .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)?
        .ignore_warnings();

    let mut camera = Camera::new();
    let mut pressed_keys = HashSet::new();

    loop {
        context.window.glfw.poll_events();

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    return Ok(())
                }

                WindowEvent::FramebufferSize(x, y) => {
                    window_size.x = x;
                    window_size.y = y;
                    back_buffer = context.back_buffer()?;
                }

                WindowEvent::Key(key, _, Action::Press, _) => {
                    pressed_keys.insert(key);
                }
                WindowEvent::Key(key, _, Action::Release, _) => {
                    pressed_keys.remove(&key);
                }

                _ => {}
            }
        }

        camera.update(&pressed_keys);

        context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default(),
                |_, mut shade_gate| {
                    shade_gate.shade(&mut program, |mut interface, uniforms, mut render_gate| {
                        interface.set(
                            &uniforms.projection,
                            camera.get_projection(&window_size).into(),
                        );

                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&tess)
                        })
                    })
                },
            )
            .assume()
            .into_result()?;

        context.window.swap_buffers();
    }
}
