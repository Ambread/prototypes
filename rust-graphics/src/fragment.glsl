uniform sampler2DArray texles;
uniform uint current;

in vec2 vertex_uv;

out vec4 fragment_color;

void main() {
   fragment_color = texture(texles, vec3(vertex_uv, current));
}
