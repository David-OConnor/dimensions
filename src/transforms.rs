use ndarray::prelude::*;

use types::{Node, Shape, Camera};

pub fn _rotate_4d(theta: &Array1<f64>) -> Array2<f64> {
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // 4d rotation example: http://kennycason.com/posts/2009-01-08-graph4d-rotation4d-project-to-2d.html
    // We rotation around each of six planes.

    // cache trig computations
    // todo fix this
    let cos_xy = theta[0].cos();
    let sin_xy = theta[0].sin();
    let cos_yz = theta[1].cos();
    let sin_yz = theta[1].sin();
    let cos_xz = theta[2].cos();
    let sin_xz = theta[2].sin();
    let cos_xu = theta[3].cos();
    let sin_xu = theta[3].sin();
    let cos_yu = theta[2].cos();
    let sin_yu = theta[2].sin();
    let cos_zu = theta[3].cos();
    let sin_zu = theta[3].sin();

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


// fn _project_4d(cam: &Camera) -> Node {
//     // Project a 4d node onto a 3d space.  Note that to turn into a 2d
//     // projection, we must then apply the 3d projection using this function's
//     // output.
//     // https://en.wikipedia.org/wiki/3D_projection

//     let rotated_shifted_point = _camera_transform_4d(cam, node);

//     let A = array![
//         [1., 0., -cam.e[1] / cam.e[3], 0.],
//         [0., 1., -cam.e[2] / cam.e[3], 0.],
//         [0., 0., 1., 0.],
//         [0., 0., -1. / cam.e[3], 1.],
//     ];

//     let f = A.dot(
//         &array![rotated_shifted_point[1], rotated_shifted_point[2], rotated_shifted_point[3], 1.]
//     );

//     // Keep the original node's id, but transform its position to 2d space.
//     Node {a: array![&f[0] / &f[3], &f[1] / &f[3]], id: node.id}
// }

pub fn rotate_3d(theta: &Array1<f64>) -> Array2<f64> {
    // Compute a 3-dimensional rotation matrix.
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    
    // cache trig computations
    let cos_x = theta[0].cos();
    let sin_x = theta[0].sin();
    let cos_y = theta[1].cos();
    let sin_y = theta[1].sin();
    let cos_z = theta[2].cos();
    let sin_z = theta[2].sin();

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

    // cam.c should remain static, ie at the origin.
    let rotated_shifted_pt = R.dot(
        &array![shifted_pt[0], shifted_pt[1], shifted_pt[2]]
    );

    // https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-
    // projection-matrix/building-basic-perspective-projection-matrix
    let s = 1. / (cam.fov / 2. as f64).tan();

    // near and far clipping planes
    let f = 20.;
    let n = 0.2;

    // todo s in [0, 0] is not negative in the description above... Working around
    // the results I
    let perspective_projection = array![
        [s, 0., 0., 0.],
        [0., s, 0., 0.],
        [0., 0., -f / (f-n), -1.],
        [0., 0., -f*n / (f-n), 0.]
    ];

    let r = 0.04;
    let t = 0.04;

    // http://www.songho.ca/opengl/gl_projectionmatrix_mathml.html
    let perspective_projection2 = array![
        [n / r, 0., 0., 0.],
        [0., n / t, 0., 0.],
        [0., 0., -(f+n) / (f-n), (-2.*f*n) / (f-n)],
        [0., 0., -1., 0.]
    ];

    let homogenous_pt = array![
        rotated_shifted_pt[0], 
        rotated_shifted_pt[1], 
        rotated_shifted_pt[2], 1.
    ];
    let f = perspective_projection.dot(&homogenous_pt);

    // Divide by w to find the 2d projected coords.
    let b = array![&f[0] / &f[3], &f[1] / &f[3]];

    // Keep the original node's id, but transform its position to 2d space.
    Node {a: b, id: node.id}
}

pub fn project_shapes(shapes: &Vec<Shape>, camera: &Camera, R: Array2<f64>, canvas_size: (f64, f64)) -> Vec<Shape> {
    // Project shapes; modify their nodes to be projected on a 2d surface.
    let mut projected_shapes: Vec<Shape> = vec![];
        for shape in shapes.iter() {
            let projected_nodes: Vec<Node> = (&shape.nodes).into_iter()
                .map(|node| project_3d(camera, &R, &node, canvas_size)).collect();

            projected_shapes.push(Shape {
                nodes: projected_nodes,
                edges: shape.edges.clone(),
                id: shape.id
            })
        }
    projected_shapes
}