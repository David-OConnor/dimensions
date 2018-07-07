use std::collections::HashMap;

use ndarray::prelude::*;

use clipping;
use types::{Camera, Shape};

pub fn dot_mv4(M: [[f32; 4]; 4], v: [f32; 4]) -> [f32; 4] {
    // Dot a len-4 matrix with a vec.
    [
        v[0]*M[0][0] + v[1]*M[0][1] + v[2]*M[0][2] + v[3]*M[0][3],
        v[0]*M[1][0] + v[1]*M[1][1] + v[2]*M[1][2] + v[3]*M[1][3],
        v[0]*M[2][0] + v[1]*M[2][1] + v[2]*M[2][2] + v[3]*M[2][3],
        v[0]*M[3][0] + v[1]*M[3][1] + v[2]*M[3][2] + v[3]*M[3][3]
    ]
}

pub fn dot_mm4(M0: [[f32; 4]; 4], M1: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    // Dot a len-4 matrix with another matrix.
    [
    // Row 0
    [M0[0][0]*M1[0][0] + M0[0][1]*M1[1][0] + M0[0][2]*M1[2][0] + M0[0][3]*M1[3][0],
    M0[0][0]*M1[0][1] + M0[0][1]*M1[1][1] + M0[0][2]*M1[2][1] + M0[0][3]*M1[3][1],
    M0[0][0]*M1[0][2] + M0[0][1]*M1[1][2] + M0[0][2]*M1[2][2] + M0[0][3]*M1[3][2],
    M0[0][0]*M1[0][3] + M0[0][1]*M1[1][3] + M0[0][2]*M1[2][3] + M0[0][3]*M1[3][3]],

    // Row 1
    [M0[1][0]*M1[0][0] + M0[1][1]*M1[1][0] + M0[1][2]*M1[2][0] + M0[1][3]*M1[3][0],
    M0[1][0]*M1[0][1] + M0[1][1]*M1[1][1] + M0[1][2]*M1[2][1] + M0[1][3]*M1[3][1],
    M0[1][0]*M1[0][2] + M0[1][1]*M1[1][2] + M0[1][2]*M1[2][2] + M0[1][3]*M1[3][2],
    M0[1][0]*M1[0][3] + M0[1][1]*M1[1][3] + M0[1][2]*M1[2][3] + M0[1][3]*M1[3][3]],

    // Row 2
    [M0[2][0]*M1[0][0] + M0[2][1]*M1[1][0] + M0[2][2]*M1[2][0] + M0[2][3]*M1[3][0],
    M0[2][0]*M1[0][1] + M0[2][1]*M1[1][1] + M0[2][2]*M1[2][1] + M0[2][3]*M1[3][1],
    M0[2][0]*M1[0][2] + M0[2][1]*M1[1][2] + M0[2][2]*M1[2][2] + M0[2][3]*M1[3][2],
    M0[2][0]*M1[0][3] + M0[2][1]*M1[1][3] + M0[2][2]*M1[2][3] + M0[2][3]*M1[3][3]],

    // Row 3
    [M0[3][0]*M1[0][0] + M0[3][1]*M1[1][0] + M0[3][2]*M1[2][0] + M0[3][3]*M1[3][0],
    M0[3][0]*M1[0][1] + M0[3][1]*M1[1][1] + M0[3][2]*M1[2][1] + M0[3][3]*M1[3][1],
    M0[3][0]*M1[0][2] + M0[3][1]*M1[1][2] + M0[3][2]*M1[2][2] + M0[3][3]*M1[3][2],
    M0[3][0]*M1[0][3] + M0[3][1]*M1[1][3] + M0[3][2]*M1[2][3] + M0[3][3]*M1[3][3]],
    ]
}

pub fn make_rotator4(θ: &Array1<f32>) -> [[f32; 4]; 4] {
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // 4d rotation example: http://kennycason.com/posts/2009-01-08-graph4d-rotation4d-project-to-2d.html
    // http://eusebeia.dyndns.org/4d/vis/10-rot-1
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
    let R_xy = [
        [cos_xy, sin_xy, 0., 0.],
        [-sin_xy, cos_xy, 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.],
    ];

    let R_yz = [
        [1., 0., 0., 0.],
        [0., cos_yz, sin_yz, 0.],
        [0., -sin_yz, cos_yz, 0.],
        [0., 0., 0., 1.],
    ];

    let R_xz = [
        [cos_xz, 0., -sin_xz, 0.],
        [0., 1., 0., 0.],
        [sin_xz, 0., cos_xz, 0.],
        [0., 0., 0., 1.],
    ];

    // Rotations involving u, the fourth dimension, should distort 3d objects.
    let R_xu = [
        [cos_xu, 0., 0., sin_xu],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [-sin_xu, 0., 0., cos_xu],
    ];

    let R_yu = [
        [1., 0., 0., 0.],
        [0., cos_yu, 0., -sin_yu],
        [0., 0., 1., 0.],
        [0., sin_yu, 0., cos_yu],
    ];

    let R_zu = [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., cos_zu, -sin_zu],
        [0., 0., sin_zu, cos_zu],
    ];

    // Combine the rotations.

    let R_1 = dot_mm4(R_xy, dot_mm4(R_yz, R_xz));
    let R_2 = dot_mm4(R_xu, dot_mm4(R_yu, R_zu));
    dot_mm4(R_1, R_2)
}

pub fn I4() -> [[f32; 4]; 4] {
    [
        [1., 0., 0., 0.],
        [0., 1., 0., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., 1.]
    ]
}

fn make_scaler4(scale: f32) -> [[f32; 4]; 4] {
    // Return a scale matrix; the pt must have 1 appended to its end.
    [
        [scale, 0., 0., 0.],
        [0., scale, 0., 0.],
        [0., 0., scale, 0.],
        [0., 0., 0., scale],
    ]
}

pub fn make_proj_mat4(cam: &Camera) -> [[f32; 4]; 4] {
    // This variant returns a 4x4, non-homogenous matrix in the array format used
    // by Vulkan.
    // Vulkan uses a right-handed coordinate system with a depth range of [0,1],

    // Using this GL matrix, multiplied by a vulkan-corrector.
    // http://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-projection-
    // matrix/opengl-perspective-projection-matrix

    let t = (cam.fov / 2.).tan() * cam.near;
    let b = -t;
    let r = t * cam.aspect;
    let l = -t * cam.aspect;
    let n = cam.near;
    let f = cam.far;

    // Row one multiplies x by (2 * cam.near) / far frustum horizontal dist
    // Row two multiplies y by (2 * cam.near) / far frustum vertical dist

    // Not really sure what row 3's doing, but it's setting up z for vulkano
    // to be used later.

    // Row 4 is used silently by Vulkan, to scale x and y to the frustum based on
    // their z distance. If the last entry isn't 0, by u as well for 4d visual
    // scaling cues. (Subjective). The values here are
    // negative, due to Vulkan's RHS coordinate system. z is divided by two here
    // and in other places due to Zulkan using a z dist of 0 to 1 in clipspace,
    // vice -1 to 1.

    // todo: It appears that Vulkan will cause incorrect clipping  against
    // todo the near plane if the u scaler is too high.

    // The terms in the third and fourth column turn out to be 0, unless the view is skewed, eg
    // more is shown to the right of center than left.  We don't do that, but leave
    // those terms in, for now.

    [
        [2.*n / (r - l), 0., (r+l) / (r-l) / 2., 0.],
        [0., -2.*n / (t-b), (t+b) / (t-b) / 2., (b+t) / (t-b) / 2.],
        [0., 0., -(f+n) / (f-n) / 2., -(2.*f*n) / (f-n) + (-f-n) / (f-n) / 2.],
        // u_scale is, ultimately, not really used.
        // This row allows us to divide by z after taking the dot product,
        // as part of our scaling operation.
        [0., 0., -0.5, -0.],
    ]
}

pub fn make_model_mat4(shape: &Shape) -> [[f32; 4]; 4] {
    // We ommit translation, since we are constrained
    let S = make_scaler4(shape.scale);
    let R = make_rotator4(&shape.orientation);
    dot_mm4(R, S)
}

pub fn make_view_mat4(cam: &Camera) -> [[f32; 4]; 4] {
    // Non-homogenous, in the nested-array format used by Vulkan.
    let negθ = array![-cam.θ[0], -cam.θ[1], -cam.θ[2], -cam.θ[3], -cam.θ[4], -cam.θ[5]];
    make_rotator4(&negθ)
}


#[cfg(test)]
mod tests {
    use super::*;
}
