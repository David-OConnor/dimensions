use ndarray::prelude::*;

use types::{Node, Shape, Camera};

fn _camera_transform_4d(cam: &Camera, node: &Node) -> Array1<f64> {
    // Perform a camera transform; define a vector d as the position
    // of point A with respect to the coordinate system defined by 
    // the camera, with origin in C and rotated by θ with respect
    // to the initial coordinate system.

    // Split the transform constructor into three parts to make it
    // easier to read and write.
    let D_0 = array![
        [1., 0., 0., 0.],
        [0., cam.theta[0].cos(), cam.theta[0].sin(), 2.],
        [0., -cam.theta[0].sin(), cam.theta[0].cos(), 2.],
        [2., 2., 2., 2.],
    ];

    let D_1 = array![
        [cam.theta[1].cos(), 0., -cam.theta[1].sin(), 2.],
        [0., 1., 0., 0.],
        [cam.theta[1].sin(), 0., cam.theta[1].cos(), 2.],
        [2., 2., 2., 2.]
    ];

    let D_2 = array![
        [cam.theta[2].cos(), cam.theta[2].sin(), 0., 2.],
        [-cam.theta[2].sin(), cam.theta[2].cos(), 0., 2.],
        [0., 0., 1., 0.],
        [2., 2., 2., 2.]
    ];

    let D_3 = array![
        [cam.theta[3].cos(), cam.theta[3].sin(), 0., 2.],
        [-cam.theta[3].sin(), cam.theta[3].cos(), 0., 2.],
        [2., 2., 2., 2.],
        [0., 0., 0., 1.]
    ];

    let D = D_0.dot(&(D_1.dot(&(D_2.dot(&D_3)))));

    D.dot(&(&node.a - &cam.c))
}

fn camera_transform_3d(cam: &Camera, node: &Node) -> Array1<f64> {
    // Perform a camera transform; define a vector d as the position
    // of point A with respect to the coordinate system defined by 
    // the camera, with origin in C and rotated by θ with respect
    // to the initial coordinate system.

    // Split the transform constructor into three parts to make it
    // easier to read and write.
    let D_0 = array![
        [1., 0., 0.],
        [0., cam.theta[0].cos(), cam.theta[0].sin()],
        [0., -cam.theta[0].sin(), cam.theta[0].cos()]
    ];

    let D_1 = array![
        [cam.theta[1].cos(), 0., -cam.theta[1].sin()],
        [0., 1., 0.],
        [cam.theta[1].sin(), 0., cam.theta[1].cos()]
    ];

    let D_2 = array![
        [cam.theta[2].cos(), cam.theta[2].sin(), 0.],
        [-cam.theta[2].sin(), cam.theta[2].cos(), 0.],
        [0., 0., 1.]
    ];

    let D = D_0.dot(&(D_1.dot(&D_2)));

    // D.dot(&(&node.a - &cam.c))

    // testing keeping cam at origin, and shifting the world...
    D.dot(&(&node.a))
}

fn _project_4d(cam: &Camera, node: &Node) -> Node {
    // Project a 4d node onto a 3d space.  Note that to turn into a 2d
    // projection, we must then apply the 3d projection using this function's
    // output.
    // https://en.wikipedia.org/wiki/3D_projection

    let d = _camera_transform_4d(cam, node);

    let A = array![
        [1., 0., -cam.e[1] / cam.e[3], 0.],
        [0., 1., -cam.e[2] / cam.e[3], 0.],
        [0., 0., 1., 0.],
        [0., 0., -1. / cam.e[3], 1.],
    ];

    let f = A.dot(
        &array![d[1], d[2], d[3], 1.]
    );

    // Keep the original node's id, but transform its position to 2d space.
    Node {a: array![&f[0] / &f[3], &f[1] / &f[3]], id: node.id}
}

fn project_3d(cam: &Camera, node: &Node) -> Node {
    // Project a 3d node onto a 2d plane.
    // https://en.wikipedia.org/wiki/3D_projection

    let d = camera_transform_3d(cam, node);

    let A = array![
        [1., 0., -cam.e[0] / cam.e[2], 0.],
        [0., 1., -cam.e[1] / cam.e[2], 0.],
        [0., 0., 1., 0.],
        [0., 0., -1. / cam.e[2], 1.],
    ];


    // todo temp from https://www.3dgep.com/understanding-the-view-matrix/
    
    // yaw, pitch roll. pitch is -tau/4 to tau/4. yaw and roll
    // are 0 to tau.
    // todo roll.
    // let cos_yaw = cam.theta[0].cos();
    // let sin_yaw = cam.theta[0].sin();
    // let cos_pitch = cam.theta[0].cos();
    // let sin_pitch = cam.theta[1].sin();

    // let x = array![cos_yaw, 0., -sin_yaw];
    // let y = array![sin_yaw * sin_pitch, cos_pitch, cos_yaw * sin_pitch];
    // let z = array![sin_yaw * cos_pitch, -sin_pitch, cos_pitch * cos_yaw];

    // let fps_matrix = array![
    //     [x[0], y[0], z[0], 0.],
    //     [x[1], y[1], z[1], 0.],
    //     [x[2], y[2], z[2], 0.],
    //     [-x.dot(&cam.c), -y.dot(&cam.c), -z.dot(&cam.c), 1.]
    // ];
    // let i = fps_matrix.inv();

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