import * as transforms from "./transforms";
import {Vec5} from "./interfaces";
import * as state from "./state";

export function moveCam(unitVec: number[], fps: boolean) {
    // Modifies the global camera
    // With first-person-shooter controls, ignore all input except rotation
    // around the y axis.
    const θ = fps ? [0, 0, state.cam.θ[2], 0, 0, 0] : state.cam.θ

    const direc = transforms.make_rotator(θ).dotV(new Vec5(unitVec))
    const amount = direc.mul(state.moveSensitivity)
    state.cam.position = state.cam.position.add(amount)
    // The skybox moves with the camera, but doesn't rotate with it.
    state.skybox.position.add(amount)
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
                    moveCam([0, 0, 1, 0], true)
                } else {
                    moveCam([0, 0, 1, 0], false)
                }
                event.preventDefault()
                break
            case 83:  // s
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam([0, 0, -1, 0], false)
                }
                break
            case 68:  // d
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam([1, 0, 0, 0], false)
                }
                break
            case 65:  // a
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam([-1, 0, 0, 0], false)
                }
                break
            case 32:  // Space
                if (scene_ === 0) {
                    console.log()
                } else if (scene_ === 2) {
                    console.log()
                } else {
                    moveCam([0, 1, 0, 0], false)
                }
                event.preventDefault()
                break
            case 67:  // c
                if (scene_ === 0) {
                    console.log()
                } else if (scene_ === 2) {
                    console.log()
                } else {
                    moveCam([0, -1, 0, 0], false)
                }
                break
            case 17:  // Control
                if (scene_ === 0) {
                    console.log()
                } else if (scene_ === 2) {
                    console.log()
                } else {
                    moveCam([0, -1, 0, 0], false)
                }
                break
            case 82:  // r
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam([0, 0, 0, 1], false)
                }
                break
            case 70:  // f
                if (scene_ === 0) {
                    console.log()
                } else {
                    moveCam([0, 0, 0, -1], false)
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