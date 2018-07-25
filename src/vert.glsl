#version 450

layout(location = 0) in vec4 position;
layout(location = 1) in vec4 shape_posit;
layout(location = 2) in vec4 normal;
layout(location = 3) in float specular_intensity;

layout(location = 0) out vec4 color;

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

vec4 find_fourd_color(vec4 positioned_pt) {
    // calculate a color to represent position in the fourth dimension,
    // based on u dist between vertex and cam.
    float u_dist = uniforms.cam_position[3] - positioned_pt[3];

    float portion_through = abs(u_dist) / uniforms.color_max;

    if (portion_through > 1.) {
        portion_through = 1.;
    }

    float base_gray = 0.0;
    float color_val = base_gray + portion_through;
    vec4 fourd_color;

    if (u_dist > 0.) {
        fourd_color = vec4(base_gray, base_gray, color_val, uniforms.shape_opacity);  // Blue
    } else {
        fourd_color = vec4(color_val, base_gray, base_gray, uniforms.shape_opacity);  // Red
    }
    return fourd_color * uniforms.ambient_intensity;
}

vec4 find_diffuse_color() {
    vec4 norm = normalize(uniforms.model * normalize(normal));
    vec4 dir = normalize(uniforms.diffuse_direction);
    // diffuse_weight is based on the andle of the face compared to the angle
    // of the incoming light.
    float diffuse_weight = max(dot(norm, dir), 0.);
    return uniforms.diffuse_color * diffuse_weight * uniforms.diffuse_intensity;
}

void main() {
    // For model transform, position after the transform
    vec4 positioned_pt = (uniforms.model * position) + shape_posit;
    // for view transform, position first.
    positioned_pt = uniforms.view * (positioned_pt - uniforms.cam_position);

    // gl_Position is a builtin name used to output the projected point.
    gl_Position = uniforms.proj * positioned_pt;

    vec4 fourd_color = find_fourd_color(positioned_pt);
    vec4 diffuse_color = find_diffuse_color();

//        view_posit = positioned_pt;
//        diffuse_direction = uniforms.diffuse_direction;
//        diffuse_color = uniforms.diffuse_color;
//        specular_intensity = uniforms.specular_intensity;

//        normal_ = uniforms.model * normal;
//        frag_pos = positioned_pt;
        color = mix(fourd_color, diffuse_color, 0.5);
}