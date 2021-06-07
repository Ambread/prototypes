uniform vec2 view;

in vec2 position;

out vec2 vertex_uv;

void main() {
    vertex_uv = position * .5 + .5;

    gl_Position = vec4(position - view, 0.0, 1.0);
}
