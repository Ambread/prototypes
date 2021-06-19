#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use cgmath::Vector2;
use glfw::{Action, Context as _, Key, WindowEvent};
use image::GenericImageView;
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
use std::{collections::HashMap, path::Path};

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

    fn generate(&mut self, noise: &Perlin, scale: f64) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            let i = [
                ((i % Self::SIZE) as isize + self.position.x * Self::SIZE as isize) as f64 / scale,
                ((i / Self::SIZE) as isize + self.position.y * Self::SIZE as isize) as f64 / scale,
            ];

            *tile = ((noise.get(i) * (TEXTURE_COUNT + 1) as f64).trunc() as u8)
                .min(TEXTURE_COUNT as u8);

            if Self::INVERT {
                *tile = TEXTURE_COUNT as u8 - *tile;
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

const TEXTURE_SIZE: u32 = 16;
const TEXTURE_COUNT: u32 = 3;

fn parse_atlas<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let atlas = image::open(path)?.to_rgb8();
    let (atlas_width, atlas_height) = atlas.dimensions();

    Ok((0..atlas_height)
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
        .collect())
}

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

    let mut texture = surface.context.new_texture::<Dim2Array, NormRGB8UI>(
        ([TEXTURE_SIZE, TEXTURE_SIZE], TEXTURE_COUNT),
        2,
        Sampler {
            mag_filter: MagFilter::Nearest,
            ..Default::default()
        },
    )?;

    texture.upload_raw(GenMipmaps::Yes, &parse_atlas("assets/pallet.png")?)?;

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

    let mut noise_scale: f64 = 2.0;
    println!("noise_scale = {}", noise_scale);

    chunk.generate(&chunk_noise, noise_scale);
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

                        Key::Q => {
                            noise_scale /= 2.0;
                            noise_scale = noise_scale.max(2.0);
                            println!("noise_scale = {}", noise_scale);
                        }
                        Key::E => {
                            noise_scale *= 2.0;
                            println!("noise_scale = {}", noise_scale);
                        }

                        Key::P => println!("{:?}", chunk),

                        _ => {}
                    }

                    chunk.generate(&chunk_noise, noise_scale);
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
