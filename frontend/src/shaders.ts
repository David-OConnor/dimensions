// Vertex shader program
export const vsSource = `
    attribute vec4 a_position;
    attribute vec4 a_normal;
                   
    // We can't pass 5x5 homogenous matrices to the shader, but can pass 4x4,
    // non-homogenous matrices, then translate separately.
    uniform mat4 u_model;
    uniform mat4 u_view;
    uniform mat4 u_proj;
    
    uniform vec4 u_shape_position;
    uniform vec4 u_cam_position;
    uniform vec4 u_ambient_light_color;
    uniform vec4 u_diffuse_light_color;
    uniform vec4 u_diffuse_light_direction;
    
    uniform float u_ambient_intensity;
    uniform float u_specular_intensity;
    uniform float u_color_max;
           
    varying vec4 v_fourd_color;
    varying vec4 v_diffuse;
    varying vec4 v_specular;

    void main() {
        // For model transform, position after the transform
        vec4 positionedPt = (u_model * a_position) + u_shape_position;
        // for view transform, position first.
        positionedPt = u_view * (positionedPt - u_cam_position);
        
        // Now remove the u coord; replace with 1. We no longer need it, 
        // and the projection matrix is set up for 3d homogenous vectors.
        vec4 positioned3d = vec4(positionedPt[0], positionedPt[1], positionedPt[2], 1.);
        
        gl_Position = u_proj * positioned3d;
      
      
        // Now calculate the color, based on passed u dist from cam.
        float u_dist = u_cam_position[3] - positionedPt[3];
        
        float portion_through = abs(u_dist) / u_color_max;

        if (portion_through > 1.) {
            portion_through = 1.;
        }
        
        float base_gray = 0.0;
        float color_val = base_gray + portion_through * 1. - base_gray;
        
        vec4 f_color;
        if (u_dist > 0.) {
            f_color = vec4(base_gray, base_gray, color_val, 0.2);  // Blue
        } else {
            f_color = vec4(color_val, base_gray, base_gray, 0.2);  // Red
        }
        
        vec4 v_fourd_color = f_color * u_ambient_intensity;
        
        // Process diffuse lighting from a single-directional source.
        // We can use the model matrix directly on the normal, since it
        // only scales uniformly, and isn't homogenous (doesn't translate).
        vec4 norm = normalize(u_model * a_normal);
        vec4 dir = normalize(u_diffuse_light_direction);
        float directional_light_weight = max(dot(norm, dir), 0.);

        v_diffuse = u_diffuse_light_color * directional_light_weight;

        v_specular = vec4(0., 0., 0., 0.);  // todo

//         // Now calculate specular lighting.
//         // todo deal with view trasnforms.
//         vec4 view_dir = normalize(u_cam_position - a_position);
// //        vec4 view_dir = normalize(uview * (u_cam_position - a_position));
//         vec4 reflect_dir = reflect(-dir, norm);
// //        vec4 reflect_dir = u_view * reflect(-dir, norm);
//         float spec = pow(max(dot(view_dir, reflect_dir), 0.), 32.);
//         v_specular = u_specular_intensity * spec * u_diffuse_light_color;
    }
`

// Fragment shader program
export const fsSource = `
    varying highp vec4 v_fourd_color;
    varying highp vec4 v_diffuse;
    varying highp vec4 v_specular;

    void main() {
        gl_FragColor = v_fourd_color + v_diffuse;
    }
`

export const vsSkybox = `
    attribute vec4 a_position;
    attribute vec2 a_texcoord;
     
    uniform mat4 u_matrix;
     
    varying vec2 v_texcoord;
     
    void main() {
      // Multiply the position by the matrix.
      gl_Position = u_matrix * a_position;
     
      // Pass the texcoord to the fragment shader.
      v_texcoord = a_texcoord;
    }
`

export const fsSkybox = `
        precision mediump float;
         
        // Passed in from the vertex shader.
        varying vec2 v_texcoord;
         
        // The texture.
        uniform sampler2D u_texture;
         
        void main() {
           gl_FragColor = texture2D(u_texture, v_texcoord);
        }
    `