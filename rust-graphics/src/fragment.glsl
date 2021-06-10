uniform sampler2DArray texles;

uniform usampler2D world;
uniform uint world_size;

in vec2 vertex_uv;

out vec4 fragment_color;

void main() {
    uint tile = texture(world, vertex_uv).r;

    vec2 local_uv = fract(vertex_uv * vec2(world_size));

    fragment_color = texture(texles, vec3(local_uv, tile));
}
