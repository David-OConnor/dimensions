// Handles keyboard and mouse input; mirrors input.rs.

import * as state from "./state"
import {addVecs4, dotMV4, mulVConst4} from "./util"
import * as transforms from "./transforms"

export function handlePressed(pressed: number[], deltaT: number,
                                moveSensitivity: number, rotateSensitivity: number,
                                camType: string) {
    // Add if it's not already there.
    const moveAmount = moveSensitivity * deltaT
    const rotateAmount = rotateSensitivity * deltaT

    for (let code of state.currentlyPressedKeys) {
        switch(code) {
            // todo fudge factors on f and back to reverse.
            case 87:  // w
                if (camType === 'single') {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 0, -1, 0]), moveAmount, false)
                }
                break
            case 83:  // s
                if (camType === 'single') {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 0, 1, 0]), moveAmount, false)
                }
                break
            case 68:  // d
                if (camType === 'single') {
                    console.log()
                } else {
                    moveCam(new Float32Array([1, 0, 0, 0]), moveAmount, false)
                }
                break
            case 65:  // a
                if (camType === 'single') {
                    console.log()
                } else {
                    moveCam(new Float32Array([-1, 0, 0, 0]), moveAmount, false)
                }
                break
            case 32:  // Space
                if (camType === 'single') {
                    console.log()
                } else if (camType === 'fps') {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 1, 0, 0]), moveAmount, false)
                }
                break
            case 67:  // c
                if (camType === 'single') {
                    console.log()
                } else if (camType === 'fps') {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, -1, 0, 0]), moveAmount, false)
                }
                break
            case 17:  // Control
                if (camType === 'single') {
                    console.log()
                } else if (camType === 'fps') {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, -1, 0, 0]), moveAmount, false)
                }
                break
            case 82:  // r
                if (camType === 'single') {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 0, 0, 1]), moveAmount, false)
                }
                break
            case 70:  // f
                if (camType === 'single') {
                    console.log()
                } else {
                    moveCam(new Float32Array([0, 0, 0, -1]), moveAmount, false)
                }
                break

            case 38:  // Up
                if (camType === 'single') {
                    state.shapes.get(0).orientation[1] -= rotateAmount
                } else {
                    state.cam.θ[1] += rotateAmount
                }
                break
            case 40:  // Down
                if (camType === 'single') {
                    state.shapes.get(0).orientation[1] += rotateAmount
                } else {
                    state.cam.θ[1] -= rotateAmount
                }
                break
            case 39:  // Right
                if (camType === 'single') {
                    state.shapes.get(0).orientation[2] += rotateAmount
                } else {
                    state.cam.θ[2] -= rotateAmount
                }
                break
            case 37:  // Left
                if (camType === 'single') {
                    state.shapes.get(0).orientation[2] -= rotateAmount
                } else {
                    state.cam.θ[2] += rotateAmount
                }
                break
            case 69:  // E
                if (camType === 'single') {
                    state.shapes.get(0).orientation[0] += rotateAmount
                } else if (camType === 'fps') {
                    console.log()
                } else {
                    state.cam.θ[0] += rotateAmount
                }
                break
            case 81:  // Q
                if (camType === 'single') {
                    state.shapes.get(0).orientation[0] -= rotateAmount
                } else if (camType === 'fps') {
                    console.log()
                } else {
                    state.cam.θ[0] -= rotateAmount
                }
                break
            case 45:  // Ins
                if (camType === 'single') {
                    state.shapes.get(0).orientation[3] += rotateAmount
                } else {
                    state.cam.θ[3] += rotateAmount
                }
                break
            case 46:  // Del
                if (camType === 'single') {
                    state.shapes.get(0).orientation[3] -= rotateAmount
                } else {
                    state.cam.θ[3] -= rotateAmount
                }
                break
            case 36:  // Home
                if (camType === 'single') {
                    state.shapes.get(0).orientation[4] += rotateAmount
                } else {
                    state.cam.θ[4] += rotateAmount
                }
                break
            case 35:  // End
                if (camType === 'single') {
                    state.shapes.get(0).orientation[4] -= rotateAmount
                } else {
                    state.cam.θ[4] -= rotateAmount
                }
                break
            case 33:  // Pgup
                if (camType === 'single') {
                    state.shapes.get(0).orientation[5] += rotateAmount
                } else {
                    state.cam.θ[5] += rotateAmount
                }
                break
            case 34:  // Pgdn
                if (camType === 'single') {
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

function moveCam(unitVec: Float32Array, amount: number, fps: boolean) {
    // Modifies the global camera
    // With first-person-shooter controls, ignore all input except rotation
    // around the y axis.
    const θ = fps ? [0, 0, state.cam.θ[2], 0, 0, 0] : state.cam.θ
    const R = transforms.makeRotator(new Float32Array(16), θ)

    let v = new Float32Array(4)
    dotMV4(v, R, unitVec)

    mulVConst4(v, v, amount)

    addVecs4(state.cam.position, state.cam.position, v)
    // The skybox moves with the camera, but doesn't rotate with it.
    addVecs4(state.skybox.position, state.skybox.position, v)
}
