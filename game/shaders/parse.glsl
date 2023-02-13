// 460 allows us to use gl_BaseInstance
#version 460

uniform GlobalUbo {
    mat4 view;
    mat4 proj;
    mat4 view_proj;
    mat4 reserved_0;
} global_ubo;

uniform LocalUbo {
    mat4 model;
} local_ubo;


layout in vec3 in_pos;
layout in vec2 in_tex_coord;
layout out vec2 out_tex_coord;



void main() {
    // gl_Position = global_ubo.view_proj * push_constants.model *  vec4(in_pos, 1.0);
    mat4 model = local_storage.data[push_constants.local_idx].model;
    gl_Position = global_ubo.view_proj * model *  vec4(in_pos, 1.0);

    // gl_Position = global_ubo.view_proj * push_constants.model * vec4(in_pos, 1.0);
}
