#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod assets;

use anyhow::Result;
use assets::{FlatWorldGenerator, NoiseWorldGenerator};
use cgmath::Vector2;
use glfw::{Action, Context as _, Key, WindowEvent};
use luminance::{
    context::GraphicsContext as _,
    pipeline::{PipelineState, TextureBinding},
    pixel::{NormRGB8UI, NormUnsigned, Unsigned, R8UI},
    render_state::RenderState,
    shader::Uniform,
    tess::Mode,
    texture::{Dim2, Dim2Array, GenMipmaps, MagFilter, MinFilter, Sampler},
};
use luminance_derive::{Semantics, UniformInterface, Vertex};
use luminance_glfw::GlfwSurface;
use luminance_windowing::{WindowDim, WindowOpt};
use noise::{NoiseFn, Perlin, Seedable};
use rand::random;
use std::{collections::HashMap, env::current_dir};

use crate::assets::Assets;

const VERTEX_SHADER: &str = include_str!("vertex.glsl");
const FRAGMENT_SHADER: &str = include_str!("fragment.glsl");

#[derive(Debug, UniformInterface)]
struct ShaderInterface {
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

#[derive(Debug, Clone)]
struct World {
    chunks: HashMap<Vector2<isize>, Chunk>,
}

#[derive(Clone)]
struct Chunk {
    tiles: [u8; Self::SIZE * Self::SIZE],
    position: Vector2<isize>,
}

impl Chunk {
    const SIZE: usize = 32;
    const INVERT: bool = false;

    fn new(position: Vector2<isize>) -> Self {
        Self {
            position,
            tiles: [0; Self::SIZE * Self::SIZE],
        }
    }

    fn generate(&mut self, assets: &Assets) {
        use assets::WorldGenerator::*;

        match &assets.world_data {
            Flat(_) => todo!(),
            Noise(gen) => self.generate_noise(gen, assets),
        }
    }

    fn generate_flat(&mut self, gen: &FlatWorldGenerator, assets: &Assets) {}

    fn generate_noise(&mut self, gen: &NoiseWorldGenerator, assets: &Assets) {
        let noise = Perlin::new().set_seed(gen.seed);
        let tile_count = gen.tiles.len();

        for (i, tile) in self.tiles.iter_mut().enumerate() {
            let i = [
                ((i % Self::SIZE) as isize + self.position.x * Self::SIZE as isize) as f64
                    / gen.scale,
                ((i / Self::SIZE) as isize + self.position.y * Self::SIZE as isize) as f64
                    / gen.scale,
            ];

            *tile = ((noise.get(i) * (tile_count + 1) as f64).trunc() as u8).min(tile_count as u8);

            if Self::INVERT {
                *tile = tile_count as u8 - *tile;
            }
        }
    }
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, tiles) in self.tiles.chunks(Chunk::SIZE).enumerate() {
            write!(f, "{}  ", i)?;
            for tile in tiles {
                write!(f, "{}", tile)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let assets = Assets::from_path(current_dir()?.join("assets"))?;

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

    let mut texture = surface.context.new_texture::<Dim2Array, NormRGB8UI>(
        (
            assets.tile_data.texture_size.cast().unwrap().into(),
            assets.tile_data.texture_count as u32,
        ),
        2,
        Sampler {
            mag_filter: MagFilter::Nearest,
            ..Default::default()
        },
    )?;

    texture.upload_raw(GenMipmaps::Yes, &assets.tile_sprites)?;

    let mut world = surface.context.new_texture::<Dim2, R8UI>(
        [Chunk::SIZE as u32, Chunk::SIZE as u32],
        0,
        Sampler {
            mag_filter: MagFilter::Nearest,
            min_filter: MinFilter::Nearest,
            ..Default::default()
        },
    )?;

    let mut chunk = Chunk::new(Vector2::new(0, 0));
    let mut chunk_noise = Perlin::new().set_seed(random());

    chunk.generate(&assets);
    world.upload(GenMipmaps::No, &chunk.tiles)?;

    'main: loop {
        surface.context.window.glfw.poll_events();
        for (_, event) in glfw::flush_messages(&surface.events_rx) {
            match event {
                WindowEvent::Close | WindowEvent::Key(Key::Escape, _, Action::Release, _) => {
                    break 'main
                }

                WindowEvent::Key(key, _, Action::Press, _) => {
                    match key {
                        Key::W => chunk.position.y += 1,
                        Key::A => chunk.position.x -= 1,
                        Key::S => chunk.position.y -= 1,
                        Key::D => chunk.position.x += 1,

                        Key::Space => chunk_noise = chunk_noise.set_seed(random()),

                        Key::P => println!("{:?}", chunk),

                        _ => {}
                    }

                    chunk.generate(&assets);
                    world.upload(GenMipmaps::No, &chunk.tiles)?;
                }

                WindowEvent::FramebufferSize(..) => {
                    back_buffer = surface.context.back_buffer()?;
                }

                _ => {}
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
                        interface.set(&uniforms.texles, bound_texture.binding());
                        interface.set(&uniforms.world, bound_world.binding());
                        interface.set(&uniforms.world_size, Chunk::SIZE as u32);

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
