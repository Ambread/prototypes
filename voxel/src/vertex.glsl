uniform mat4 projection;

in vec4 position;
in vec3 color;

out vec3 v_color;

void main() {
    v_color = color;
    gl_Position = projection * position;
}
