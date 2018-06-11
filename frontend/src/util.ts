// See note in transforms.ts file about matrix conventions.

// These functions avoid creating new arrays, hence the modify-in-place logic
// with out arguments..
// Having algorithms tuned to the specific size matrix is ugly, but efficient.

import * as state from "./state";

export function makeV5(vals: number[]): Float32Array {
    // Custom constructor to make a 5-element Float32Array from 4.
    // We store things internally as 5-vecs as much as possible, for compatibility
    // with 5x5 transform matrices.
    return new Float32Array([vals[0], vals[1], vals[2], vals[3], 1.])
}

export function addVecs5(out: Float32Array, a: Float32Array, b: Float32Array): Float32Array {
    // Must have 5 elements.
    out[0] = a[0] + b[0]
    out[1] = a[1] + b[1]
    out[2] = a[2] + b[2]
    out[3] = a[3] + b[3]
    out[4] = a[4] + b[4]

    return out
}

export function mulVConst5(out: Float32Array, V: Float32Array, c: number): Float32Array {
    // Multiply a Matrix (flattened Float32Array) by a constant.
    out[0] = V[0] * c
    out[1] = V[1] * c
    out[2] = V[2] * c
    out[3] = V[3] * c
    out[4] = V[4] * c

    return out
}

export function dotMV5(out: Float32Array, M: Float32Array, v: Float32Array): Float32Array {
    // M is a 5x5 matrix, flattened. v is a 5-element vector.
    // Ugly, but efficient.
    // See note on dotMM5 re the out argument.

    out[0] = v[0]*M[0] + v[1]*M[1] + v[2]*M[2] + v[3]*M[3] + v[4]*M[4]
    out[1] = v[0]*M[5] + v[1]*M[6] + v[2]*M[7] + v[3]*M[8] + v[4]*M[9]
    out[2] = v[0]*M[10] + v[1]*M[11] + v[2]*M[12] + v[3]*M[13] + v[4]*M[14]
    out[3] = v[0]*M[15] + v[1]*M[16] + v[2]*M[17] + v[3]*M[18] + v[4]*M[19]
    out[4] = v[0]*M[20] + v[1]*M[21] + v[2]*M[22] + v[3]*M[23] + v[4]*M[24]

    return out
}

export function dotMM5(out: Float32Array, M0: Float32Array, M1: Float32Array): Float32Array {
    // M0 and M1 are both 5x5 flattened matrices.
    // Very ugly, but efficient.
    // out is the array we output; copying glmatrix; I think this prevents creating
    // excess arrays.
    // Row 0
    out[0] = M0[0]*M1[0] + M0[1]*M1[5] + M0[2]*M1[10] + M0[3]*M1[15] + M0[4]*M1[20]
    out[1] = M0[0]*M1[1] + M0[1]*M1[6] + M0[2]*M1[11] + M0[3]*M1[16] + M0[4]*M1[21]
    out[2] = M0[0]*M1[2] + M0[1]*M1[7] + M0[2]*M1[12] + M0[3]*M1[17] + M0[4]*M1[22]
    out[3] = M0[0]*M1[3] + M0[1]*M1[8] + M0[2]*M1[13] + M0[3]*M1[18] + M0[4]*M1[23]
    out[4] = M0[0]*M1[4] + M0[1]*M1[9] + M0[2]*M1[14] + M0[3]*M1[19] + M0[4]*M1[24]

    // Row 1
    out[5] = M0[5]*M1[0] + M0[6]*M1[5] + M0[7]*M1[10] + M0[8]*M1[15] + M0[9]*M1[20]
    out[6] = M0[5]*M1[1] + M0[6]*M1[6] + M0[7]*M1[11] + M0[8]*M1[16] + M0[9]*M1[21]
    out[7] = M0[5]*M1[2] + M0[6]*M1[7] + M0[7]*M1[12] + M0[8]*M1[17] + M0[9]*M1[22]
    out[8] = M0[5]*M1[3] + M0[6]*M1[8] + M0[7]*M1[13] + M0[8]*M1[18] + M0[9]*M1[23]
    out[9] = M0[5]*M1[4] + M0[6]*M1[9] + M0[7]*M1[14] + M0[8]*M1[19] + M0[9]*M1[24]

    // Row 2
    out[10] = M0[10]*M1[0] + M0[11]*M1[5] + M0[12]*M1[10] + M0[13]*M1[15] + M0[14]*M1[20]
    out[11] = M0[10]*M1[1] + M0[11]*M1[6] + M0[12]*M1[11] + M0[13]*M1[16] + M0[14]*M1[21]
    out[12] = M0[10]*M1[2] + M0[11]*M1[7] + M0[12]*M1[12] + M0[13]*M1[17] + M0[14]*M1[22]
    out[13] = M0[10]*M1[3] + M0[11]*M1[8] + M0[12]*M1[13] + M0[13]*M1[18] + M0[14]*M1[23]
    out[14] = M0[10]*M1[4] + M0[11]*M1[9] + M0[12]*M1[14] + M0[13]*M1[19] + M0[14]*M1[24]

    // Row 3
    out[15] = M0[15]*M1[0] + M0[16]*M1[5] + M0[17]*M1[10] + M0[18]*M1[15] + M0[19]*M1[20]
    out[16] = M0[15]*M1[1] + M0[16]*M1[6] + M0[17]*M1[11] + M0[18]*M1[16] + M0[19]*M1[21]
    out[17] = M0[15]*M1[2] + M0[16]*M1[7] + M0[17]*M1[12] + M0[18]*M1[17] + M0[19]*M1[22]
    out[18] = M0[15]*M1[3] + M0[16]*M1[8] + M0[17]*M1[13] + M0[18]*M1[18] + M0[19]*M1[23]
    out[19] = M0[15]*M1[4] + M0[16]*M1[9] + M0[17]*M1[14] + M0[18]*M1[19] + M0[19]*M1[24]

    // Row 4
    out[20] = M0[20]*M1[0] + M0[21]*M1[5] + M0[22]*M1[10] + M0[23]*M1[15] + M0[24]*M1[20]
    out[21] = M0[20]*M1[1] + M0[21]*M1[6] + M0[22]*M1[11] + M0[23]*M1[16] + M0[24]*M1[21]
    out[22] = M0[20]*M1[2] + M0[21]*M1[7] + M0[22]*M1[12] + M0[23]*M1[17] + M0[24]*M1[22]
    out[23] = M0[20]*M1[3] + M0[21]*M1[8] + M0[22]*M1[13] + M0[23]*M1[18] + M0[24]*M1[23]
    out[24] = M0[20]*M1[4] + M0[21]*M1[9] + M0[22]*M1[14] + M0[23]*M1[19] + M0[24]*M1[24]

    return out
}

function testDotMatrixVec() {
    const M = Float32Array.from([
        0, 2, 3, 1, 0,
        1, 2, 3, 1, 2,
        -1, -2, -3, 1, 0,
        0, 0, 1, 2, 1,
        3, 2, -2, 1, 0
    ])

    const v = Float32Array.from([3, 1, 1.5, 2, -4.])
    let result = new Float32Array(5)
    dotMV5(result, M, v)
    console.log("Test", result)
    // Should be [8.5, 3.5, -7.5, 1.5, 10]
}

function testDotMatrixMatrix() {
    const M = Float32Array.from([
        0, 2, 3, 1, 0,
        1, 2, 3, 1, 2,
        -1, -2, -3, 1, 0,
        0, 0, 1, 2, 1,
        3, 2, -2, 1, 0
    ])

    let result = new Float32Array(25)
    dotMM5(result, M, M)
    console.log("Test", result)
    // Should be [-1, -2, -2,  7,  5],
    //        [ 5,  4, -3, 10,  5],
    //        [ 1,  0,  1, -4, -3],
    //        [ 2,  0, -3,  6,  2],
    //        [ 4, 14, 22,  5,  5]
}

export function findColor(dist: number): number[] {
    // produce a color ranging from red to blue, based on how close a point is
    // to the edge.
    let portion_through = Math.abs(dist) / state.colorMax

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
