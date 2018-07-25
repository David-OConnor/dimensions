// This file mirrors transforms.rs.

import {dotMM4} from './util'
import {Shape, Camera} from './types'

// Note: We use matrix conventions that make our 1D matrices, when written
// here, appear as they would in standard linear algebra conventions; this may
// not be the same as OpenGl etc conventions; ie transposed.

export function makeRotator(out: Float32Array, θ: number[]): Float32Array {
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // 4d rotation example: http://kennycason.com/posts/2009-01-08-graph4d-rotation4d-project-to-2d.html
    // http://eusebeia.dyndns.org/4d/vis/10-rot-1

    // We rotation around each of six planes; the combinations of the 4
    // dimensions.

    // cache trig computations
    const cos_xy = Math.cos(θ[0])
    const sin_xy = Math.sin(θ[0])
    const cos_yz = Math.cos(θ[1])
    const sin_yz = Math.sin(θ[1])
    const cos_xz = Math.cos(θ[2])
    const sin_xz = Math.sin(θ[2])
    const cos_xu = Math.cos(θ[3])
    const sin_xu = Math.sin(θ[3])
    const cos_yu = Math.cos(θ[4])
    const sin_yu = Math.sin(θ[4])
    const cos_zu = Math.cos(θ[5])
    const sin_zu = Math.sin(θ[5])

    // Potentially there exist 4 hyperrotations as well? ie combinations of
    // 3 axes ?  xyz  yzu  zux  uxy

    // Rotations around the xy, yz, and xz planes should appear normal.
    let R_xy = Float32Array.from([
        cos_xy, sin_xy, 0., 0.,
        -sin_xy, cos_xy, 0., 0.,
        0., 0., 1., 0.,
        0., 0., 0., 1.
    ])

    const R_yz = Float32Array.from([
        1., 0., 0., 0.,
        0., cos_yz, sin_yz, 0.,
        0., -sin_yz, cos_yz, 0.,
        0., 0., 0., 1.
    ])

    const R_xz = Float32Array.from([
        cos_xz, 0., -sin_xz, 0.,
        0., 1., 0., 0.,
        sin_xz, 0., cos_xz, 0.,
        0., 0., 0., 1.
    ])

    // Rotations involving u, the fourth dimension, should distort 3d objects.
    const R_xu = Float32Array.from([
        cos_xu, 0., 0., sin_xu,
        0., 1., 0., 0.,
        0., 0., 1., 0.,
        -sin_xu, 0., 0., cos_xu
    ])

    const R_yu = Float32Array.from([
        1., 0., 0., 0.,
        0., cos_yu, 0., -sin_yu,
        0., 0., 1., 0.,
        0., sin_yu, 0., cos_yu
    ])

    const R_zu = Float32Array.from([
        1., 0., 0., 0.,
        0., 1., 0., 0.,
        0., 0., cos_zu, -sin_zu,
        0., 0., sin_zu, cos_zu
    ])

    // let R = new Float32Array([
    //     (sin_xy*sin_xz*sin_yz + cos_xy * cos_xz)*cos_xu, (sin_xy*sin_xz*sin_yz + cos_xy*cos_xz)
    // ])

    // todo this defeats the purpose of our out array; ie we create loads of new
    // todo arrays whenever we call this function. Address.
    // Combine the rotations.
    const a = dotMM4(new Float32Array(16), R_yu, R_zu)
    const b = dotMM4(new Float32Array(16), R_xu, a)
    const c = dotMM4(new Float32Array(16), R_xz, b)
    const d = dotMM4(new Float32Array(16), R_yz, c)
    const e = dotMM4(new Float32Array(16), R_xy, d)
    out.set(e)

    // // Combine the rotations.
    // dotMM5(out, R_yu, R_zu)
    // dotMM5(out, R_xu, out)
    // dotMM5(out, R_xz, out)
    // dotMM5(out, R_yz, out)
    // dotMM5(out, R_xy, out)
    return out
}

function makeScaler(out: Float32Array, scale: number): Float32Array {
    // Return a scale matrix; the pt must have 1 appended to its end.
    out[0] = scale; out[1] = 0; out[2] = 0; out[3] = 0;
    out[4] = 0; out[5] = scale; out[6] = 0; out[7] = 0;
    out[8] = 0; out[9] = 0; out[10] = scale; out[11] = 0;
    out[12] = 0; out[13] = 0; out[14] = 0; out[15] = scale

    return out
}

export function makeModelMat4(orientation: number[], scale: number): Float32Array {
    // T must be done last, since we scale and rotate with respect to the orgin,
    // defined in the shape's initial nodes. S may be applied at any point.
    // These 4d matrices don't translate; we do it separately, since we can only
    // pass 4d matrices/vecs to the shader; we use non-homogenous 4d arrays, which
    // we can't perform translations with.

    // In the 4d versions of these functions, we leave out translations, since
    // we require homogenous coords to handle those via matrices, but GL only
    // supports up to 4d matrices.
    let R = new Float32Array(16)
    let S = new Float32Array(16)

    makeScaler(S, scale)
    makeRotator(R, orientation)

    dotMM4(R, R, S)

    return R
}

export function makeViewMat4(θ: number[]): Float32Array {
    // For a first-person sperspective, Translate first; then rotate (around the
    // camera=origin)

    // See note in makeViewMat4 re leaving out translation.

    // Negate, since we're rotating the world relative to the camera.
    const negθ = [-θ[0], -θ[1], -θ[2], -θ[3], -θ[4], -θ[5]]

    // todo creating extra matrices here
    let R = new Float32Array(16)
    makeRotator(R, negθ)
    return R
}


export function makeProjMat(cam: Camera): Float32Array {
    let t = Math.tan(cam.fov / 2.) * cam.near;
    let b = -t;
    let r = t * cam.aspect;
    let l = -t * cam.aspect;
    let n = cam.near;
    let f = cam.far;

    return new Float32Array([
        2.*n / (r - l), 0., (r+l) / (r-l), 0.,
        0., 2.*n / (t-b), (t+b) / (t-b), 0.,
        0., 0., -(f+n) / (f-n), -(2.*f*n) / (f-n) + (-f-n) / (f-n),
        // 0., 0., -(f+n) / (f-n), -(2.*f*n) / (f-n),
        0., 0., 1., cam.fourd_proj_dist * 2.,
    ])
}