pub mod vs {
    #[derive(VulkanoShader)]
    #[ty = "vertex"]
    #[src = "
    #version 450

    layout(location = 0) in vec4 position;

    layout(location = 1) in vec4 shape_posit;
    layout(location = 2) in vec4 cam_posit;
    layout(location = 3) in vec4 normal;

    layout(location = 0) out vec4 fragColor;
    layout(location = 1) out vec4 v_normal;

    layout(set = 0, binding = 0) uniform Data {
        mat4 model;
        mat4 view;
        mat4 proj;
        float color_max;
    } uniforms;

    //out gl_PerVertex {
    //    vec4 gl_Position;
    //};

    void main() {
        // For model transform, position after the transform
        vec4 positioned_pt = (uniforms.model * position) + shape_posit;
        // for view transform, position first.
        positioned_pt = uniforms.view * (positioned_pt - cam_posit);

        // Now remove the u coord; replace with 1. We no longer need it,
        // and the projection matrix is set up for 3d homogenous vectors.
        float u = positioned_pt[3];
        positioned_pt = vec4(positioned_pt[0], positioned_pt[1], positioned_pt[2], 1.);

        gl_Position = uniforms.proj * positioned_pt;

        // Now calculate the color, based on passed u dist from cam.
        float u_dist = cam_posit[3] - u;

        float portion_through = abs(u_dist) / uniforms.color_max;

        if (portion_through > 1.) {
            portion_through = 1.;
        }

        float base_gray = 0.0;
        float color_val = base_gray + portion_through * 1. - base_gray;

        vec4 calced_color;
        if (u_dist > 0.) {
            calced_color = vec4(base_gray, base_gray, color_val, 0.2);  // Blue
        } else {
            calced_color = vec4(color_val, base_gray, base_gray, 0.2);  // Red
        }

        v_normal = normal; // todo temp
        fragColor = calced_color;
    }
    "]
        struct Dummy;
}

pub mod fs {
    #[derive(VulkanoShader)]
    #[ty = "fragment"]
    #[src = "
    #version 450
    layout(location = 0) in vec4 fragColor;
    layout(location = 1) in vec4 v_normal;

    layout(location = 0) out vec4 f_color;

    void main() {
        f_color = fragColor;
    }
    "]
        struct Dummy;
}