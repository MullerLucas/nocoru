#version 460

layout(set = 0, binding = 1) uniform sampler2D global_tex;

layout(location = 0) in vec2 in_tex_coord;
layout(location = 0) out vec4 out_color;

void main() {
    // out_color = vec4(1.0, 1.0, 0.0, 1.0);
    out_color = texture(global_tex, in_tex_coord);
}