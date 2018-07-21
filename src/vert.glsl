#version 450

layout(location = 0) in vec4 position;

layout(location = 1) in vec4 shape_posit;
layout(location = 2) in vec4 normal;

layout(location = 0) out vec4 fourd_color;
layout(location = 2) out vec4 view_posit;
layout(location = 3) out vec4 diffuse_direction;
layout(location = 4) out vec4 diffuse_color;
layout(location = 6) out vec4 normal_;
layout(location = 7) out vec4 frag_pos;

layout(set = 0, binding = 0) uniform Data {
    mat4 model;
    mat4 view;
    mat4 proj;
    vec4 cam_position;

    vec4 ambient_color;
    vec4 diffuse_color;
    vec4 diffuse_direction;

    float ambient_intensity;
    float diffuse_intensity;

    float color_max;
    float shape_opacity;
} uniforms;

void main() {
    // For model transform, position after the transform
    vec4 positioned_pt = (uniforms.model * position) + shape_posit;
    // for view transform, position first.
    positioned_pt = uniforms.view * (positioned_pt - uniforms.cam_position);

    // This operation scales the 4d components based on the light
    // distance creating their projection.

    gl_Position = uniforms.proj * positioned_pt;  // todo 3d

    // Now calculate the color, based on passed u dist from cam.
    float u_dist = uniforms.cam_position[3] - positioned_pt[3];

    float portion_through = abs(u_dist) / uniforms.color_max;

    if (portion_through > 1.) {
        portion_through = 1.;
    }

    float base_gray = 0.0;
    float color_val = base_gray + portion_through;

    if (u_dist > 0.) {
        fourd_color = vec4(base_gray, base_gray, color_val, uniforms.shape_opacity);  // Blue
    } else {
        fourd_color = vec4(color_val, base_gray, base_gray, uniforms.shape_opacity);  // Red
    }
    fourd_color = fourd_color * uniforms.ambient_intensity;

    view_posit = positioned_pt;
    diffuse_direction = uniforms.diffuse_direction;
    diffuse_color = uniforms.diffuse_color;

    normal_ = uniforms.model * normal;
    frag_pos = positioned_pt;
}