use std::collections::HashMap;

use ndarray::prelude::*;

use clipping;
use types::{Camera, Node, Shape};

pub fn make_rotator_4d(θ: &Array1<f64>) -> Array2<f64> {
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // 4d rotation example: http://kennycason.com/posts/2009-01-08-graph4d-rotation4d-project-to-2d.html
    // http://eusebeia.dyndns.org/4d/vis/10-rot-1
    assert_eq![θ.len(), 6];

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

pub fn make_rotator_3d(θ: &Array1<f64>) -> Array2<f64> {
    // Compute a 3-dimensional rotation matrix.
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // We return 5x5 matrices for compatibility with other transforms, and to
    // reduce repetition between 4d and 3d vectors.

    // Note that we might accept a rotation vector of len 6, but only the
    // first 3 values will be used.

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
        [-sin_y, 0., cos_y, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ];

    let R_z = array![
        [cos_z, -sin_z, 0., 0., 0.],
        [sin_z, cos_z, 0., 0., 0.],
        [0., 0., 1., 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ];

    // Combine the three rotations.
    R_x.dot(&(R_y.dot(&R_z)))
}

pub fn make_translator(cam_position: &Array1<f64>) -> Array2<f64> {
    // Return a translation matrix; the pt must have 1 appended to its end.
    // We do this augmentation so we can add a constant term.  Scale and
    // rotation matrices may have this as well for matrix compatibility.
    assert_eq![cam_position.len(), 4];

    array![
        [1., 0., 0., 0., cam_position[0]],
        [0., 1., 0., 0., cam_position[1]],
        [0., 0., 1., 0., cam_position[2]],
        [0., 0., 0., 1., cam_position[3]],
        [0., 0., 0., 0., 1.],
    ]
}

fn make_scaler(scale: Array1<f64>) -> Array2<f64> {
    // Return a scale matrix; the pt must have 1 appended to its end.
    assert_eq![scale.len(), 4];

    array![
        [scale[0], 0., 0., 0., 0.],
        [0., scale[1], 0., 0., 0.],
        [0., 0., scale[2], 0., 0.],
        [0., 0., 0., scale[3], 0.],
        [0., 0., 0., 0., 1.],
    ]
}

pub fn make_projector(cam: &Camera, is_4d: bool) -> Array2<f64> {
    // Create the projection matrix, used to transform translated and
    // rotated points.

    // https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-
    // projection-matrix/building-basic-perspective-projection-matrix
    let s_h = 1. / (cam.fov_hor / 2. as f64).tan();
    let s_v = 1. / (cam.fov_vert / 2. as f64).tan();
    let s_d = s_h;  // depth for 4d?

    // Is the discarded projection axis z (like 3d), or u?
    // Currently set for u.

    match is_4d {
        true => array![
            [s_h, 0., 0., 0., 0.],
            [0., s_v, 0., 0., 0.],
            [0., 0., s_d, 0., 0.],
            [0., 0., 0., -cam.clip_far / (cam.clip_far-cam.clip_near), -1.,],
            [0., 0., -cam.clip_far*cam.clip_near / (cam.clip_far-cam.clip_near), 0., 1.],
        ],

        false => array![
            [s_h, 0., 0., 0., 0.],
            [0., s_v, 0., 0., 0.],
            [0., 0., -cam.clip_far / (cam.clip_far-cam.clip_near), -1., 0.],
            [0., 0., 0., 1., 0.],  // unused row for u.
            [0., 0., -cam.clip_far*cam.clip_near / (cam.clip_far-cam.clip_near), 0., 1.],
        ]

//        array![
//            [s_h, 0., 0., 0., 0.],
//            [0., s_v, 0., 0., 0.],
//            [0., 0., -cam.clip_far / (cam.clip_far-cam.clip_near), -1., 0.],
//            [0., 0., 0., 1., 0.],  // unused row for u.
//            [0., 0., -cam.clip_far*cam.clip_near / (cam.clip_far-cam.clip_near), 0., 1.],
//        ]
    }
}

pub fn project(R: &Array2<f64>, P: &Array2<f64>, T: &Array2<f64>, pt: Array1<f64>,
           is_4d: bool) -> Array1<f64> {
    // Project a 3d node onto a 2d plane, or a 4d node onto a 3d volume.  This
    // assumes the point is already positioned and rotated relative to the camera.
    // https://en.wikipedia.org/wiki/3D_projection
    let augmented_pt = array![pt[0], pt[1], pt[2], pt[3], 1.];
    let positioned_pt = R.dot(&(T.dot(&augmented_pt)));

    let f = P.dot(&positioned_pt);

    if is_4d {
        // Divide to find the 3d projected coords.
       array![f[0] / f[4], f[1] / f[4], f[2] / f[4]]
    }
    else {
        // Divide to find the 2d projected coords.
        array![f[0] / f[4], f[1] / f[4]]
    }
}

pub fn position_shape(shape: &Shape) -> HashMap<i32, Array1<f64>> {
    // Position a shape's nodes in 3 or 4d space, based on its position
    // and rotation parameters.

    let mut is_4d = false;
    // todo Need more checks than rotation speed... temp.
    if shape.rotation_speed[3].abs() > 0. || shape.rotation_speed[4] .abs() > 0. ||
        shape.rotation_speed[5].abs() > 0. {
        is_4d = true;
    }

    // T must be done last, since we scale and rotate with respect to the orgin,
    // defined in the shape's initial nodes. S may be applied at any point.
    let R = match is_4d {
        true => make_rotator_4d(&shape.orientation),
        false => make_rotator_3d(&shape.orientation),
    };
    let S = make_scaler(array![shape.scale, shape.scale, shape.scale, shape.scale]);
    let T = make_translator(&shape.position);

    let mut positioned_nodes = HashMap::new();
    for (id, node) in &shape.nodes {
    // We dot what OpenGL calls the 'Model matrix' with our point. Scale,
    // then rotate, then translate.
        let new_pt = T.dot(&(R.dot(&(S.dot(&node.augmented())))));
        positioned_nodes.insert(*id, new_pt);
    }

    positioned_nodes
}

pub fn project_shapes_3d(shapes: &HashMap<i32, Shape>, cam: &Camera)
        -> HashMap<(i32, i32), Array1<f64>> {
    // Project shapes; modify their nodes to be projected on a 2d surface.
    // The HashMap key is (shape_index, node_index), so we can tie back to the
    // original shapes later.
    let R = make_rotator_3d(&cam.θ_3d);
    let P = make_projector(&cam, false);
    let T = make_translator(&-&(cam.position));

    let mut result = HashMap::new();

    for (shape_id, shape) in shapes {
        let positioned_nodes = position_shape(shape);

        // Iterate over edges so we can clip lines.
        for edge in &shape.edges {
            let node0 = &positioned_nodes[&edge.node0];
            let node1 = &positioned_nodes[&edge.node1];
            let clipped_line = clipping::clip_3d(cam, (node0.clone(), node1.clone()));

            match clipped_line {
                Some(clipped) => {
                    result.insert((*shape_id, edge.node0), project(&R, &P, &T, clipped.0, false));
                    result.insert((*shape_id, edge.node1), project(&R, &P, &T, clipped.1, false));
                },
                // The edge is completely outside the viewing area; move on.
                None => continue
            }
        }
    }
    result
}

pub fn project_shapes_4d(shapes: &HashMap<i32, Shape>, cam: &Camera)
         -> HashMap<(i32, i32), Array1<f64>> {
    // Project shapes; modify their nodes to be projected on a 2d surface.
    let R_4d = make_rotator_4d(&cam.θ_4d);
    let R_3d = make_rotator_3d(&cam.θ_3d);
    let P_4d = make_projector(&cam, true);
    let P_3d = make_projector(&cam, false);
    let T = make_translator(&-&(cam.position));

    // First position the nodes, and project from 4d to 3d
    let mut projected_3d = HashMap::new();

    // todo DRY between here and project_shapes_3d.
    for (shape_id, shape) in shapes {
        let positioned_nodes = position_shape(shape);

        // Iterate over edges so we can clip lines.
        for edge in &shape.edges {
            let node0 = &positioned_nodes[&edge.node0];
            let node1 = &positioned_nodes[&edge.node1];
            let clipped_line = clipping::clip_3d(cam, (node0.clone(), node1.clone()));

            match clipped_line {
                Some(clipped) => {
                    projected_3d.insert((*shape_id, edge.node0), project(&R_4d, &P_4d, &T, clipped.0, true));
                    projected_3d.insert((*shape_id, edge.node1), project(&R_4d, &P_4d, &T, clipped.1, true));
                },
                // The edge is completely outside the viewing area; move on.
                None => continue
            }
        }
    }

    let mut projected_2d = HashMap::new();
    // Now project from 3d to 2d.  We don't need to position
    for (ids, pt) in projected_3d {
        // Re-agument our 3d nodes to len 5, in preparation for projecting to 2d.
        // We only need to clip once; clipping in a higher dimension clips in
        // all lower ones projected into.
        let aug = array![pt[0], pt[1], pt[2], 0., 1.];
        projected_2d.insert(ids, project(&R_3d, &P_3d, &T, aug, false));
    }
    projected_2d
}

pub enum MoveDirection{
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
    Sky,
    Earth,
}

pub fn move_camera(direction: MoveDirection, θ: &Array1<f64>) -> Array1<f64> {
    // Move the camera to a new position, based on where it's pointing.
    let unit_vec = match direction {
        MoveDirection::Forward => array![0., 0., 1., 0.],
        MoveDirection::Back => array![0., 0., -1., 0.],
        MoveDirection::Left => -array![-1., 0., 0., 0.],
        MoveDirection::Right => -array![1., 0., 0., 0.],
        MoveDirection::Up => array![0., 1., 0., 0.],
        MoveDirection::Down => array![0., -1., 0., 0.],
        MoveDirection::Sky => array![0., 0., 0., 1.],
        MoveDirection::Earth => array![0., 0., 0., -1.],
    };

    unit_vec
    // transforms::rotate_4d(θ).dot(&unit_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
}
