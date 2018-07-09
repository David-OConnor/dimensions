pub mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
    #version 450

    layout(location = 0) in vec4 position;

    layout(location = 1) in vec4 shape_posit;
    layout(location = 2) in vec4 normal;

    layout(location = 0) out vec4 fourd_color;
    layout(location = 2) out vec4 view_posit;
    layout(location = 3) out vec4 diffuse_direction;
    layout(location = 4) out vec4 diffuse_color;
    layout(location = 5) out float specular_intensity;
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
        vec4 pt_light_position;

        float ambient_intensity;
        float diffuse_intensity;

        float specular_intensity;
        float color_max;
        float shape_opacity;

        float pt_light_brightness;
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
        specular_intensity = uniforms.specular_intensity;

        normal_ = uniforms.model * normal;
        frag_pos = positioned_pt;
    }
    "]
        struct Dummy;
}

pub mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
    #version 450
    layout(location = 0) in vec4 fourd_color;
    layout(location = 2) in vec4 view_posit;
    layout(location = 3) in vec4 diffuse_direction;
    layout(location = 4) in vec4 diffuse_color;
    layout(location = 5) in float specular_intensity;

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

        f_color = diffuse;
//        f_color = (diffuse + fourd_color) / 2;
    }
    "]
        struct Dummy;
}
