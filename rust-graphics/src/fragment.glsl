uniform sampler2D texles;

in vec2 vertex_uv;

out vec4 fragment_color;

void main() {
   fragment_color = texture(texles, vertex_uv);
}
