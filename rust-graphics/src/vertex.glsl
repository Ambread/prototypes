uniform vec3 color;
uniform vec2 view;

in vec2 position;

out vec3 vertex_color;

void main() {
    vertex_color = color;

    gl_Position = vec4(position + view, 0.0, 1.0);
}
