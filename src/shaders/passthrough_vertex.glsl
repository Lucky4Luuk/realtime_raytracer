#version 450

//Vertex semantics
in vec3 position;

out vec2 uv;

void main() {
    uv = (position.xy + 1.0) / 2.0;
    gl_Position = vec4(position, 1.0);
}
