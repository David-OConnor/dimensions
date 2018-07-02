pub mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
    #version 450

    layout(location = 0) in vec4 position;

    layout(location = 1) in vec4 shape_posit;
    layout(location = 2) in vec4 normal;

    layout(location = 0) out vec4 fourd_color;
    layout(location = 1) out vec4 diffuse;
    layout(location = 2) out vec4 view_posit;
    layout(location = 3) out vec4 diffuse_light_direction;
    layout(location = 4) out vec4 diffuse_light_color;
    layout(location = 5) out float specular_intensity;
    layout(location = 6) out vec4 norm2;

    layout(set = 0, binding = 0) uniform Data {
        mat4 model;
        mat4 view;
        mat4 proj;
        vec4 cam_position;

        vec4 ambient_light_color;
        vec4 diffuse_light_color;
        vec4 diffuse_light_direction;

        float ambient_intensity;
        float diffuse_intensity;
        float specular_intensity;
        float color_max;
        float shape_opacity;
    } uniforms;

    void main() {
        // For model transform, position after the transform
        vec4 positioned_pt = (uniforms.model * position) + shape_posit;
        // for view transform, position first.
        positioned_pt = uniforms.view * (positioned_pt - uniforms.cam_position);

        // Now remove the u coord; replace with 1. We no longer need it,
        // and the projection matrix is set up for 3d homogenous vectors.
        float u = positioned_pt[3];
        vec4 positioned_3d = vec4(positioned_pt[0], positioned_pt[1], positioned_pt[2], 1.);

        gl_Position = uniforms.proj * positioned_3d;

        // Now calculate the color, based on passed u dist from cam.
        float u_dist = uniforms.cam_position[3] - u;

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

        // Process diffuse lighting from a single-directional source.
        // We can use the model matrix directly on the normal, since it
        // only scales uniformly, and isn't homogenous (doesn't translate).
        vec4 norm = normalize(uniforms.model * normal);
        vec4 dir = normalize(uniforms.diffuse_light_direction);

        float directional_light_weight = max(dot(norm, dir), 0.);
        diffuse = uniforms.diffuse_light_color * directional_light_weight *
            uniforms.diffuse_intensity;

        view_posit = positioned_pt;
        diffuse_light_direction = uniforms.diffuse_light_direction;
        diffuse_light_color = uniforms.diffuse_light_color;
        specular_intensity = uniforms.specular_intensity;
        norm2 = norm;

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
    layout(location = 1) in vec4 diffuse;
    layout(location = 2) in vec4 view_posit;
    layout(location = 3) in vec4 diffuse_light_direction;
    layout(location = 4) in vec4 diffuse_light_color;
    layout(location = 5) in float specular_intensity;
    layout(location = 6) in vec4 norm2;
//    layout(location = 6) in FragPos;

    layout(location = 0) out vec4 f_color;

    void main() {

        vec4 light_dir = normalize(diffuse_light_direction);
        vec4 view_norm = normalize(-norm2);
        vec4 R = normalize(reflect(light_dir, view_norm));

        float specular = specular_intensity * pow(max(dot(R, view_norm), 0.0), 32.);

        f_color = mix(fourd_color, diffuse, 0.5) + specular * diffuse_light_color;
    }
    "]
        struct Dummy;
}
