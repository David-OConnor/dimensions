use std::collections::HashMap;

use ndarray::prelude::*;

use clipping;
use types::{Camera, Shape};

pub fn make_rotator(θ: &Array1<f64>) -> Array2<f64> {
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

pub fn make_translator(position: &Array1<f64>) -> Array2<f64> {
    // Return a translation matrix; the pt must have 1 appended to its end.
    // We do this augmentation so we can add a constant term.  Scale and
    // rotation matrices may have this as well for matrix compatibility.
    assert_eq![position.len(), 4];

    array![
        [1., 0., 0., 0., position[0]],
        [0., 1., 0., 0., position[1]],
        [0., 0., 1., 0., position[2]],
        [0., 0., 0., 1., position[3]],
        [0., 0., 0., 0., 1.],
    ]
}

fn make_scaler(scale: &Array1<f64>) -> Array2<f64> {
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

pub fn make_projector(cam: &Camera) -> Array2<f64> {
    // Create the projection matrix, used to transform translated and
    // rotated points.

    // Let's compile the different versions you've seen:
    // 1: http://learnwebgl.brown37.net/08_projections/projections_perspective.html
    // 2: https://en.wikipedia.org/wiki/3D_projection
    // 3: https://solarianprogrammer.com/2013/05/22/opengl-101-matrices-projection-view-model/
    // 4: https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-
    // projection-matrix/building-basic-perspective-projection-matrix
    // 5: https://github.com/brendanzab/cgmath/blob/master/src/projection.rs
    // 6: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/WebGL_model_view_projection
    // 7: http://www.songho.ca/opengl/gl_projectionmatrix.html

    // 7's on point with my calcs, although stated in terms of right/top.

    let y_scale = 1. / (cam.fov / 2. as f64).tan();
    let x_scale = y_scale / cam.aspect;
    let u_scale = y_scale / cam.aspect_4;  // depth for 4d

    // We are defining z as the axis that determines how x and y points are
    // scaled, for both 4d and 3d projections. U points don't play a factor
    // in our final result; their data is only included during rotations;
    // This function transforms them, but that ultimately is not projected to
    // 2d screens.

    // Insight: z (or u, depending on which convention we settle on) is used
    // for two things: Determining how we should scale x and y (The vars that


    // I've derived these matrices myself; none of the ones described in the
    // above links seem to produce a unit cube for easy clipping.
    // They map the frustum to a "unit" [hyper]cube; actually ranging from -1 to +1,
    // along each axis.
    // Note: Unlike x, y, (and u?) z (doesn't map in a linear way; it goes
    // as a decaying exponential from -1 to +1.

    array![
            [x_scale, 0., 0., 0., 0.],
            [0., y_scale, 0., 0., 0.],
            [0., 0., (cam.far + cam.near) / (cam.far - cam.near),
                (-2. * cam.far * cam.near) / (cam.far - cam.near),  0.],
            // u_scale is, ultimately, not really used.
            [0., 0., 0., u_scale, 0.],
            // This row allows us to divide by z after taking the dot product,
            // as part of our scaling operation.
            [0., 0., 1., 0., 1.],
        ]
}

pub fn position_shape(shape: &Shape) -> HashMap<u32, Array1<f64>> {
    // Position a shape's nodes in 3 or 4d space, based on its position
    // and rotation parameters.

    let is_4d = shape.rotation_speed[3].abs() > 0. || shape.rotation_speed[4] .abs() > 0. ||
        shape.rotation_speed[5].abs() > 0. || shape.orientation[3].abs() > 0. ||
        shape.orientation[4].abs() > 0. || shape.orientation[5].abs() > 0.;

    // T must be done last, since we scale and rotate with respect to the orgin,
    // defined in the shape's initial nodes. S may be applied at any point.
    let R = make_rotator(&shape.orientation);
    let S = make_scaler(&array![shape.scale, shape.scale, shape.scale, shape.scale]);
    let T = make_translator(&shape.position);

    let mut positioned_nodes = HashMap::new();
    for (id, node) in &shape.nodes {
    // We dot what OpenGL calls the 'Model matrix' with our point. Scale,
    // then rotate, then translate.
        let homogenous = array![node.a[0], node.a[1], node.a[2], node.a[3], 1.];
        let new_pt = T.dot(&(R.dot(&(S.dot(&homogenous)))));
        positioned_nodes.insert(*id, new_pt);
    }

    positioned_nodes
}

fn project(pt: &Array1<f64>, T: &Array2<f64>, R: &Array2<f64>,
             P: &Array2<f64>) -> Array1<f64> {
    // Helper function to reduce repetition in project_shapes.
    // Clip our edge, which has been projected into "clipspace", eg a
    // cube ranging from -1 to +1 on each axes.
    // augmented points let us add constant values with matrix math.

    // Homogenous points simplify calculations by allowing us to add constant values.
    let homogeneous = array![pt[0], pt[1], pt[2], pt[3], 1.];
    // Project into clipspace.
    let f = P.dot(&(R.dot(&(T.dot(&homogeneous)))));
    // We divide by z (or u in 4d), since this is part of our calculation for
    // projecting into the "unit" cube clipspace.

    // todo: I'm not sure exactly why we must reverse y.
    array![f[0] / f[4], -f[1] / f[4], f[2] / f[4], f[3] / f[4]]
}

pub fn project_shapes(shapes: &HashMap<u32, Shape>, cam: &Camera)
        -> HashMap<(u32, u32), Array1<f64>> {
    // Position and rotate shapes relative to the camera; project into a
    // clipspace [hyper]frustum.
    // The HashMap key is (shape_index, node_index), so we can tie back to the
    // original shapes later.
    // We negate R and T, since we're shifting and rotating relative to the
    // [fixed] camera.
    let T = make_translator(&-&(cam.position));
    let R = make_rotator(&-&(cam.θ));
    let P = make_projector(&cam);

    let mut result = HashMap::new();

    for (shape_id, shape) in shapes {
        let positioned_pts = position_shape(shape);

        // Iterate over edges so we can clip lines.
        for edge in &shape.edges {
            let pt_0 = &positioned_pts[&edge.node0];
            let pt_1 = &positioned_pts[&edge.node1];
            let pt_0_clipspace = project(pt_0, &T, &R, &P);
            let pt_1_clipspace = project(pt_1, &T, &R, &P);

            let clipped = clipping::clip_3d((&pt_0_clipspace, &pt_1_clipspace));
            if let Some((pt_0_clipped, pt_1_clipped)) = clipped {
                // We return the full (non-homogenous) projected point, even
                // though we only need x and y to display.
                result.insert((*shape_id, edge.node0), pt_0_clipped);
                result.insert((*shape_id, edge.node1), pt_1_clipped);
            }
        }
    }
    result
}

pub enum MoveDirection{
    Forward,
    Back,
    Left,
    Right,
    Up,
    Down,
    Ana,
    Kata,
}

pub fn move_camera(direction: MoveDirection, θ: &Array1<f64>) -> Array1<f64> {
    // Move the camera to a new position, based on where it's pointing.
    let unit_vec = match direction {
        MoveDirection::Forward => array![0., 0., 1., 0.],
        MoveDirection::Back => array![0., 0., -1., 0.],
        MoveDirection::Left => array![-1., 0., 0., 0.],
        MoveDirection::Right => array![1., 0., 0., 0.],
        MoveDirection::Up => array![0., 1., 0., 0.],
        MoveDirection::Down => array![0., -1., 0., 0.],
        MoveDirection::Ana => array![0., 0., 0., 1.],
        MoveDirection::Kata => array![0., 0., 0., -1.],
    };

    unit_vec
    // transforms::rotate_4d(θ).dot(&unit_vec)
}

#[cfg(test)]
mod tests {
    use super::*;
}
