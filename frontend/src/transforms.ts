// This file mirrors transforms.rs.

import {dotMM5, dotMV5, mulVConst5, addVecs5, makeV5} from './util'
import {Shape, Camera} from './interfaces'
import * as state from "./state";

// Note: We use matrix conventions that make our 1D matrices, when written
// here, appear as they would in standard linear algebra conventions; this may
// not be the same as OpenGl etc conventions; ie transposed.

function moveCam(unitVec: Float32Array, fps: boolean) {
    // Modifies the global camera
    // With first-person-shooter controls, ignore all input except rotation
    // around the y axis.
    const θ = fps ? [0, 0, state.cam.θ[2], 0, 0, 0] : state.cam.θ
    const R = makeRotator(new Float32Array(25), θ)

    let v = new Float32Array(5)
    dotMV5(v, R, unitVec)

    mulVConst5(v, v, state.moveSensitivity)

    addVecs5(state.cam.position, state.cam.position, v)
    // The skybox moves with the camera, but doesn't rotate with it.
    addVecs5(state.skybox.position, state.skybox.position, v)
}

// export function rotateVec(out: Float32Array, cachedTrig: any, θ: number[]): Float32Array {
//     // See note about operating on vectors directly as oppposed to returning
//     // a composable matrix, in the scaleVec / translateVec funcs.
//     let cos_xy, sin_xy, cos_yz, sin_yz, cos_xz, sin_xz, cos_xu, sin_xu, cos_yu,
//         sin_yu, cos_zu, sin_zu
//
//     // cache trig computations; or not.
//     if (cachedTrig == null) {
//         cos_xy = Math.cos(θ[0])
//         sin_xy = Math.sin(θ[0])
//         cos_yz = Math.cos(θ[1])
//         sin_yz = Math.sin(θ[1])
//         cos_xz = Math.cos(θ[2])
//         sin_xz = Math.sin(θ[2])
//         cos_xu = Math.cos(θ[3])
//         sin_xu = Math.sin(θ[3])
//         cos_yu = Math.cos(θ[4])
//         sin_yu = Math.sin(θ[4])
//         cos_zu = Math.cos(θ[5])
//         sin_zu = Math.sin(θ[5])
//     } else {
//         cos_xy = cachedTrig.cos_xy
//         sin_xy = cachedTrig.sin_xy
//         cos_yz = cachedTrig.cos_yz
//         sin_yz = cachedTrig.sin_yz
//         cos_xz = cachedTrig.cos_xz
//         sin_xz = cachedTrig.sin_xz
//         cos_xu = cachedTrig.cos_xu
//         sin_xu = cachedTrig.sin_xu
//         cos_yu = cachedTrig.cos_yu
//         sin_yu = cachedTrig.sin_yu
//         cos_zu = cachedTrig.cos_zu
//         sin_zu = cachedTrig.sin_zu
//     }
//
//     // zu
//     out[2] = out[2] * cos_zu + out[3] * -sin_zu
//     out[3] = out[2] * sin_zu + out[3] * cos_zu
//
//     // yu
//     out[1] = out[1] * cos_yu + out[3] * -sin_yu
//     out[3] = out[1] * sin_yu + out[3] * cos_yu
//
//     // xu
//     out[0] = out[0] * cos_xu  + out[3] * sin_xu
//     out[3] = out[0] * -sin_xu  + out[3] * cos_xu
//
//     // xz
//     out[0] = out[0] * cos_xz + out[2] * -sin_xz
//     out[2] = out[0] * sin_xz + out[2] * cos_xz
//
//     // yz
//     out[1] = out[1] * cos_yz + out[2] * sin_yz
//     out[2] = out[1] * -sin_yz + out[2] * cos_yz
//
//     // xy
//     out[0] = out[0] * cos_xy + out[1] * sin_xy
//     out[1] = out[0] * -sin_xy + out[1] * cos_xy
//
//     return out
// }

function makeRotator(out: Float32Array, θ: number[]): Float32Array {
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
        cos_xy, sin_xy, 0., 0., 0.,
        -sin_xy, cos_xy, 0., 0., 0.,
        0., 0., 1., 0., 0.,
        0., 0., 0., 1., 0.,
        0., 0., 0., 0., 1.
    ])

    const R_yz = Float32Array.from([
        1., 0., 0., 0., 0.,
        0., cos_yz, sin_yz, 0., 0.,
        0., -sin_yz, cos_yz, 0., 0.,
        0., 0., 0., 1., 0.,
        0., 0., 0., 0., 1.
    ])

    const R_xz = Float32Array.from([
        cos_xz, 0., -sin_xz, 0., 0.,
        0., 1., 0., 0., 0.,
        sin_xz, 0., cos_xz, 0., 0.,
        0., 0., 0., 1., 0.,
        0., 0., 0., 0., 1.
    ])

    // Rotations involving u, the fourth dimension, should distort 3d objects.
    const R_xu = Float32Array.from([
        cos_xu, 0., 0., sin_xu, 0.,
        0., 1., 0., 0., 0.,
        0., 0., 1., 0., 0.,
        -sin_xu, 0., 0., cos_xu, 0.,
        0., 0., 0., 0., 1.
    ])

    const R_yu = Float32Array.from([
        1., 0., 0., 0., 0.,
        0., cos_yu, 0., -sin_yu, 0.,
        0., 0., 1., 0., 0.,
        0., sin_yu, 0., cos_yu, 0.,
        0., 0., 0., 0., 1.
    ])

    const R_zu = Float32Array.from([
        1., 0., 0., 0., 0.,
        0., 1., 0., 0., 0.,
        0., 0., cos_zu, -sin_zu, 0.,
        0., 0., sin_zu, cos_zu, 0.,
        0., 0., 0., 0., 1.
    ])

    // Combine the rotations.
    dotMM5(out, R_yu, R_zu)
    dotMM5(out, R_xu, out)
    dotMM5(out, R_xz, out)
    dotMM5(out, R_yz, out)
    dotMM5(out, R_xy, out)
    return out
}

function makeTranslator(out: Float32Array, position: Float32Array): Float32Array {
    // Return a translation matrix; the pt must have 1 appended to its end.
    // We do this augmentation so we can add a constant term.  Scale and
    // rotation matrices may have this as well for matrix compatibility.
    // Ugly, but efficient...
    out[0] = 1; out[1] = 0; out[2] = 0; out[3] = 0; out[4] = position[0]
    out[5] = 0; out[6] = 1; out[7] = 0; out[8] = 0; out[9] = position[1]
    out[10] = 0; out[11] = 0; out[12] = 1; out[13] = 0; out[14] = position[2]
    out[15] = 0; out[16] = 0; out[17] = 0; out[18] = 1; out[19] = position[3]
    out[20] = 0; out[21] = 0; out[22] = 0; out[23] = 0; out[24] = 1

    return out
}

// function translateVec(out: Float32Array, position: Float32Array): Float32Array {
//     // Again, not as elegant as returning a composable matrix, but efficient for Webgl.
//     out[0] += position[0]
//     out[1] += position[1]
//     out[2] += position[2]
//     out[3] += position[3]
//
//     return out
// }

function makeScaler(out: Float32Array, scale: Float32Array): Float32Array {
    // Return a scale matrix; the pt must have 1 appended to its end.
    out[0] = scale[0]; out[1] = 0; out[2] = 0; out[3] = 0; out[4] = 0
    out[5] = 0; out[6] = scale[1]; out[7] = 0; out[8] = 0; out[9] = 0
    out[10] = 0; out[11] = 0; out[12] = scale[2]; out[13] = 0; out[14] = 0
    out[15] = 0; out[16] = 0; out[17] = 0; out[18] = scale[3]; out[19] = 0
    out[20] = 0; out[21] = 0; out[22] = 0; out[23] = 0; out[24] = 1

    return out
}

// function scaleVec(out: Float32Array, scale: Float32Array): Float32Array {
//     // Apply a scale transform to a vector. Inelegant to do it this way, but efficient.
//     out[0] *= scale[0]
//     out[1] *= scale[1]
//     out[2] *= scale[2]
//     out[3] *= scale[3]
//
//     return out
// }

function makeProjector(cam: Camera): Float32Array {
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

    let y_scale = 1. / Math.tan(cam.fov / 2.)
    let x_scale = y_scale / cam.aspect
    let u_scale = y_scale / cam.aspect_4  // depth for 4d

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

    return Float32Array.from([
        x_scale, 0., 0., 0., 0.,
        0., y_scale, 0., 0., 0.,
        0., 0., (cam.far + cam.near) / (cam.far - cam.near),
        (-2. * cam.far * cam.near) / (cam.far - cam.near),  0.,
        // u_scale is, ultimately, not really used.
        0., 0., 0., u_scale, 0.,
        // This row allows us to divide by z after taking the dot product,
        // as part of our scaling operation.
        0., 0., 1., 0., 1.,
    ])
}

export function makeModelMat(shape: Shape): Float32Array {
    // T must be done last, since we scale and rotate with respect to the orgin,
    // defined in the shape's initial nodes. S may be applied at any point.
    const scaler = new Float32Array([shape.scale, shape.scale,
                                     shape.scale, shape.scale, shape.scale])
    // todo creating extra matrices here
    let R = new Float32Array(25)
    let T = new Float32Array(25)
    let S = new Float32Array(25)
    let M = new Float32Array(25)

    // makeRotator(R, shape.orientation)
    makeScaler(S, scaler)
    makeTranslator(T, shape.position)

    dotMM5(M, R, S)
    dotMM5(M, T, M)

    return T
    // return M
}

export function makeViewMat(cam: Camera): Float32Array {
    // For a first-person sperspective, Translate first; then rotate (around the
    // camera=origin)

    // Negate, since we're rotating the world relative to the camera.
    const negθ = [-cam.θ[0], -cam.θ[1], -cam.θ[2], -cam.θ[3], -cam.θ[4], -cam.θ[5]]
    const negPos = Float32Array.from([-cam.position[0], -cam.position[1],
                                      -cam.position[2], -cam.position[3], 1])

    // todo creating extra matrices here
    let T = new Float32Array(25)
    let R = new Float32Array(25)
    let M = new Float32Array(25)
    makeRotator(R, negθ)
    makeTranslator(T, negPos)

    dotMM5(M, R, T)

    return M
}

function positionShape(shape: Shape): Map<number, Float32Array> {
    // Position a shape's nodes in 3 or 4d space, based on its position
    // and rotation parameters.
    const modelMatrix = makeModelMat(shape)
    let pt

    let positionedNodes = new Map()
    for (let id=0; id < shape.nodes.size; id++) {
        let node: any = shape.nodes.get(id)
        pt = new Float32Array(5)
        pt.set(node.a)
        dotMV5(pt, modelMatrix, pt)

        positionedNodes.set(id, pt)
    }

    return positionedNodes
}

export function processShapes(cam: Camera, shapes: Map<number, Shape>): Map<string, Float32Array> {
    // Set up shapes rel to their model, and the camera.
    // T must be done last.
    const viewMatrix = makeViewMat(cam)
    let pt

    let result = new Map()
    shapes.forEach(
        (shape, shapeId, map) => {
            positionShape(shape).forEach(
                (node, nodeId, _map) => {
                    pt = new Float32Array(5)
                    pt.set(node)

                    dotMV5(pt, viewMatrix, pt)
                    // Map doesn't accept tuples/arrays as keys
                    result.set([shapeId, nodeId].join(','), pt)
                }
            )
        }
    )
    return result
}

export function handleKeyDown(event: any, scene_: number) {
    // Add if it's not already there.
    if (state.currentlyPressedKeys.indexOf(event.keyCode) === -1) {
        state.currentlyPressedKeys.push(event.keyCode)
    }

    for (let code of state.currentlyPressedKeys) {
        switch(code) {
            case 87:  // w
                if (scene_ === 0) {
                    console.log()
                } else if (scene_ === 2) {
                    moveCam(makeV5([0, 0, 1, 0]), true)
                } else {
                    moveCam(makeV5([0, 0, 1, 0]), false)
                }
                event.preventDefault()
                break
            case 83:  // s
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam(makeV5([0, 0, -1, 0]), false)
                }
                break
            case 68:  // d
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam(makeV5([1, 0, 0, 0]), false)
                }
                break
            case 65:  // a
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam(makeV5([-1, 0, 0, 0]), false)
                }
                break
            case 32:  // Space
                if (scene_ === 0) {
                    console.log()
                } else if (scene_ === 2) {
                    console.log()
                } else {
                    moveCam(makeV5([0, 1, 0, 0]), false)
                }
                event.preventDefault()
                break
            case 67:  // c
                if (scene_ === 0) {
                    console.log()
                } else if (scene_ === 2) {
                    console.log()
                } else {
                    moveCam(makeV5([0, -1, 0, 0]), false)
                }
                break
            case 17:  // Control
                if (scene_ === 0) {
                    console.log()
                } else if (scene_ === 2) {
                    console.log()
                } else {
                    moveCam(makeV5([0, -1, 0, 0]), false)
                }
                break
            case 82:  // r
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam(makeV5([0, 0, 0, 1]), false)
                }
                break
            case 70:  // f
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam(makeV5([0, 0, 0, -1]), false)
                }
                break
            // todo add deltaTime!
            case 38:  // Up
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[1] -= state.rotateSensitivity
                } else {
                    state.cam.θ[1] += state.rotateSensitivity
                }
                event.preventDefault();
                break
            case 40:  // Down
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[1] += state.rotateSensitivity
                } else {
                    state.cam.θ[1] -= state.rotateSensitivity
                }
                event.preventDefault();
                break
            case 39:  // Right
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[2] += state.rotateSensitivity
                } else {
                    state.cam.θ[2] -= state.rotateSensitivity
                }
                event.preventDefault();
                break
            case 37:  // Left
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[2] -= state.rotateSensitivity
                } else {
                    state.cam.θ[2] += state.rotateSensitivity
                    event.preventDefault();
                }
                break
            case 69:  // E
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[0] += state.rotateSensitivity
                } else if (scene_ === 2) {
                    console.log()
                } else {
                    state.cam.θ[0] += state.rotateSensitivity
                }
                break
            case 81:  // Q
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[0] -= state.rotateSensitivity
                } else if (scene_ === 2) {
                    console.log()
                } else {
                    state.cam.θ[0] -= state.rotateSensitivity
                }
                break
            case 45:  // Ins
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[3] += state.rotateSensitivity
                } else {
                    state.cam.θ[3] += state.rotateSensitivity
                }
                break
            case 46:  // Del
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[3] -= state.rotateSensitivity
                } else {
                    state.cam.θ[3] -= state.rotateSensitivity
                }
                event.preventDefault();
                break
            case 36:  // Home
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[4] += state.rotateSensitivity
                } else {
                    state.cam.θ[4] += state.rotateSensitivity
                }
                event.preventDefault();
                break
            case 35:  // End
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[4] -= state.rotateSensitivity
                } else {
                    state.cam.θ[4] -= state.rotateSensitivity
                }
                event.preventDefault();
                break
            case 33:  // Pgup
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[5] += state.rotateSensitivity
                } else {
                    state.cam.θ[5] += state.rotateSensitivity
                }
                event.preventDefault();
                break
            case 34:  // Pgdn
                if (scene_ === 0) {
                    state.shapes.get(0).orientation[5] -= state.rotateSensitivity
                } else {
                    state.cam.θ[5] -= state.rotateSensitivity
                }
                event.preventDefault();
                break
            default:
                break
        }
    }
}

export function handleKeyUp(event: any) {
    let index = state.currentlyPressedKeys.indexOf(event.keyCode)
    if (index !== -1) { state.currentlyPressedKeys.splice(index, 1) }
}
