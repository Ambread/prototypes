uniform sampler2DArray tile_texture;

uniform usampler2D world_texture;
uniform uint world_size;

in vec2 vertex_uv;

out vec4 fragment_color;

void main() {
    uint tile = texture(world_texture, vertex_uv).r;

    vec2 local_uv = fract(vertex_uv * vec2(world_size));

    fragment_color = texture(tile_texture, vec3(local_uv, tile));
}
