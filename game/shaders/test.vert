// 460 allows us to use gl_BaseInstance
#version 460


layout(set = 0, binding = 0) uniform GlobalUbo{
    mat4 view;
    mat4 proj;
    mat4 view_proj;
    mat4 reserved_0;
} global_ubo;



layout(location = 0) in vec3 in_pos;
layout(location = 1) in vec2 in_tex_coord;

layout(location = 0) out vec2 out_tex_coord;



void main() {
    gl_Position = global_ubo.view_proj * vec4(in_pos, 1.0);
    // gl_Position = vec4(in_pos, 1.0);
    out_tex_coord = in_tex_coord;
}
