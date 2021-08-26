use std::collections::HashSet;

use anyhow::Result;
use cgmath::{perspective, prelude::*, Deg, Matrix4, Vector2, Vector3};
use glfw::{Action, Context, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _, pipeline::PipelineState, render_state::RenderState,
    shader::Uniform, tess::Mode,
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};

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

const VERTICES: &[Vertex] = &[
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

    let triangle = context
        .new_tess()
        .set_vertices(VERTICES)
        .set_mode(Mode::Triangle)
        .build()?;

    let mut program = context
        .new_shader_program::<VertexSemantics, (), ShaderInterface>()
        .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)?
        .ignore_warnings();

    let mut projection = Matrix4::from([[0.0; 4]; 4]);

    let mut has_rotated = true;
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

        {
            let mut position = Vector3::new(0.0, 0.0, 0.0);

            if pressed_keys.contains(&Key::W) {
                position.z += 1.0;
            }
            if pressed_keys.contains(&Key::A) {
                position.x += 1.0;
            }
            if pressed_keys.contains(&Key::S) {
                position.z -= 1.0;
            }
            if pressed_keys.contains(&Key::D) {
                position.x -= 1.0;
            }
            if pressed_keys.contains(&Key::Space) {
                position.y -= 1.0;
            }
            if pressed_keys.contains(&Key::LeftShift) {
                position.y += 1.0;
            }

            const MOVE_SPEED: f32 = 0.05;

            if !position.is_zero() {
                if has_rotated {
                    has_rotated = false;
                    projection = perspective(
                        Deg(90.0),
                        window_size.x as f32 / window_size.y as f32,
                        0.1,
                        10.0,
                    );
                }

                projection =
                    projection * Matrix4::from_translation(position.normalize() * MOVE_SPEED);
            }

            let mut rotation = Vector2::new(0.0, 0.0);
            if pressed_keys.contains(&Key::Up) {
                rotation.x += 1.0;
            }
            if pressed_keys.contains(&Key::Left) {
                rotation.y += 1.0;
            }
            if pressed_keys.contains(&Key::Down) {
                rotation.x -= 1.0;
            }
            if pressed_keys.contains(&Key::Right) {
                rotation.y -= 1.0;
            }

            const ROTATE_SPEED: f32 = 1.0;

            if !rotation.is_zero() {
                has_rotated = true;

                projection = projection
                    * Matrix4::from_angle_x(Deg(rotation.x * ROTATE_SPEED))
                    * Matrix4::from_angle_y(Deg(rotation.y * ROTATE_SPEED));
            }
        }

        context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default(),
                |_, mut shade_gate| {
                    shade_gate.shade(&mut program, |mut interface, uniforms, mut render_gate| {
                        interface.set(&uniforms.projection, projection.into());

                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&triangle)
                        })
                    })
                },
            )
            .assume()
            .into_result()?;

        context.window.swap_buffers();
    }
}
