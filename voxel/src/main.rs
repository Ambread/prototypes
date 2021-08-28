pub mod camera;

use anyhow::Result;
use camera::Camera;
use cgmath::{vec3, Vector2, Vector3};
use glfw::{Action, Context, CursorMode, Key, WindowEvent};
use itertools::iproduct;
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

const COLORS: [Vector3<u8>; 6] = [
    vec3(255, 0, 0),
    vec3(0, 255, 0),
    vec3(0, 0, 255),
    vec3(255, 255, 0),
    vec3(0, 255, 255),
    vec3(255, 0, 255),
];

const CUBE: [Vector3<f32>; 36] = [
    // Face 1
    vec3(-0.5, -0.5, -0.5),
    vec3(-0.5, 0.5, -0.5),
    vec3(0.5, -0.5, -0.5),
    vec3(-0.5, 0.5, -0.5),
    vec3(0.5, 0.5, -0.5),
    vec3(0.5, -0.5, -0.5),
    // Face 2
    vec3(-0.5, -0.5, 0.5),
    vec3(0.5, -0.5, 0.5),
    vec3(-0.5, 0.5, 0.5),
    vec3(-0.5, 0.5, 0.5),
    vec3(0.5, -0.5, 0.5),
    vec3(0.5, 0.5, 0.5),
    // Face 3
    vec3(-0.5, 0.5, -0.5),
    vec3(-0.5, 0.5, 0.5),
    vec3(0.5, 0.5, -0.5),
    vec3(-0.5, 0.5, 0.5),
    vec3(0.5, 0.5, 0.5),
    vec3(0.5, 0.5, -0.5),
    // Face 4
    vec3(-0.5, -0.5, -0.5),
    vec3(0.5, -0.5, -0.5),
    vec3(-0.5, -0.5, 0.5),
    vec3(-0.5, -0.5, 0.5),
    vec3(0.5, -0.5, -0.5),
    vec3(0.5, -0.5, 0.5),
    // Face 5
    vec3(-0.5, -0.5, -0.5),
    vec3(-0.5, -0.5, 0.5),
    vec3(-0.5, 0.5, -0.5),
    vec3(-0.5, -0.5, 0.5),
    vec3(-0.5, 0.5, 0.5),
    vec3(-0.5, 0.5, -0.5),
    // Face 6
    vec3(0.5, -0.5, -0.5),
    vec3(0.5, 0.5, -0.5),
    vec3(0.5, -0.5, 0.5),
    vec3(0.5, -0.5, 0.5),
    vec3(0.5, 0.5, -0.5),
    vec3(0.5, 0.5, 0.5),
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
    const CHUNK_SPREAD: f32 = 2.0;
    const SKY_BOX_SIZE: f32 = 40.0;

    // For all points in a (CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE) grid
    let vertices = iproduct!(0..CHUNK_SIZE, 0..CHUNK_SIZE, 0..CHUNK_SIZE)
        // Cast position to floats and apply chunk spread
        .map(|position| Vector3::from(position).cast().unwrap() * CHUNK_SPREAD)
        // For each point, create a cube using the const cube vertices and offset it to said point
        .flat_map(|offset| IntoIterator::into_iter(CUBE).map(move |position| position + offset))
        // Add an additional cube as the sky box
        .chain(IntoIterator::into_iter(CUBE).map(|position| position * SKY_BOX_SIZE))
        // Pair each triangle with a color
        .zip(
            IntoIterator::into_iter(COLORS)
                // Each face has 6 triangles which all will be the same color
                .flat_map(|color| std::iter::repeat(color).take(6))
                // Repeat for all cubes
                .cycle(),
        )
        // Turn the positions and colors into vertices for the tesselation to handle
        .map(|(position, color)| {
            Vertex::new(
                VertexPosition::new(position.into()),
                VertexRGB::new(color.into()),
            )
        })
        // Collect into a vec so we can pass to tesselation
        .collect::<Vec<_>>();

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
    let mut cursor_position;
    context.window.set_cursor_mode(CursorMode::Hidden);

    loop {
        cursor_position = Vector2::new(0.0, 0.0);

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

                WindowEvent::CursorPos(x, y) => {
                    let window_center = window_size.cast().unwrap().map(|it: f64| it / 2.0);

                    cursor_position.x = -(window_center.x - x);
                    cursor_position.y = -(window_center.y - y);

                    context
                        .window
                        .set_cursor_pos(window_center.x, window_center.y);
                }

                _ => {}
            }
        }

        camera.update(&cursor_position, &pressed_keys);

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
