// See note in transforms.ts file about matrix conventions.

// These functions avoid creating new arrays, hence the modify-in-place logic
// with out arguments..
// Having algorithms tuned to the specific size matrix is ugly, but efficient.

import * as state from "./state";
import {Camera, Lighting, Mesh, Normal, Scene, Shape, Vertex} from "./types";

export function addVecs4(out: Float32Array, a: Float32Array, b: Float32Array): Float32Array {
    // Must have 5 elements.
    out[0] = a[0] + b[0]
    out[1] = a[1] + b[1]
    out[2] = a[2] + b[2]
    out[3] = a[3] + b[3]

    return out
}

export function mulVConst4(out: Float32Array, V: Float32Array, c: number): Float32Array {
    // Multiply a Matrix (flattened Float32Array) by a constant.
    out[0] = V[0] * c
    out[1] = V[1] * c
    out[2] = V[2] * c
    out[3] = V[3] * c

    return out
}

export function dotMV4(out: Float32Array, M: Float32Array, v: Float32Array): Float32Array {
    // M is a 5x5 matrix, flattened. v is a 5-element vector.
    // Ugly, but efficient.
    // See note on dotMM5 re the out argument.

    out[0] = v[0]*M[0] + v[1]*M[1] + v[2]*M[2] + v[3]*M[3]
    out[1] = v[0]*M[4] + v[1]*M[5] + v[2]*M[6] + v[3]*M[7]
    out[2] = v[0]*M[8] + v[1]*M[9] + v[2]*M[10] + v[3]*M[11]
    out[3] = v[0]*M[12] + v[1]*M[13] + v[2]*M[14] + v[3]*M[15]

    return out
}

export function dotMM4(out: Float32Array, M0: Float32Array, M1: Float32Array): Float32Array {
    // M0 and M1 are both 5x5 flattened matrices.
    // Very ugly, but efficient.
    // out is the array we output; copying glmatrix; I think this prevents creating
    // excess arrays.
    // Row 0

    out[0] = M0[0]*M1[0] + M0[1]*M1[4] + M0[2]*M1[8] + M0[3]*M1[12]
    out[1] = M0[0]*M1[1] + M0[1]*M1[5] + M0[2]*M1[9] + M0[3]*M1[13]
    out[2] = M0[0]*M1[2] + M0[1]*M1[6] + M0[2]*M1[10] + M0[3]*M1[14]
    out[3] = M0[0]*M1[3] + M0[1]*M1[7] + M0[2]*M1[11] + M0[3]*M1[15]

    // Row 1
    out[4] = M0[4]*M1[0] + M0[5]*M1[4] + M0[6]*M1[8] + M0[7]*M1[12]
    out[5] = M0[4]*M1[1] + M0[5]*M1[5] + M0[6]*M1[9] + M0[7]*M1[13]
    out[6] = M0[4]*M1[2] + M0[5]*M1[6] + M0[6]*M1[10] + M0[7]*M1[14]
    out[7] = M0[4]*M1[3] + M0[5]*M1[7] + M0[6]*M1[11] + M0[7]*M1[15]

    // Row 2
    out[8] = M0[8]*M1[0] + M0[9]*M1[4] + M0[10]*M1[8] + M0[11]*M1[12]
    out[9] = M0[8]*M1[1] + M0[9]*M1[5] + M0[10]*M1[9] + M0[11]*M1[13]
    out[10] = M0[8]*M1[2] + M0[9]*M1[6] + M0[10]*M1[10] + M0[11]*M1[14]
    out[11] = M0[8]*M1[3] + M0[9]*M1[7] + M0[10]*M1[11] + M0[11]*M1[15]

    // Row 3
    out[12] = M0[12]*M1[0] + M0[13]*M1[4] + M0[14]*M1[8] + M0[15]*M1[12]
    out[13] = M0[12]*M1[1] + M0[13]*M1[5] + M0[14]*M1[9] + M0[15]*M1[13]
    out[14] = M0[12]*M1[2] + M0[13]*M1[6] + M0[14]*M1[10] + M0[15]*M1[14]
    out[15] = M0[12]*M1[3] + M0[13]*M1[7] + M0[14]*M1[11] + M0[15]*M1[15]

    return out
}

function testDotMatrixVec() {
    const M = Float32Array.from([
        0, 2, 3, 1,
        1, 2, 3, 1,
        -1, -2, -3, 1,
        0, 0, 1, 2,
    ])

    const v = Float32Array.from([3, 1, 1.5, 2])
    let result = new Float32Array(4)
    dotMV4(result, M, v)
    console.log("Test", result)
}

function testDotMatrixMatrix() {
    const M = Float32Array.from([
        0, 2, 3, 1,
        1, 2, 3, 1,
        -1, -2, -3, 1,
        0, 0, 1, 2
    ])

    let result = new Float32Array(16)
    dotMM4(result, M, M)
    console.log("Test", result)
    // Should be [-1, -2, -2,  7], // todo nope
    //        [ 5,  4, -3, 10],
    //        [ 1,  0,  1, -4],
    //        [ 2,  0, -3,  6],
}

export function findColor(dist: number): number[] {
    // produce a color ranging from red to blue, based on how close a point is
    // to the edge.
    let portion_through = Math.abs(dist) / state.scene.color_max

    if (portion_through > 1.) {
        portion_through = 1.
    }
    const baseGray = .0
    const colorVal = (baseGray + portion_through * 1. - baseGray)

    if (dist > 0) {
        return [baseGray, baseGray, colorVal, 0.2]  // Blue
    } else {
        return [colorVal, baseGray, baseGray, 0.2]  // Red
    }
}

export function deserSceneLib(rawLib: any) : Map<number, Scene> {
    // Convert the deserialized nested object passed from wasm_bindgen into the
    // format used here; eg Map instead of object when appropriate, typed arrays.
    let result = new Map()
    let cam, scene: any, shapes: Map<number, Shape>, shape: Shape, mesh: Mesh,
        vertices: Map<number, Vertex>
    // Convert from an object with strings as keys to a map.
    Object.keys(rawLib).forEach((id) => {
        scene = rawLib[id]

        shapes = new Map()
        // Like with out scenelib, iterate through a shape object to produce a Map.
        Object.keys(scene.shapes).forEach((s_id) => {
            shape = scene.shapes[s_id]

            vertices = new Map()
            Object.keys(shape.mesh.vertices).forEach((v_id: any) => {
                vertices.set(parseInt(v_id), new Vertex(shape.mesh.vertices[v_id]))
            })

            mesh = new Mesh(
                vertices,
                shape.mesh.faces_vert.map((fv: any) => new Uint16Array(fv)),
                shape.mesh.normals.map((n: any) => new Normal(n))
            )

            shapes.set(parseInt(s_id), new Shape(
                mesh,
                new Float32Array(shape.position),
                shape.orientation,
                shape.rotation_speed,
                shape.opacity
                )
            )
        })

        cam = new Camera (
            new Float32Array(scene.cam.position),
            scene.cam.θ,
            scene.cam.fov,
            scene.cam.aspect,
            scene.cam.aspect_4,
            scene.cam.fourd_proj_dist,
            scene.cam.near,
            scene.cam.far,
            scene.cam.strange,
        )

        result.set(parseInt(id),
            {
                shapes: shapes,
                cam: cam,
                cam_type: scene.cam_type.toLowerCase(),
                color_max: scene.color_max,
                lighting: scene.lighting,
                sensitivities: scene.sensitivities
            }
        )
    })

    return result
}
