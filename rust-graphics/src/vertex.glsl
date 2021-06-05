uniform vec3 color;

in vec2 position;

out vec3 vertex_color;

void main() {
    vertex_color = color;

    gl_Position = vec4(position, 0.0, 1.0);
}
