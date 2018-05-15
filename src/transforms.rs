use ndarray::prelude::*;

use types::{Node, Shape, Camera};

pub fn rotate_4d(θ: &Array1<f64>) -> Array2<f64> {
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // 4d rotation example: http://kennycason.com/posts/2009-01-08-graph4d-rotation4d-project-to-2d.html
    // http://eusebeia.dyndns.org/4d/vis/10-rot-1
    assert![θ.len() == 6];

    // We rotation around each of six planes; the combinations of the 4
    // dimensions. 

    // cache trig computations
    let cos_xy = θ[0].cos();
    let sin_xy = θ[0].sin();
    let cos_yz = θ[1].cos();
    let sin_yz = θ[1].sin();
    let cos_xz = θ[2].cos();
    let sin_xz = θ[2].sin();
    let cos_xu = θ[3].cos();
    let sin_xu = θ[3].sin();
    let cos_yu = θ[4].cos();
    let sin_yu = θ[4].sin();
    let cos_zu = θ[5].cos();
    let sin_zu = θ[5].sin();

    // Potentially there exist 4 hyperrotations as well? ie combinations of 
    // 3 axes ?  xyz  yzu  zux  uxy

    // Rotations around the xy, yz, and xz planes should appear normal.
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

    // Rotations involving u, the fourth dimension, should distort 3d objects.
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

fn project_4d(cam: &Camera, R: &Array2<f64>, node: &Node) -> Node {
    // Project a 4d node onto a 3d space. We'll then need to transform this
    // into a 2d projection for display on the screen, using project_3d.
    // Ref project_3d for more details and links.  Most of this is a simple
    // extension.
    assert![R.rows() == 4 && R.cols() == 4];

    // http://eusebeia.dyndns.org/4d/vis/01-intro

    let translation_matrix = array![
        [1., 0., 0., 0., -cam.position[0]],
        [0., 1., 0., 0., -cam.position[1]],
        [0., 0., 1., 0., -cam.position[2]],
        [0., 0., 0., 1., -cam.position[3]],
        [0., 0., 0., 0., 1.],
    ];
    let shifted_pt = translation_matrix.dot(
        &array![node.a[0], node.a[1], node.a[2], node.a[3], 1.]
    );

    let rotated_shifted_pt = R.dot(
        &array![shifted_pt[0], shifted_pt[1], shifted_pt[2], shifted_pt[3]]
    );

    let s = 1. / (cam.fov / 2. as f64).tan();

    let perspective_projection = array![
        [s, 0., 0., 0., 0.],
        [0., s, 0., 0., 0.],
        [0., 0., s, 0., 0.],
        [0., 0., 0., -cam.f / (cam.f-cam.n), -1.],
        [0., 0., 0., -cam.f*cam.n / (cam.f-cam.n), 0.]
    ];

    let f = perspective_projection.dot(
        &array![rotated_shifted_pt[0], rotated_shifted_pt[1], 
                rotated_shifted_pt[2], rotated_shifted_pt[3], 1.]    
    );

    // Divide by w to find the 2d projected coords.
    let b = array![f[0] / f[4], f[1] / f[4], f[2] / f[4]];

    Node {a: b, id: node.id}
}

pub fn rotate_3d(θ: &Array1<f64>) -> Array2<f64> {
    // Compute a 3-dimensional rotation matrix.
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    assert![θ.len() == 3];

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

fn project_3d(cam: &Camera, R: &Array2<f64>, node: &Node) -> Node {
    // Project a 3d node onto a 2d plane.
    // https://en.wikipedia.org/wiki/3D_projection
    assert![R.rows() == 3 && R.cols() == 3];

    // Perform a camera transform; define a vector rotated_shifted_point as the position
    // of point A with respect to the coordinate system defined by 
    // the camera, with origin in C and rotated by θ with respect
    // to the initial coordinate system.

    // World transform matrix, translation only. Shift first, since we're
    // rotating around the camera as the origin.
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

    let f = perspective_projection.dot(
        &array![rotated_shifted_pt[0], rotated_shifted_pt[1], 
                rotated_shifted_pt[2], 1.]
    );

    // Divide by w to find the 2d projected coords.
    let b = array![f[0] / f[3], f[1] / f[3]];

    // Keep the original node's id, but transform its position to 2d space.
    Node {a: b, id: node.id}
}

pub fn project_shapes_3d(shapes: &[Shape], camera: &Camera,
                         R: &Array2<f64>) -> Vec<Shape> {
    // Project shapes; modify their nodes to be projected on a 2d surface.
    assert![R.rows() == 3 && R.cols() == 3];

    let mut projected_shapes: Vec<Shape> = vec![];

    for shape in shapes.iter() {
        let projected_nodes: Vec<Node> = (&shape.nodes).into_iter()
            .map(|node| project_3d(camera, R, &node)).collect();

        projected_shapes.push(Shape {
            nodes: projected_nodes,
            edges: shape.edges.clone(),
            id: shape.id
        })
    }
    
    projected_shapes
}

pub fn project_shapes_4d(shapes: &[Shape], camera: &Camera, 
                         R_4d: &Array2<f64>, R_3d: &Array2<f64>) -> Vec<Shape> {
    // Project shapes; modify their nodes to be projected on a 2d surface.
    assert![R_4d.rows() == 4 && R_4d.cols() == 4];
    assert![R_3d.rows() == 3 && R_3d.cols() == 3];

    let mut projected_shapes: Vec<Shape> = vec![];

    for shape in shapes.iter() {  
        // Project from 4d space to 3d.
        let mut projected_nodes_3d: Vec<Node> = (&shape.nodes).into_iter()
            .map(|node| project_4d(camera, R_4d, &node)).collect();

        // Now that we've projected the 4d shapes into 3d, project into 2d.
        let projected_nodes_2d: Vec<Node> = (&projected_nodes_3d).into_iter()
            .map(|node| project_3d(camera, R_3d, &node)).collect();
        
        projected_shapes.push(Shape {
            nodes: projected_nodes_2d,
            edges: shape.edges.clone(),
            id: shape.id
        })
    }
    
    projected_shapes
}

#[cfg(test)]
mod tests {
    use super::*;
}
