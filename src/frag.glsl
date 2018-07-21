#version 450

//    uniform struct LightSource {
//        vec4 position;
//        vec4 color;
//        float intensity;
//    } light_source;

layout(location = 0) in vec4 fourd_color;
layout(location = 2) in vec4 view_posit;
layout(location = 3) in vec4 diffuse_direction;
layout(location = 4) in vec4 diffuse_color;

layout(location = 6) in vec4 normal_;
layout(location = 7) in vec4 frag_pos;

layout(location = 0) out vec4 f_color;

void main() {

//        vec4 light_dir = normalize(diffuse_direction);
//        vec4 view_norm = normalize(-norm2);
//        vec4 R = normalize(reflect(light_dir, view_norm));

//        float specular = specular_intensity * pow(max(dot(R, view_norm), 0.0), 32.);

//        f_color = mix(fourd_color, diffuse, 0.5) + specular * diffuse_color;



    // todo test point light source.
//        float dist_from_light = sqrt(
//            pow(positioned_pt[0] - uniforms.pt_light_position[0], 2.) +
//            pow(positioned_pt[1] - uniforms.pt_light_position[1], 2.) +
//            pow(positioned_pt[2] - uniforms.pt_light_position[2], 2.) +
//            pow(positioned_pt[3] - uniforms.pt_light_position[3], 2.)
//        );
//        float diffuse_brightness = uniforms.pt_light_brightness / pow(dist_from_light, 3.);

    // todo may not need this direc. Is direc from light to point.
//        vec4 diffuse_direction = positioned_pt - uniforms.pt_light_position;

    // Calculate diffuse lighting.
    vec4 norm = normalize(normal_);
    vec4 dir = normalize(diffuse_direction);
    float diffuse_weight = max(dot(norm, dir), 0.);
    vec4 diffuse = diffuse_color * diffuse_weight * 1.;

//        f_color = diffuse;
    f_color = (diffuse + fourd_color) / 2;
}