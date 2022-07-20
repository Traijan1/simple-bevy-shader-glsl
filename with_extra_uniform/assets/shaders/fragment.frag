#version 450

layout(set=1, binding=0) uniform MyMaterial {
    vec4 Color;   
};

layout(location=0) out vec4 color_out;

void main() {
    color_out = Color;
}