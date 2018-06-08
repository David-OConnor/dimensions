import * as state from "./state";

export function makeV5(vals: number[]) {
    // Custom constructor to make a 5-element Float32Array from 4.
    // We store things internally as 5-vecs as much as possible, for compatibility
    // with 5x5 transform matrices.
    return Float32Array.from([vals[0], vals[1], vals[2], vals[3], 1.])
}

// export function addVecs4(a: Float32Array, b: Float32Array): Float32Array {
//     // Must have 4 elements.
//     return Float32Array.from([
//         a[0] + b[0],
//         a[1] + b[1],
//         a[2] + b[2],
//         a[3] + b[3],
//     ])
// }

export function addVecs5(a: Float32Array, b: Float32Array): Float32Array {
    // Must have 5 elements.
    return Float32Array.from([
        a[0] + b[0],
        a[1] + b[1],
        a[2] + b[2],
        a[3] + b[3],
        a[4] + b[4],
    ])
}

// export function mulVConst4(V: Float32Array, c: number): Float32Array {
//     // Multiply a Matrix (flattened Float32Array) by a constant.
//     V[0] *= c
//     V[1] *= c
//     V[2] *= c
//     V[3] *= c
//     return V
// }

export function mulVConst5(V: Float32Array, c: number): Float32Array {
    // Multiply a Matrix (flattened Float32Array) by a constant.
    V[0] *= c
    V[1] *= c
    V[2] *= c
    V[3] *= c
    V[4] *= c
    return V
}

export function dotMV5(M: Float32Array, v: Float32Array): Float32Array {
    // M is a 5x5 matrix, flattened. v is a 5-element vector.
    // Ugly, but efficient.
    return Float32Array.from([
        v[0]*M[0] + v[1]*M[1] + v[2]*M[2] + v[3]*M[3] + v[4]*M[4],
        v[0]*M[5] + v[1]*M[6] + v[2]*M[7] + v[3]*M[8] + v[4]*M[9],
        v[0]*M[10] + v[1]*M[11] + v[2]*M[12] + v[3]*M[13] + v[4]*M[14],
        v[0]*M[15] + v[1]*M[16] + v[2]*M[17] + v[3]*M[18] + v[4]*M[19],
        v[0]*M[20] + v[1]*M[21] + v[2]*M[22] + v[3]*M[23] + v[4]*M[24],
    ])
}

export function dotMM5(M0: Float32Array, M1: Float32Array): Float32Array {
    // M0 and M1 are both 5x5 flattened matrices.
    // Very ugly, but efficient.
    return Float32Array.from([
        // Row 0
        M0[0]*M1[0] + M0[1]*M1[5] + M0[2]*M1[10] + M0[3]*M1[15] + M0[4]*M1[20],
        M0[0]*M1[1] + M0[1]*M1[6] + M0[2]*M1[11] + M0[3]*M1[16] + M0[4]*M1[21],
        M0[0]*M1[2] + M0[1]*M1[7] + M0[2]*M1[12] + M0[3]*M1[17] + M0[4]*M1[22],
        M0[0]*M1[3] + M0[1]*M1[8] + M0[2]*M1[13] + M0[3]*M1[18] + M0[4]*M1[23],
        M0[0]*M1[4] + M0[1]*M1[9] + M0[2]*M1[14] + M0[3]*M1[19] + M0[4]*M1[24],

        // Row 1
        M0[5]*M1[0] + M0[6]*M1[5] + M0[7]*M1[10] + M0[8]*M1[15] + M0[9]*M1[20],
        M0[5]*M1[1] + M0[6]*M1[6] + M0[7]*M1[11] + M0[8]*M1[16] + M0[9]*M1[21],
        M0[5]*M1[2] + M0[6]*M1[7] + M0[7]*M1[12] + M0[8]*M1[17] + M0[9]*M1[22],
        M0[5]*M1[3] + M0[6]*M1[8] + M0[7]*M1[13] + M0[8]*M1[18] + M0[9]*M1[23],
        M0[5]*M1[4] + M0[6]*M1[9] + M0[7]*M1[14] + M0[8]*M1[19] + M0[9]*M1[24],

        // Row 2
        M0[10]*M1[0] + M0[11]*M1[5] + M0[12]*M1[10] + M0[13]*M1[15] + M0[14]*M1[20],
        M0[10]*M1[1] + M0[11]*M1[6] + M0[12]*M1[11] + M0[13]*M1[16] + M0[14]*M1[21],
        M0[10]*M1[2] + M0[11]*M1[7] + M0[12]*M1[12] + M0[13]*M1[17] + M0[14]*M1[22],
        M0[10]*M1[3] + M0[11]*M1[8] + M0[12]*M1[13] + M0[13]*M1[18] + M0[14]*M1[23],
        M0[10]*M1[4] + M0[11]*M1[9] + M0[12]*M1[14] + M0[13]*M1[19] + M0[14]*M1[24],

        // Row 3
        M0[15]*M1[0] + M0[16]*M1[5] + M0[17]*M1[10] + M0[18]*M1[15] + M0[19]*M1[20],
        M0[15]*M1[1] + M0[16]*M1[6] + M0[17]*M1[11] + M0[18]*M1[16] + M0[19]*M1[21],
        M0[15]*M1[2] + M0[16]*M1[7] + M0[17]*M1[12] + M0[18]*M1[17] + M0[19]*M1[22],
        M0[15]*M1[3] + M0[16]*M1[8] + M0[17]*M1[13] + M0[18]*M1[18] + M0[19]*M1[23],
        M0[15]*M1[4] + M0[16]*M1[9] + M0[17]*M1[14] + M0[18]*M1[19] + M0[19]*M1[24],

        // Row 4
        M0[20]*M1[0] + M0[21]*M1[5] + M0[22]*M1[10] + M0[23]*M1[15] + M0[24]*M1[20],
        M0[20]*M1[1] + M0[21]*M1[6] + M0[22]*M1[11] + M0[23]*M1[16] + M0[24]*M1[21],
        M0[20]*M1[2] + M0[21]*M1[7] + M0[22]*M1[12] + M0[23]*M1[17] + M0[24]*M1[22],
        M0[20]*M1[3] + M0[21]*M1[8] + M0[22]*M1[13] + M0[23]*M1[18] + M0[24]*M1[23],
        M0[20]*M1[4] + M0[21]*M1[9] + M0[22]*M1[14] + M0[23]*M1[19] + M0[24]*M1[24],
    ])
}

// export function makeHomo(V: Float32Array): Float32Array {
//     // Add a trailing 1 to a len-4 unit vector, to make it homogenous.
//     return Float32Array.from([V[0], V[1], V[2], V[3], 1.])
// }
//
// export function noHomo(V: Float32Array): Float32Array {
//     // Remove trailing 1 from homogenous len 5 vec.
//     return Float32Array.from([V[0], V[1], V[2], V[3]])
// }
