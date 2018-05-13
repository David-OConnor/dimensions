use ndarray::prelude::*;

use types::{Node, Shape, Camera};

pub fn _rotate_4d(θ: &Array1<f64>) -> Array2<f64> {
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // 4d rotation example: http://kennycason.com/posts/2009-01-08-graph4d-rotation4d-project-to-2d.html
    // We rotation around each of six planes.

    // cache trig computations
    // todo fix this
    let cos_xy = θ[0].cos();
    let sin_xy = θ[0].sin();
    let cos_yz = θ[1].cos();
    let sin_yz = θ[1].sin();
    let cos_xz = θ[2].cos();
    let sin_xz = θ[2].sin();
    let cos_xu = θ[3].cos();
    let sin_xu = θ[3].sin();
    let cos_yu = θ[2].cos();
    let sin_yu = θ[2].sin();
    let cos_zu = θ[3].cos();
    let sin_zu = θ[3].sin();

    // R_axis1axis2 matrices rotate a vector around a plane
    // There may be a second approach to this that rotates around each xis
    // rather than planes.

    let R_xy = array![
        [cos_xy, sin_xy, 0., 0.],
        [-sin_xy, cos_xy, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.]
    ];

    let R_yz = array![
        [1., 0., 0., 0.],
        [0., cos_yz, sin_yz, 0.],
        [0., -sin_yz, cos_yz, 0.],
        [0., 0., 0., 1.]
    ];

    let R_xz = array![
        [cos_xz, 0., -sin_xz, 0.],
        [0., 1., 0., 0.], 
        [sin_xz, 0., cos_xz, 0.],
        [0., 0., 0., 1.]
    ];

    let R_xu = array![
        [cos_xu, 0., 0., sin_xu],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [-sin_xu, 0., 0., cos_xu]
    ];

    let R_yu = array![
        [1., 0., 0., 0.],
        [0., cos_yu, 0., -sin_yu],
        [0., 0., 1., 0.],
        [0., sin_yu, 0., cos_yu]
    ];

    let R_zu = array![
        [1., 0., 0., 0.],
        [0., 1., 0., 0.], 
        [0., 0., cos_zu, -sin_zu],
        [0., 0., sin_zu, cos_zu]
    ];

    // Combine the rotations.
    let R_1 = R_xy.dot(&(R_yz.dot(&R_xz)));
    let R_2 = R_xu.dot(&(R_yu.dot(&R_zu)));
    R_1.dot(&R_2)
}

// fn project_4d(cam: &Camera, R: &Array2<f64>, node: &Node, canvas_size: (f64, f64)) -> Node {
//     // Project a 4d node onto a 2d space.
//     // https://en.wikipedia.org/wiki/3D_projection

//     // Translating the point is a simple extension of the 2d translation.
//     let translation_matrix = array![
//         [1., 0., 0., 0., -cam.position[0]],
//         [0., 1., 0., 0., -cam.position[1]],
//         [0., 0., 1., 0., -cam.position[2]],
//         [0., 0., 0., 1., -cam.position[3]],
//         [0., 0., 0., 0., 1.],
//     ];
//     let shifted_pt = translation_matrix.dot(
//         &array![node.a[0], node.a[1], node.a[2], node.a[3], 1.]
//     );

//     let rotated_shifted_pt = R.dot(
//         &array![shifted_pt[0], shifted_pt[1], shifted_pt[2], shifted_pt[3]]
//     );

//     node
// }

pub fn rotate_3d(θ: &Array1<f64>) -> Array2<f64> {
    // Compute a 3-dimensional rotation matrix.
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    
    // cache trig computations
    let cos_x = θ[0].cos();
    let sin_x = θ[0].sin();
    let cos_y = θ[1].cos();
    let sin_y = θ[1].sin();
    let cos_z = θ[2].cos();
    let sin_z = θ[2].sin();

    // R matrices rotate a vector around a single axis.
    let R_x = array![
        [1., 0., 0.],
        [0., cos_x, sin_x],
        [0., -sin_x, cos_x],
    ];

    let R_y = array![
        [cos_y, 0., -sin_y],
        [0., 1., 0.],
        [sin_y, 0., cos_y]
    ];

    let R_z = array![
        [cos_z, sin_z, 0.],
        [-sin_z, cos_z, 0.],
        [0., 0., 1.]
    ];

    // Combine the three rotations.
    R_x.dot(&(R_y.dot(&R_z)))
}

fn project_3d(cam: &Camera, R: &Array2<f64>, node: &Node, canvas_size: (f64, f64)) -> Node {
    // Project a 3d node onto a 2d plane.
    // https://en.wikipedia.org/wiki/3D_projection

    // Perform a camera transform; define a vector rotated_shifted_point as the position
    // of point A with respect to the coordinate system defined by 
    // the camera, with origin in C and rotated by θ with respect
    // to the initial coordinate system.

    // World transform matrix, translation only.
    let translation_matrix = array![
        [1., 0., 0., -cam.position[0]],
        [0., 1., 0., -cam.position[1]],
        [0., 0., 1., -cam.position[2]],
        [0., 0., 0., 1.],
    ];
    let shifted_pt = translation_matrix.dot(
        &array![node.a[0], node.a[1], node.a[2], 1.]
    );

    let rotated_shifted_pt = R.dot(
        &array![shifted_pt[0], shifted_pt[1], shifted_pt[2]]
    );

    // https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-
    // projection-matrix/building-basic-perspective-projection-matrix
    let s = 1. / (cam.fov / 2. as f64).tan();

    let perspective_projection = array![
        [s, 0., 0., 0.],
        [0., s, 0., 0.],
        [0., 0., -cam.f / (cam.f-cam.n), -1.],
        [0., 0., -cam.f*cam.n / (cam.f-cam.n), 0.]
    ];

    // let r = 0.04;
    // let t = 0.04;

    // // http://www.songho.ca/opengl/gl_projectionmatrix_mathml.html
    // let perspective_projection2 = array![
    //     [n / r, 0., 0., 0.],
    //     [0., n / t, 0., 0.],
    //     [0., 0., -(f+n) / (f-n), (-2.*f*n) / (f-n)],
    //     [0., 0., -1., 0.]
    // ];

    let homogenous_pt = array![
        rotated_shifted_pt[0], 
        rotated_shifted_pt[1], 
        rotated_shifted_pt[2], 
        1.
    ];
    let f = perspective_projection.dot(&homogenous_pt);

    // Divide by w to find the 2d projected coords.
    let b = array![f[0] / f[3], f[1] / f[3]];

    // Keep the original node's id, but transform its position to 2d space.
    Node {a: b, id: node.id}
}

pub mod clipping {
    // Functions for clipping; Cohen-Sutherland algorithm, from:
    // https://en.wikipedia.org/wiki/Cohen%E2%80%93Sutherland_algorithm

    // This algorithm clips post-projection, ie on the 2d screen.

    use types::Pt2D;

    const INSIDE: i8 = 0;
    const LEFT: i8 = 1;
    const RIGHT: i8 = 2;
    const BOTTOM: i8 = 4;
    const TOP: i8 = 8;
   
    fn compute_outcode(pt: &Pt2D, x_min: f64, x_max: f64, y_min: f64, y_max: f64) -> i8 {
        // Initialised as being inside of clip window
        let mut code = INSIDE;

        if pt.x < x_min {
            code |= LEFT;
        }
        else if pt.x > x_max {
            code |= RIGHT;
        }
        if pt.y < y_min {
            code |= BOTTOM;
        }
        else if pt.y > y_max {
            code |= TOP;
        }
        code
    }

    pub fn clip(pt_0: &Pt2D, pt_1: &Pt2D, x_min: f64, x_max: f64, 
                y_min: f64, y_max: f64) -> (Pt2D, Pt2D) {
        // Cohen–Sutherland clipping algorithm clips a line from
        // P0 = (x0, y0) to P1 = (x1, y1) against a rectangle with 
        // diagonal from (xmin, ymin) to (xmax, ymax).
        
        let mut outcode_0 = compute_outcode(&pt_0, x_min, x_max, y_min, y_max);
        let mut outcode_1 = compute_outcode(&pt_1, x_min, x_max, y_min, y_max);

        let mut accept = false;
        let x_0: f64;
        let y_0: f64;
        let x_1: f64;
        let y_1: f64;
        let mut x: f64;
        let mut y: f64;
        
        let mut x_0 = pt_0.x;
        let mut y_0 = pt_0.y;
        let mut x_1 = pt_1.x;
        let mut y_1 = pt_1.y;
        
        loop {
            if outcode_0 | outcode_1 <= 0 {
                // bitwise OR is 0: both points inside window; trivially accept and exit loop
                accept = true;
                break;
            } else if outcode_0 & outcode_1 > 0 {
                // bitwise AND is not 0: both points share an outside zone (LEFT, RIGHT, TOP,
			    // or BOTTOM), so both must be outside window; exit loop (accept is false)
                break;
            } else {
                // At least one endpoint is outside the clip rectangle; pick it.
                let outcode_out = if outcode_0 > 0 { outcode_0 } else { outcode_1 };
            
                // Now find the intersection point;
                // use formulas:
                //   slope = (y1 - y0) / (x1 - x0)
                //   x = x0 + (1 / slope) * (ym - y0), where ym is ymin or ymax
                //   y = y0 + slope * (xm - x0), where xm is xmin or xmax
                // No need to worry about divide-by-zero because, in each case, the
                // outcode bit being tested guarantees the denominator is non-zero
                if outcode_out & TOP > 0 { 
                    // point is above the clip window
                    x = x_0 + (x_1 - x_0) * (y_max - y_0) / (y_1 - y_0); 
                    y = y_max;
                }
                else if outcode_out & BOTTOM > 0 { 
                    // point is below the clip window
                    x = x_0 + (x_1 - x_0) * (y_min - y_0) / (y_1 - y_0); 
                    y = y_min;
                }
                else if outcode_out & RIGHT > 0 { 
                    // point is to the right of the clip window
                    y = y_0 + (y_1 - y_0) * (x_max - x_0) / (x_1 - x_0); 
                    x = x_max;
                }
                else {
                    // point is to the left of the clip window
                    y = y_0 + (y_1 - y_0) * (x_min - x_0) / (x_1 - x_0); 
                    x = x_min;
                }

                // Now we move outside point to intersection point to clip
			    // and get ready for next pass.
                if outcode_out == outcode_0 {
                    x_0 = x;
                    y_0 = y;
                    outcode_0 = compute_outcode(&Pt2D {x: x_0, y: y_0}, x_min, x_max, y_min, y_max);
                } else {
                    x_1 = x;
                    y_1 = y;
                    outcode_1 = compute_outcode(&Pt2D {x: x_1, y: y_1}, x_min, x_max, y_min, y_max);
                }
            }
        }

        // If we've accepted, return the full line.  If rejected, return the
        // clipped line. 
        if accept {
            (Pt2D {x: pt_0.x, y: pt_0.y}, Pt2D {x: pt_1.x, y: pt_0.y})
        }
        else {
            (Pt2D {x: x_0, y: y_0}, Pt2D {x: x_1, y: y_1})
        }
    }
}

pub fn project_shapes(shapes: &[Shape], camera: &Camera, R: &Array2<f64>, canvas_size: (f64, f64)) -> Vec<Shape> {
    // Project shapes; modify their nodes to be projected on a 2d surface.
    let mut projected_shapes: Vec<Shape> = vec![];
        for shape in shapes.iter() {
            let projected_nodes: Vec<Node> = (&shape.nodes).into_iter()
                .map(|node| project_3d(camera, R, &node, canvas_size)).collect();

            projected_shapes.push(Shape {
                nodes: projected_nodes,
                edges: shape.edges.clone(),
                id: shape.id
            })
        }
    projected_shapes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clip_1() {
        let pt_0 = (32., -11.);
        let pt_1 = (0., -1.2);

        let expected = 0;

        assert_eq!(clipping::clip_and_draw(pt_0[0], pt_0[1], pt_1[0], pt_1[1]), expected);
    }

        #[test]
    fn outcodes() {
        let pt_0 = (32., -11.);
        let pt_1 = (0., -1.2);

        let expected = 0;

        assert_eq!(clipping::clip_and_draw(pt_0[0], pt_0[1], pt_1[0], pt_1[1]), expected);
    }
}
