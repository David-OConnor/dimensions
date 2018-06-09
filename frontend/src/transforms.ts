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
    let v = new Float32Array(5)
    dotMV5(v, make_rotator(θ), unitVec)

    mulVConst5(v, v, state.moveSensitivity)

    addVecs5(state.cam.position, state.cam.position, v)
    // The skybox moves with the camera, but doesn't rotate with it.
    addVecs5(state.skybox.position, state.skybox.position, v)
}

export function make_rotator(θ: number[]): Float32Array {
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
    let R = new Float32Array(25)

    dotMM5(R, R_yu, R_zu)
    dotMM5(R, R_xu, R)
    dotMM5(R, R_xz, R)
    dotMM5(R, R_yz, R)
    dotMM5(R, R_xy, R)
    return R

    // const R_1 = dotMM5(R_xy, dotMM5(R_yz, R_xz))
    // const R_2 = dotMM5(R_xu, dotMM5(R_yu, R_zu))
    // return dotMM5(R_1, R_2)
}

export function make_translator(out: Float32Array, position: Float32Array): Float32Array {
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

export function make_scaler(out: Float32Array, scale: Float32Array): Float32Array {
    // Return a scale matrix; the pt must have 1 appended to its end.
    out[0] = scale[0]; out[1] = 0; out[2] = 0; out[3] = 0; out[4] = 0
    out[5] = 0; out[6] = scale[1]; out[7] = 0; out[8] = 0; out[9] = 0
    out[10] = 0; out[11] = 0; out[12] = scale[2]; out[13] = 0; out[14] = 0
    out[15] = 0; out[16] = 0; out[17] = 0; out[18] = scale[3]; out[19] = 0
    out[20] = 0; out[21] = 0; out[22] = 0; out[23] = 0; out[24] = 1

    return out
}

export function make_projector(cam: Camera): Float32Array {
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

export function positionShape(shape: Shape): Map<number, Float32Array> {
    // Position a shape's nodes in 3 or 4d space, based on its position
    // and rotation parameters.

    // T must be done last, since we scale and rotate with respect to the orgin,
    // defined in the shape's initial nodes. S may be applied at any point.
    const R = make_rotator(shape.orientation)
    let S = new Float32Array(25)
    make_scaler(S, new Float32Array([shape.scale, shape.scale,
        shape.scale, shape.scale, shape.scale]))
    let T = new Float32Array(25)
    make_translator(T, shape.position)

    let positionedNodes = new Map()
    for (let id=0; id < shape.nodes.size; id++) {

        let node: any = shape.nodes.get(id)
        // We dot what OpenGL calls the 'Model matrix' with our point. Scale,
        // then rotate, then translate.
        let M = new Float32Array(25)
        dotMM5(M, R, S)
        dotMM5(M, T, M)
        let pt = new Float32Array(5)
        dotMV5(pt, M, node.a)

        // const transform = dotMM5(T, dotMM5(R, S))
        // const newPt = dotMV5(transform, node.a)
        positionedNodes.set(id, pt)
    }

    return positionedNodes
}

export function processShapes(cam: Camera, shapes: Map<number, Shape>): Map<string, Float32Array> {
    // Set up shapes rel to their model, and the camera.
    // T must be done last.
    let result = new Map()
    let positionedModel

    let negRot = [-cam.θ[0], -cam.θ[1], -cam.θ[2], -cam.θ[3], -cam.θ[4], -cam.θ[5]]

    const R = make_rotator(negRot)

    const negPos = Float32Array.from([-cam.position[0],
        -cam.position[1], -cam.position[2],
        -cam.position[3], 1])

    let T = new Float32Array(25)
    make_translator(T, negPos)
    // For cam transform, position first; then rotate.
    let M = new Float32Array(25)
    dotMM5(M, R, T)

    let V = new Float32Array(5)
    shapes.forEach(
        (shape, id, map) => {
            positionedModel = positionShape(shape)
            positionedModel.forEach(
                (node, nid, _map) => {
                    dotMV5(V, M, node)
                    // Map doesn't like tuples/arrays as keys :/
                    result.set([id, nid].join(','), V)
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