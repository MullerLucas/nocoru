// 460 allows us to use gl_BaseInstance
#version 460

layout(set = 0, binding = 0) uniform UBOCamera {
    mat4 view;
    mat4 proj;
    mat4 view_proj;
} camera_data;

layout(push_constant) uniform constants {
    mat4 model;
} push_constants;

struct ObjectData {
    mat4 model;
};

struct FontCharData {
    int x;
    int y;
    int width;
    int height;
};

// std140 enforces cpp memory layout
layout(std140, set = 1, binding = 0) readonly buffer ObjectBuffer {
    ObjectData objects[];
} object_buffer;

layout(location = 0) in vec4 in_pos;
layout(location = 1) in vec4 in_color;
layout(location = 2) in vec2 in_tex_coord;

layout(location = 0) out vec4 out_color;
layout(location = 1) out vec2 out_tex_coord;



void main() {
    // 'gl_BaseInstance' corresponds to 'first_instance' paramterer
    mat4 model = object_buffer.objects[gl_BaseInstance].model;
    // mat4 transform_mat = (camera_data.view_proj * push_constants.model);
    mat4 transform_mat = (camera_data.view_proj * model);
    gl_Position = transform_mat * in_pos;

    out_color = in_color;

    float x_offset = 49.0  / 449.0;
    float y_offset = 255.0 / 449.0;

    float width  = 41.0 / 449.0;
    float height = 53.0 / 449.0;

    out_tex_coord = vec2(
        in_tex_coord.x * width  + x_offset,
        in_tex_coord.y * height + y_offset
    );
}
