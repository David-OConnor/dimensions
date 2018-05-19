use std::collections::HashMap;

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
        [cos_xy, sin_xy, 0., 0., 0.],
        [-sin_xy, cos_xy, 0., 0., 0.],
        [0., 0., 1., 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ];

    let R_yz = array![
        [1., 0., 0., 0., 0.],
        [0., cos_yz, sin_yz, 0., 0.],
        [0., -sin_yz, cos_yz, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ];

    let R_xz = array![
        [cos_xz, 0., -sin_xz, 0., 0.],
        [0., 1., 0., 0., 0.],
        [sin_xz, 0., cos_xz, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ];

    // Rotations involving u, the fourth dimension, should distort 3d objects.
    let R_xu = array![
        [cos_xu, 0., 0., sin_xu, 0.],
        [0., 1., 0., 0., 0.],
        [0., 0., 1., 0., 0.],
        [-sin_xu, 0., 0., cos_xu, 0.],
        [0., 0., 0., 0., 1.]
    ];

    let R_yu = array![
        [1., 0., 0., 0., 0.],
        [0., cos_yu, 0., -sin_yu, 0.],
        [0., 0., 1., 0., 0.],
        [0., sin_yu, 0., cos_yu, 0.],
        [0., 0., 0., 0., 1.]
    ];

    let R_zu = array![
        [1., 0., 0., 0., 0.],
        [0., 1., 0., 0., 0.],
        [0., 0., cos_zu, -sin_zu, 0.],
        [0., 0., sin_zu, cos_zu, 0.],
        [0., 0., 0., 0., 1.]
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
    assert![R.rows() == 5 && R.cols() == 5];

    // http://eusebeia.dyndns.org/4d/vis/01-intro

    // todo lots of DRY between this and project_3d.
    let augmented_pt = array![&node.a[0], &node.a[1], &node.a[2], &node.a[3], 1.];

    // Matrices and pts here are all len 5.
    let T = translate(&cam.position);

    // https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-
    // projection-matrix/building-basic-perspective-projection-matrix
    let s_h = 1. / (cam.fov_hor / 2. as f64).tan();
    let s_v = 1. / (cam.fov_vert / 2. as f64).tan();
    let P = array![
        [s_h, 0., 0., 0., 0.],
        [0., s_v, 0., 0., 0.],
        [0., 0., -cam.f / (cam.f-cam.n), -1., 0.],
        [s_h, 0., 0., 0., 0.],  // unused row for u.
        [0., 0., -cam.f*cam.n / (cam.f-cam.n), 0., 0.],
    ];

    // Translate first, since we rotate around the origin. Then rotate.
    // Then project.
    let f = R.dot(&(T.dot(&(P.dot(&augmented_pt)))));

    // Divide by w to find the 2d projected coords.
    Node {a: array![f[0] / f[4], f[1] / f[4], f[2] / f[4]]}
}

pub fn rotate_3d(θ: &Array1<f64>) -> Array2<f64> {
    // Compute a 3-dimensional rotation matrix.
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // We return 5x5 matrices for compatibility with other transforms, and to
    // reduce repetition between 4d and 3d vectors.
    assert_eq![θ.len(), 3];

    // cache trig computations
    let cos_x = θ[0].cos();
    let sin_x = θ[0].sin();
    let cos_y = θ[1].cos();
    let sin_y = θ[1].sin();
    let cos_z = θ[2].cos();
    let sin_z = θ[2].sin();

    // R matrices rotate a vector around a single axis.
    let R_x = array![
        [1., 0., 0., 0., 0.],
        [0., cos_x, -sin_x, 0., 0.],
        [0., sin_x, cos_x, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]

    ];

    let R_y = array![
        [cos_y, 0., sin_y, 0., 0.],
        [0., 1., 0., 0., 0.],
        [-sin_y, 0., cos_y, 0., 0.]
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ];

    let R_z = array![  // Official OpenGl docs list this as reversed -sins.
        [cos_z, sin_z, 0., 0., 0.],
        [-sin_z, cos_z, 0., 0., 0.],
        [0., 0., 1., 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ];

    // Combine the three rotations.
    R_x.dot(&(R_y.dot(&R_z)))
}

fn translate(cam_position: &Array1<f64>) -> Array2<f64> {
    // Return a translation matrix; the pt must have 1 appended to its end.
    // We do this augmentation so we can add a constant term.  Scale and
    // rotation matrices may have this as well for matrix compatibility.
    assert_eq![cam_position.len, 4];

    array![
        [1., 0., 0., 0., cam_position[0]],
        [0., 1., 0., 0., cam_position[1]],
        [0., 0., 1., 0., cam_position[2]],
        [0., 0., 0., 1., cam_position[3]],
        [0., 0., 0., 0., 1.],
    ]
}

fn scale(scale: Array1<f64>) -> Array2<f64> {
    // Return a scale matrix; the pt must have 1 appended to its end.
    assert_eq![scale.len, 4];

    array![
        [scale[0], 0., 0., 0., 0.],
        [0., scale[1]., 0., 0., 0.],
        [0., 0., scale[2]., 0., 0.],
        [0., 0., 0., scale[3]., 0.],
        [0., 0., 0., 0., 1.],
    ]
}

fn project_3d(cam: &Camera, R: &Array2<f64>, node: &Node) -> Node {
    // Project a 3d node onto a 2d plane.
    // https://en.wikipedia.org/wiki/3D_projection
    assert![R.rows() == 5 && R.cols() == 5];

    // Perform a camera transform; define a vector rotated_shifted_point as the position
    // of point A with respect to the coordinate system defined by 
    // the camera, with origin in C and rotated by θ with respect
    // to the initial coordinate system.

    // Matrices and pts here are all len 5.
    let augmented_pt = array![&node.a[0], &node.a[1], &node.a[2], &node.a[3], 1.];

    let T = translate(&cam.position);

    // https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-
    // projection-matrix/building-basic-perspective-projection-matrix
    let s_h = 1. / (cam.fov_hor / 2. as f64).tan();
    let s_v = 1. / (cam.fov_vert / 2. as f64).tan();
    let P = array![
        [s_h, 0., 0., 0., 0.],
        [0., s_v, 0., 0., 0.],
        [0., 0., -cam.f / (cam.f-cam.n), -1., 0.],
        [s_h, 0., 0., 0., 0.],  // unused row for u.
        [0., 0., -cam.f*cam.n / (cam.f-cam.n), 0., 0.],
    ];

    // Translate first, since we rotate around the origin. Then rotate.
    // Then project.
    let f = R.dot(&(T.dot(&(P.dot(&augmented_pt)))));


    // Divide by w to find the 2d projected coords.
    Node {a: array![f[0] / f[3], f[1] / f[3]]}
}

fn position_shape(shape: &Shape, is_4d: bool) -> HashMap<i32, Node> {
    // Position a shape's nodes in 3 or 4d space, based on its position
    // and rotation parameters.

    // T must be done last, since we scale and rotate with respect to the orgin,
    // defined in the shape's initial nodes.
    let R = match is_4d {
        true => rotate_4d(&shape.rotation),
        false => rotate_3d(&shape.rotation),
    };
    let S = scale(array![shape.scale, shape.scale, shape.scale, shape.scale]);
    let T = translate(&shape.position);

    let mut positioned_nodes = HashMap::new();
    for (id, node) in &shape.nodes {
        let augmented_pt = array![&node.a[0], &node.a[1], &node.a[2], &node.a[3], 1.];
        let new_pt = T.dot(&(S.dot(&(R.dot(&augmented_pt)))));
        positioned_nodes.insert(id, Node {a: new_pt});
    }

    positioned_nodes
}

pub fn project_shapes_3d(shapes: &HashMap<i32, Shape>, camera: &Camera,
                         R: &Array2<f64>) -> HashMap<(i32, i32), Node> {
    // Project shapes; modify their nodes to be projected on a 2d surface.
    // The HashMap key is (shape_index, node_index), so we can tie back to the
    // original shapes later.
    assert![R.rows() == 5 && R.cols() == 5];

    let mut projected_nodes = HashMap::new();

    for (shape_id, shape) in shapes {
        for (node_id, node) in &shape.nodes {
            projected_nodes.insert((*shape_id, *node_id), project_3d(camera, R, &node));
        }
    }
    
    projected_nodes
}

pub fn project_shapes_4d(shapes: &HashMap<i32, Shape>, camera: &Camera,
                         R_4d: &Array2<f64>, R_3d: &Array2<f64>) -> HashMap<(i32, i32), Node> {
    // Project shapes; modify their nodes to be projected on a 2d surface.
    assert![R_4d.rows() == 4 && R_4d.cols() == 4];
    assert![R_3d.rows() == 3 && R_3d.cols() == 3];

    // First project from 4d to 3d
    let mut projected_nodes_3d = HashMap::new();

    // todo DRY between here and project_shapes_3d.
    for (shape_id, shape) in shapes {
        for (node_id, node) in &shape.nodes {
            projected_nodes.insert((*shape_id, *node_id), project_4d(camera, R_4d, &node));
        }
    }

    // Now project from 3d to 2d.
    project_shapes_3d(&projected_nodes_3d, camera, R_3d)
}

#[cfg(test)]
mod tests {
    use super::*;
}
