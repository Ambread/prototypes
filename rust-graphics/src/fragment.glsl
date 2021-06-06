uniform sampler2D texles;

in vec2 vertex_uv;

out vec4 fragment_color;

void main() {
   fragment_color = vec4(texture(texles, vertex_uv).r, 0.0, 0.0, 1.0);
}
