#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use cgmath::{prelude::*, Vector2};
use glfw::{Action, Context as _, Key, WindowEvent};
use image::GenericImageView;
use luminance::{
    context::GraphicsContext as _,
    pipeline::{PipelineState, TextureBinding},
    pixel::{NormRGB8UI, NormUnsigned, Unsigned, R8UI},
    render_state::RenderState,
    shader::Uniform,
    tess::Mode,
    texture::{Dim2, Dim2Array, GenMipmaps, MagFilter, Sampler},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use noise::{NoiseFn, Perlin, Seedable};
use rand::random;
use std::collections::HashSet;

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
    #[uniform(unbound)]
    view: Uniform<[f32; 2]>,
    #[uniform(unbound)]
    texles: Uniform<TextureBinding<Dim2Array, NormUnsigned>>,
    #[uniform(unbound)]
    world: Uniform<TextureBinding<Dim2, Unsigned>>,
    #[uniform(unbound)]
    world_size: Uniform<u32>,
}

#[derive(Debug, Clone, Copy, Semantics)]
pub enum VertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
}

#[derive(Debug, Clone, Copy, Vertex)]
#[vertex(sem = "VertexSemantics")]
pub struct Vertex {
    position: VertexPosition,
}

const VERTICES: [Vertex; 6] = [
    Vertex::new(VertexPosition::new([1.0, 1.0])),
    Vertex::new(VertexPosition::new([1.0, -1.0])),
    Vertex::new(VertexPosition::new([-1.0, 1.0])),
    Vertex::new(VertexPosition::new([-1.0, -1.0])),
    Vertex::new(VertexPosition::new([-1.0, 1.0])),
    Vertex::new(VertexPosition::new([1.0, -1.0])),
];

fn main() -> Result<()> {
    let mut surface = GlfwSurface::new_gl33(
        "Rust Graphics Test",
        WindowOpt::default().set_dim(WindowDim::Windowed {
            width: 960,
            height: 540,
        }),
    )?;

    let mut back_buffer = surface.context.back_buffer()?;

    let mut program = surface
        .context
        .new_shader_program::<VertexSemantics, (), ShaderInterface>()
        .from_strings(VERTEX_SHADER, None, None, FRAGMENT_SHADER)?
        .ignore_warnings();

    let quad = surface
        .context
        .new_tess()
        .set_vertices(&VERTICES[..])
        .set_mode(Mode::Triangle)
        .build()?;

    const TEXTURE_SIZE: u32 = 16;
    const TEXTURE_COUNT: u32 = 3;

    let mut texture = surface.context.new_texture::<Dim2Array, NormRGB8UI>(
        ([TEXTURE_SIZE, TEXTURE_SIZE], TEXTURE_COUNT),
        2,
        Sampler {
            mag_filter: MagFilter::Nearest,
            ..Default::default()
        },
    )?;

    let atlas = image::open("assets/pallet.png")?.to_rgb8();
    let (atlas_width, atlas_height) = atlas.dimensions();

    let texles: Vec<_> = (0..atlas_height)
        .step_by(TEXTURE_SIZE as usize)
        .flat_map(|y| {
            (0..atlas_width)
                .step_by(TEXTURE_SIZE as usize)
                .map(move |x| (x, y))
        })
        .take(TEXTURE_COUNT as usize)
        .flat_map(|(x, y)| {
            atlas
                .view(x, y, TEXTURE_SIZE, TEXTURE_SIZE)
                .to_image()
                .into_raw()
                .into_iter()
        })
        .collect();

    texture.upload_raw(GenMipmaps::Yes, &texles)?;

    const WORLD_SIZE: usize = 64;

    let mut world = surface.context.new_texture::<Dim2, R8UI>(
        [WORLD_SIZE as u32, WORLD_SIZE as u32],
        0,
        Sampler {
            mag_filter: MagFilter::Nearest,
            ..Default::default()
        },
    )?;

    let world_noise = Perlin::new().set_seed(random());
    let mut tiles = [0; WORLD_SIZE * WORLD_SIZE];

    for (i, tile) in tiles.iter_mut().enumerate() {
        let i = [
            (i % WORLD_SIZE) as f64 / WORLD_SIZE as f64,
            (i / WORLD_SIZE) as f64 / WORLD_SIZE as f64,
        ];

        *tile = (world_noise.get(i) * TEXTURE_COUNT as f64).trunc() as u8;
    }

    world.upload(GenMipmaps::No, &tiles)?;

    let mut view = Vector2::new(0.0, 0.0);

    let mut pressed_keys = HashSet::new();

    const SPEED: f32 = 0.05;

    'main: loop {
        surface.context.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&surface.events_rx) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'main
                }

                WindowEvent::Key(Key::R, _, Action::Press, _) => {
                    view = Vector2::new(0.0, 0.0);
                }

                WindowEvent::Key(key, _, Action::Press, _) => {
                    pressed_keys.insert(key);
                }
                WindowEvent::Key(key, _, Action::Release, _) => {
                    pressed_keys.remove(&key);
                }

                WindowEvent::FramebufferSize(..) => {
                    back_buffer = surface.context.back_buffer()?;
                }

                _ => {}
            }
        }

        {
            let mut position = Vector2::new(0.0, 0.0);

            if pressed_keys.contains(&Key::W) {
                position.y += 1.0;
            }
            if pressed_keys.contains(&Key::A) {
                position.x -= 1.0;
            }
            if pressed_keys.contains(&Key::S) {
                position.y -= 1.0;
            }
            if pressed_keys.contains(&Key::D) {
                position.x += 1.0;
            }

            if !position.is_zero() {
                view += position.normalize() * SPEED;
            }
        }

        let render = surface
            .context
            .new_pipeline_gate()
            .pipeline(
                &back_buffer,
                &PipelineState::default(),
                |pipeline, mut shade_gate| {
                    let bound_texture = pipeline.bind_texture(&mut texture)?;
                    let bound_world = pipeline.bind_texture(&mut world)?;

                    shade_gate.shade(&mut program, |mut interface, uniforms, mut render_gate| {
                        interface.set(&uniforms.view, view.into());
                        interface.set(&uniforms.texles, bound_texture.binding());
                        interface.set(&uniforms.world, bound_world.binding());
                        interface.set(&uniforms.world_size, WORLD_SIZE as u32);

                        render_gate.render(&RenderState::default(), |mut tess_gate| {
                            tess_gate.render(&quad)
                        })
                    })
                },
            )
            .assume();

        if render.is_ok() {
            surface.context.window.swap_buffers();
        } else {
            break 'main;
        }
    }

    Ok(())
}
