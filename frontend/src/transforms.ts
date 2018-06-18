// This file mirrors transforms.rs.

import {dotMM4, dotMV4, mulVConst4, addVecs4} from './util'
import {Shape, Camera} from './types'
import * as state from "./state";
import {setCam} from "./state";

// Note: We use matrix conventions that make our 1D matrices, when written
// here, appear as they would in standard linear algebra conventions; this may
// not be the same as OpenGl etc conventions; ie transposed.

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

export function makeModelMat4(shape: Shape): Float32Array {
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

    makeScaler(S, shape.scale)
    makeRotator(R, shape.orientation)

    dotMM4(R, R, S)

    return R
}

export function makeViewMat4(cam: Camera): Float32Array {
    // For a first-person sperspective, Translate first; then rotate (around the
    // camera=origin)

    // See note in makeViewMat4 re leaving out translation.

    // Negate, since we're rotating the world relative to the camera.
    const negθ = [-cam.θ[0], -cam.θ[1], -cam.θ[2], -cam.θ[3], -cam.θ[4], -cam.θ[5]]

    // todo creating extra matrices here
    let R = new Float32Array(16)
    makeRotator(R, negθ)
    return R
}

function moveCam(unitVec: Float32Array, amount: number, fps: boolean) {
    // Modifies the global camera
    // With first-person-shooter controls, ignore all input except rotation
    // around the y axis.
    const θ = fps ? [0, 0, state.cam.θ[2], 0, 0, 0] : state.cam.θ
    const R = makeRotator(new Float32Array(16), θ)

    let v = new Float32Array(4)
    dotMV4(v, R, unitVec)

    mulVConst4(v, v, amount)

    addVecs4(state.cam.position, state.cam.position, v)
    // The skybox moves with the camera, but doesn't rotate with it.
    addVecs4(state.skybox.position, state.skybox.position, v)
}

export function handleKeys(currentlyPressed: number[], deltaT: number,
                           moveSensitivity: number, rotateSensitivity: number) {
    // Add if it's not already there.
    const moveAmount = moveSensitivity * deltaT
    const rotateAmount = rotateSensitivity * deltaT
    
    for (let code of state.currentlyPressedKeys) {
        switch(code) {
            // todo fudge factors on f and back to reverse.
            case 87:  // w
                if (state.scene === 0) {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 0, -1, 0]), moveAmount, false)
                }
                break
            case 83:  // s
                if (state.scene === 0) {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 0, 1, 0]), moveAmount, false)
                }
                break
            case 68:  // d
                if (state.scene === 0) {
                    console.log()
                } else {
                    moveCam(new Float32Array([1, 0, 0, 0]), moveAmount, false)
                }
                break
            case 65:  // a
                if (state.scene === 0) {
                    console.log()
                } else {
                    moveCam(new Float32Array([-1, 0, 0, 0]), moveAmount, false)
                }
                break
            case 32:  // Space
                if (state.scene === 0) {
                    console.log()
                } else if (state.scene === 2) {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 1, 0, 0]), moveAmount, false)
                }
                break
            case 67:  // c
                if (state.scene === 0) {
                    console.log()
                } else if (state.scene === 2) {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, -1, 0, 0]), moveAmount, false)
                }
                break
            case 17:  // Control
                if (state.scene === 0) {
                    console.log()
                } else if (state.scene === 2) {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, -1, 0, 0]), moveAmount, false)
                }
                break
            case 82:  // r
                if (state.scene === 0) {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 0, 0, 1]), moveAmount, false)
                }
                break
            case 70:  // f
                if (state.scene === 0) {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 0, 0, -1]), moveAmount, false)
                }
                break
            // todo add deltaTime!
            case 38:  // Up
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[1] -= rotateAmount
                } else {
                    state.cam.θ[1] += rotateAmount
                }
                break
            case 40:  // Down
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[1] += rotateAmount
                } else {
                    state.cam.θ[1] -= rotateAmount
                }
                break
            case 39:  // Right
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[2] += rotateAmount
                } else {
                    state.cam.θ[2] -= rotateAmount
                }
                break
            case 37:  // Left
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[2] -= rotateAmount
                } else {
                    state.cam.θ[2] += rotateAmount
                }
                break
            case 69:  // E
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[0] += rotateAmount
                } else if (state.scene === 2) {
                    console.log()
                } else {
                    state.cam.θ[0] += rotateAmount
                }
                break
            case 81:  // Q
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[0] -= rotateAmount
                } else if (state.scene === 2) {
                    console.log()
                } else {
                    state.cam.θ[0] -= rotateAmount
                }
                break
            case 45:  // Ins
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[3] += rotateAmount
                } else {
                    state.cam.θ[3] += rotateAmount
                }
                break
            case 46:  // Del
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[3] -= rotateAmount
                } else {
                    state.cam.θ[3] -= rotateAmount
                }
                break
            case 36:  // Home
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[4] += rotateAmount
                } else {
                    state.cam.θ[4] += rotateAmount
                }
                break
            case 35:  // End
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[4] -= rotateAmount
                } else {
                    state.cam.θ[4] -= rotateAmount
                }
                break
            case 33:  // Pgup
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[5] += rotateAmount
                } else {
                    state.cam.θ[5] += rotateAmount
                }
                break
            case 34:  // Pgdn
                if (state.scene === 0) {
                    state.shapes.get(0).orientation[5] -= rotateAmount
                } else {
                    state.cam.θ[5] -= rotateAmount
                }
                break
            default:
                break
        }
    }
}

export function handleKeyDown(event: any) {
    // Prevent scrolling etc behavior from keys we use.
    if ([87, 83, 68, 65, 32, 67, 17, 82, 70, 38, 40, 39, 37, 59, 81, 45, 46, 36,35, 33, 34 ].
        indexOf(event.keyCode) > -1) { event.preventDefault() }
    if (state.currentlyPressedKeys.indexOf(event.keyCode) === -1) {
        state.currentlyPressedKeys.push(event.keyCode)
    }
}

export function handleKeyUp(event: any) {
    let index = state.currentlyPressedKeys.indexOf(event.keyCode)
    if (index !== -1) { state.currentlyPressedKeys.splice(index, 1) }
}
