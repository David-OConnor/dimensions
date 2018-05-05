use ndarray::prelude::*;

use types::{Node, Shape, Camera};

// fn rotate_4d(cam: &Camera, node: &Node) -> Array4<f64> {
//     // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
//     // 4d rotation example: http://kennycason.com/posts/2009-01-08-graph4d-rotation4d-project-to-2d.html
   
//     // cache trig computations
//     let cos_x = cam.theta[0].cos();
//     let sin_x = cam.theta[0].sin();
//     let cos_y = cam.theta[1].cos();
//     let sin_y = cam.theta[1].sin();
//     let cos_z = cam.theta[2].cos();
//     let sin_z = cam.theta[2].sin();
//     let sin_u = cam.theta[3].sin();
//     let sin_u = cam.theta[3].sin();

//     // R_axis1axis2 matrices rotate a vector around a plane
//     // There may be a second approach to this that rotates around each xis
//     // rather than planes.

//     // TODO which thetas??
//     let R_xy = &array![
//         [cos_x, sin_x, 0., 0.],
//         [-sin_x, cos_x, 0., 0.],
//         [0., 0., 1., 0.],
//         [0., 0., 0., 1.]
//     ];

//     let R_yz = &array![
//         [1., 0., 0., 0.],
//         [0., cos_y, sin_y, 0.],
//         [0., -sin_y, cos_y, 0.],
//         [0., 0., 0., 1.]
//     ];

//     let xz = &array![
//         [cos_z, sin_z, 0.],
//         [-sin_z, cos_z, 0.],
//         [0., 0., 1.]
//     ];

//     // Combine the rotations.
//     R_xy.dot(R_yz.dot(R_xz.dot(R_xu.dot(R_yu.dot(R_zu)))))
// }


// fn _project_4d(cam: &Camera) -> Node {
//     // Project a 4d node onto a 3d space.  Note that to turn into a 2d
//     // projection, we must then apply the 3d projection using this function's
//     // output.
//     // https://en.wikipedia.org/wiki/3D_projection

//     let d = _camera_transform_4d(cam, node);

//     let A = array![
//         [1., 0., -cam.e[1] / cam.e[3], 0.],
//         [0., 1., -cam.e[2] / cam.e[3], 0.],
//         [0., 0., 1., 0.],
//         [0., 0., -1. / cam.e[3], 1.],
//     ];

//     let f = A.dot(
//         &array![d[1], d[2], d[3], 1.]
//     );

//     // Keep the original node's id, but transform its position to 2d space.
//     Node {a: array![&f[0] / &f[3], &f[1] / &f[3]], id: node.id}
// }

pub fn rotate_3d(cam: &Camera) -> Array2<f64> {
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
   
    // cache trig computations
    let cos_x = cam.theta[0].cos();
    let sin_x = cam.theta[0].sin();
    let cos_y = cam.theta[1].cos();
    let sin_y = cam.theta[1].sin();
    let cos_z = cam.theta[2].cos();
    let sin_z = cam.theta[2].sin();

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

fn project_3d(cam: &Camera, node: &Node) -> Node {
    // Project a 3d node onto a 2d plane.
    // https://en.wikipedia.org/wiki/3D_projection

    // Perform a camera transform; define a vector d as the position
    // of point A with respect to the coordinate system defined by 
    // the camera, with origin in C and rotated by θ with respect
    // to the initial coordinate system.

    // todo could use a matrix for this 'world transform', especially if we allow
    // shapes to be rotated.
    let shifted_node = Node {
        a: array![
            node.a[0] - cam.position[0],
            node.a[1] - cam.position[1],
            node.a[2] - cam.position[2],
        ],
        id: node.id
    };

    let R = rotate_3d(cam);

    // let view_offset = R.dot(&cam.e);
    
    let d = R.dot(&(&shifted_node.a - &cam.c));

    let A = array![
        [1., 0., -cam.e[0] / cam.e[2], 0.],
        [0., 1., -cam.e[1] / cam.e[2], 0.],
        [0., 0., 1., 0.],
        [0., 0., -1. / cam.e[2], 1.],
    ];

    let f = A.dot(
        &array![d[0], d[1], d[2], 1.]
    );

    // Keep the original node's id, but transform its position to 2d space.
    Node {a: array![&f[0] / &f[3], &f[1] / &f[3]], id: node.id}
}

pub fn project_shapes(shapes: &Vec<Shape>, camera: &Camera) -> Vec<Shape> {
    // Project shapes; modify their nodes to be projected on a 2d surface.
    let mut projected_shapes: Vec<Shape> = vec![];
        for shape in shapes.iter() {
            let projected_nodes: Vec<Node> = (&shape.nodes).into_iter()
                .map(|node| project_3d(camera, &node)).collect();

            projected_shapes.push(Shape {
                nodes: projected_nodes,
                edges: shape.edges.clone(),
                id: shape.id
            })
        }
    projected_shapes
}