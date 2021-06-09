uniform sampler2DArray texles;
uniform sampler2D tiles;

in vec2 vertex_uv;

out vec4 fragment_color;

void main() {
    vec4 tile = texture(tiles, vertex_uv);

    if (tile.r == 0) {
        fragment_color = vec4(0.0, 1.0, 0.0, 0.0);
    } else {
        fragment_color = vec4(0.0, 0.0, 1.0, 0.0);
    }
}
