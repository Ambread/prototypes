use anyhow::Result;
use luminance::{
    pipeline::TextureBinding,
    pixel::{NormUnsigned, Unsigned},
    shader::Uniform,
    texture::{Dim2, Dim2Array},
};
use luminance_derive::UniformInterface;

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)]
    pub tile_texture: Uniform<TextureBinding<Dim2Array, NormUnsigned>>,
    #[uniform(unbound)]
    pub world_texture: Uniform<TextureBinding<Dim2, Unsigned>>,
    #[uniform(unbound)]
    pub world_size: Uniform<u32>,
}
