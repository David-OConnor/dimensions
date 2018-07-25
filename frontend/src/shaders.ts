// Vertex shader program
// Similar to vert.glsl and frag.glsl. Look there for comments.

export const vsSource = `
    attribute vec4 position;
    attribute vec4 normal;
                   
    // We can't pass 5x5 homogenous matrices to the shader, but can pass 4x4,
    // non-homogenous matrices, then translate separately.
    uniform mat4 u_model;
    uniform mat4 u_view;
    uniform mat4 u_proj;
    
    uniform vec4 u_shape_position;
    uniform vec4 u_cam_position;
    uniform vec4 u_ambient_color;
    uniform vec4 u_diffuse_color;
    uniform vec4 u_diffuse_direction;
    
    uniform float u_ambient_intensity;
    uniform float u_diffuse_intensity; 
    
    uniform float u_color_max;
    uniform float u_shape_opacity;
           
    varying vec4 v_color;
    
    vec4 find_fourd_color(vec4 positioned_pt) {
        float u_dist = u_cam_position[3] - positioned_pt[3];

        float portion_through = abs(u_dist) / u_color_max;

        if (portion_through > 1.) {
            portion_through = 1.;
        }

        float base_gray = 0.0;
        float color_val = base_gray + portion_through;
        vec4 fourd_color;

        if (u_dist > 0.) {
            fourd_color = vec4(base_gray, base_gray, color_val, u_shape_opacity);  // Blue
        } else {
            fourd_color = vec4(color_val, base_gray, base_gray, u_shape_opacity);  // Red
        }
        return fourd_color * u_ambient_intensity;
    }

    vec4 find_diffuse_color() {
        vec4 norm = normalize(u_model * normalize(normal));
        vec4 dir = normalize(u_diffuse_direction);
        float diffuse_weight = max(dot(norm, dir), 0.);
        return u_diffuse_color * diffuse_weight * u_diffuse_intensity;
    }

    void main() {
        vec4 positioned_pt = (u_model * position) + u_shape_position;
        positioned_pt = u_view * (positioned_pt - u_cam_position);
        
        gl_Position = u_proj * positioned_pt;

        vec4 fourd_color = find_fourd_color(positioned_pt);
        vec4 diffuse_color = find_diffuse_color();

        v_color = mix(fourd_color, diffuse_color, 0.5);
    }
`

// Fragment shader program
export const fsSource = `
    varying highp vec4 v_color;

    void main() {
        // gl_FragColor is a special name for GLSL ES 1.0
        gl_FragColor = v_color;
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