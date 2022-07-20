#version 450

layout (location = 0) in vec3 vector_pos;
layout (location = 2) in vec2 uv;

struct View {
    mat4 view_proj;
    vec3 world_pos;
};

layout (binding = 0) uniform View view;

void main() {
    gl_Position = view.view_proj * vec4(vector_pos, 1.0f);
}