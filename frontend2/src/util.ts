// See note in transforms.ts file about matrix conventions.

// These functions avoid creating new arrays, hence the modify-in-place logic
// with out arguments..
// Having algorithms tuned to the specific size matrix is ugly, but efficient.

import * as state from "./state";

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

// export function handleFiles(e: any) {
//     const file = e.target.files[0];
//     const reader = new FileReader();  reader.addEventListener("load", processimage, false)
//     reader.readAsArrayBuffer(file);
// }
//
// function processimage(e: any) {
//     const buffer = e.target.result;
//     const bitmap = getBMP(buffer);
//     const imageData = convertToImageData(bitmap);  ctx1.putImageData(imageData, 0, 0);
// }
//
// function getBMP(buffer: any) {
//     const datav = new DataView(buffer);
//     let bitmap: any = {}
//     bitmap.fileheader = {}; bitmap.fileheader.bfType =
//         datav.getUint16(0, true);
//     bitmap.fileheader.bfSize =
//         datav.getUint32(2, true);
//     bitmap.fileheader.bfReserved1 =
//         datav.getUint16(6, true);
//     bitmap.fileheader.bfReserved2 =
//         datav.getUint16(8, true);
//     bitmap.fileheader.bfOffBits =
//         datav.getUint32(10, true);
//     bitmap.infoheader = {};
//     bitmap.infoheader.biSize =
//         datav.getUint32(14, true);
//     bitmap.infoheader.biWidth =
//         datav.getUint32(18, true);
//     bitmap.infoheader.biHeight =
//         datav.getUint32(22, true);
//     bitmap.infoheader.biPlanes =
//         datav.getUint16(26, true);
//     bitmap.infoheader.biBitCount =
//         datav.getUint16(28, true);
//     bitmap.infoheader.biCompression =
//         datav.getUint32(30, true);
//     bitmap.infoheader.biSizeImage =
//         datav.getUint32(34, true);
//     bitmap.infoheader.biXPelsPerMeter =
//         datav.getUint32(38, true);
//     bitmap.infoheader.biYPelsPerMeter =
//         datav.getUint32(42, true);
//     bitmap.infoheader.biClrUsed =
//         datav.getUint32(46, true);
//     bitmap.infoheader.biClrImportant =
//         datav.getUint32(50, true);
//
//      const start = bitmap.fileheader.bfOffBits;  bitmap.stride =
//   Math.floor((bitmap.infoheader.biBitCount
//     *bitmap.infoheader.biWidth +
//                             31) / 32) * 4;
//  bitmap.pixels =
//          new Uint8Array(buffer, start);
//  return bitmap;
// }
//
// function convertToImageData(bitmap) {
//  canvas = document.createElement("canvas");
//  var ctx = canvas.getContext("2d");
//  var Width = bitmap.infoheader.biWidth;
//  var Height = bitmap.infoheader.biHeight;
//  var imageData = ctx.createImageData(
//                            Width, Height);