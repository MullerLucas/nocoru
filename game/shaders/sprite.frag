#version 460

layout(set = 0, binding = 1) uniform SceneData {
    vec4 tint;
    vec4 sun_color;
    vec4 sun_direction;
} scene_data;

layout(set = 2, binding = 0) uniform sampler2D texture_sampler;

layout(location = 0) in vec2 in_tex_coord;

layout(location = 0) out vec4 out_color;



void main() {
    out_color = texture(texture_sampler, in_tex_coord);
}
