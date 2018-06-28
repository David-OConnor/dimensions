// This file mirrors shape_maker.rs  Uses _ naming conventions in some case for
// interoperability. Uses semicolons in many places, other like other parts
// of the frontend.

import {Shape, Node2, Edge, Face} from "./types"

export function makeBox(lens: [number, number, number],
                        position: Float32Array, orientation: number[],
                        rotation_speed: number[]): Shape {
    // Make a rectangular prism.  Use negative lengths to draw in the opposite
    // direction.

    const coords = [
        // Front
        [-1., -1., -1., 0.],
        [1., -1., -1., 0.],
        [1., 1., -1., 0.],
        [-1., 1., -1., 0.],

        // Back
        [-1., -1., 1., 0.],
        [1., -1., 1., 0.],
        [1., 1., 1., 0.],
        [-1., 1., 1., 0.],
    ];

    let nodes = new Map()
    for (let id=0; id < coords.length; id++) {
        let coord = coords[id];
        nodes.set(id, new Node2(new Float32Array([coord[0] * lens[0]/2, coord[1] * lens[1]/2,
            coord[2] * lens[2]/2, coord[3]])))
    }

    // Divide the vertex position by its length to make normalized vectors.
    // The distance from the center to a corner.
    let d = Math.sqrt(Math.pow(lens[0] / 2, 2) + Math.pow(lens[1], 2) + Math.pow(lens[2] / 2., 2))

    const edges = [
        // Front
        new Edge(0, 1),
        new Edge(1, 2),
        new Edge(2, 3),
        new Edge(3, 0),

        // Back
        new Edge(4, 5),
        new Edge(5, 6),
        new Edge(6, 7),
        new Edge(7, 4),

        // Bridger
        new Edge(0, 4),
        new Edge(1, 5),
        new Edge(2, 6),
        new Edge(3, 7)
    ];

    let faces = [
        // Front
        new Face([edges[0], edges[1], edges[2], edges[3]]),
        // Back
        new Face([edges[4], edges[5], edges[6], edges[7]]),
        // Top
        new Face([edges[2], edges[10], edges[6], edges[11]]),
        // Bottom
        new Face([edges[0], edges[9], edges[4], edges[8]]),
        // Left
        new Face([edges[3], edges[8], edges[7], edges[11]]),
        // Right
        new Face([edges[1], edges[9], edges[5], edges[10]]),
    ];

    const faces_vert = [  // Vertex indices for each face.
        [0, 1, 2, 3],  // Front
        [4, 5, 6, 7],  // Back
        [3, 2, 6, 7],  // Top
        [0, 1, 5, 4],  // Bottom
        [0, 4, 7, 3],  // Left
        [1, 5, 6, 2],  // Right
    ];

    // Normals correspond to faces.
    const normals = [
        [0., 0., -1., 0.],
        [0., 0., 1., 0.],
        [0., 1., 0., 0.],
        [0., -1., 0., 0.],
        [-1., 0., 0., 0.],
        [1., 0., 0., 0.]
    ]

    return new Shape(nodes, edges, faces, faces_vert, normals, position,
        orientation, rotation_speed)
}

export function makeCube(side_len: number,
                         position: Float32Array, orientation: number[],
                         rotation_speed: number[]): Shape {
    // Convenience function.
    // We'll still treat the center as the center of the base portion.
    return makeBox([side_len, side_len, side_len], position, orientation, rotation_speed)
}

export function makeRectangularPyramid(lens: [number, number, number],
                                       position: Float32Array, orientation: number[],
                                       rotation_speed: number[]): Shape {
    const coords = [
        // Base
        [-1., 0., -1., 0.],
        [1., 0., -1., 0.],
        [1., 0., 1., 0.],
        [-1., 0., 1., 0.],

        // Top
        [0., 1., 0., 0.],
    ]

    let nodes = new Map()
    for (let id=0; id < coords.length; id++) {
        let coord = coords[id]
        nodes.set(id, new Node2(new Float32Array([coord[0] * lens[0]/2, coord[1] * lens[1]/2,
            coord[2] * lens[2]/2, coord[3]/2])))
    }

    const edges = [
        // Base
        new Edge(0, 1),
        new Edge(1, 2),
        new Edge(2, 3),
        new Edge(3, 0),

        // Connect base to tip
        new Edge(0, 4),
        new Edge(1, 4),
        new Edge(2, 4),
        new Edge(3, 4)
    ]

    const faces = [
        // Base
        new Face([edges[0], edges[1], edges[2], edges[3]]),
        // Front
        new Face([edges[0], edges[4], edges[5]]),
        // Right
        new Face([edges[0], edges[4], edges[5]]),
        // Back
        new Face([edges[2], edges[6], edges[7]]),
        // Left
        new Face([edges[3], edges[7], edges[4]])
    ]

    const faces_vert = [  // Vertex indices for each face.
        [0, 1, 2, 3],  // Base
        [0, 1, 4],  // Front
        [1, 2, 4],  // Right
        [2, 3, 4],  // Back
        [3, 0, 4],  // Left
    ]

    // Normals correspond to faces.
    // Note that these don't need to be normalized here; the shader will do it.
    const normals = [
        [0., -1., 0., 0.],
        [0., lens[2], -lens[1], 0.],
        [-lens[2], lens[1], 0., 0.],
        [0., lens[2], lens[1], 0.],
        [lens[2], lens[1], 0., 0.],
    ]

    return new Shape(nodes, edges, faces, faces_vert, normals, position, orientation, rotation_speed)
}

export function make_house(lens: [number, number, number],
                           position: Float32Array,
                           orientation: number[],
                           rotation_speed: number[]): Shape {
    const empty_array = [0., 0., 0., 0., 0., 0.];

    // We'll modify base in-place, then return it.
    let base = makeBox(lens, position, orientation, rotation_speed);

    let roof = makeRectangularPyramid(
        // Let the roof overhang the base by a little.
        // Make the roof height a portion of the base height.
        [lens[0] * 1.2, lens[1] / 3., lens[2] * 1.2],
        new Float32Array([0, 0, 0, 0]), empty_array, empty_array
    );

    // Now that we've made the shapes, recompose them to be one shape.
    // todo make this a separate, (reusable) func?1
    const base_node_count = base.nodes.size;

    roof.nodes.forEach(
        (node, id, map) => {
            let lifted_node = node;
            // Raise the roof
            lifted_node.a[1] += lens[1] / 2.
            base.nodes.set(
                id + base_node_count,
                lifted_node
            )
        }
    )
    // todo combine normals.

    for (let edge of roof.edges) {
        base.edges.push(new Edge(edge.node0 + base_node_count, edge.node1 + base_node_count))
    }

    for (let face of roof.faces) {
        base.faces.push(face)
    }

    let updated_fv
    for (let face of roof.faces_vert) {
        updated_fv = []
        for (let vertex of face) {
            updated_fv.push(vertex + base_node_count)
        }
        base.faces_vert.push(updated_fv);
    }

    for (let normal of roof.normals) {
        base.normals.push(normal)
    }

    return base
}

export function make_hyperrect(lens: [number, number, number, number],
                               position: Float32Array, orientation: number[],
                               rotation_speed: number[]): Shape {
    // Make a 4d hypercube.
    const coords = [
        // Front inner
        [-1., -1., -1., -1.],
        [1., -1., -1., -1.],
        [1., 1., -1., -1.],
        [-1., 1., -1., -1.],

        // Back inner
        [-1., -1., 1., -1.],
        [1., -1., 1., -1.],
        [1., 1., 1., -1.],
        [-1., 1., 1., -1.],

        // Front outer
        [-1., -1., -1., 1.],
        [1., -1., -1., 1.],
        [1., 1., -1., 1.],
        [-1., 1., -1., 1.],

        // Back outer
        [-1., -1., 1., 1.],
        [1., -1., 1., 1.],
        [1., 1., 1., 1.],
        [-1., 1., 1., 1.],
    ];

    let nodes = new Map()
    for (let id=0; id < coords.length; id++) {
        let coord = coords[id]
        nodes.set(id, new Node2(new Float32Array([coord[0] * lens[0]/2, coord[1] * lens[1]/2,
            coord[2] * lens[2]/2, coord[3] * lens[3]/2])))
    }

    // Divide the vertex position by its length to make normalized vectors.
    // The distance from the center to a corner.
    let d = Math.sqrt(Math.pow(lens[0] / 2, 2) + Math.pow(lens[1], 2) +
        Math.pow(lens[2] / 2., 2) + Math.pow(lens[3] / 2., 2))

    let edges = [
        // Front inner
        new Edge(0, 1),
        new Edge(1, 2),
        new Edge(2, 3),
        new Edge(3, 0),

        // Back inner
        new Edge(4, 5),
        new Edge(5, 6),
        new Edge(6, 7),
        new Edge(7, 4),

        // Connect front to back inner
        new Edge(0, 4),
        new Edge(1, 5),
        new Edge(2, 6),
        new Edge(3, 7),

        // Front outer
        new Edge(8, 9),
        new Edge(9, 10),
        new Edge(10, 11),
        new Edge(11, 8),

        // Back Outer
        new Edge(12, 13),
        new Edge(13, 14),
        new Edge(14, 15),
        new Edge(15, 12),

        // Connect front to back outer
        new Edge(8, 12),
        new Edge(9, 13),
        new Edge(10, 14),
        new Edge(11, 15),

        // Connect front inner to front outer
        new Edge(0, 8),
        new Edge(1, 9),
        new Edge(2, 10),
        new Edge(3, 11),

        // Connect back inner to back outer
        new Edge(4, 12),
        new Edge(5, 13),
        new Edge(6, 14),
        new Edge(7, 15),
    ]

    const faces: Face[] = []
    // Drawing a picture helps!
    const faces_vert = [  // Vertex indices for each face.
        [0, 1, 2, 3],  // Front inner
        [4, 5, 6, 7],  // Back inner
        [3, 2, 6, 7],  // Top inner
        [0, 1, 5, 4],  // Bottom inner
        [0, 4, 7, 3],  // Left inner
        [1, 5, 6, 2],  // Right inner

        [8, 9, 10, 11],  // Front outer
        [12, 13, 14, 15],  // Back outer
        [11, 10, 14, 15],  // Top outer
        [8, 9, 13, 12],  // Bottom outer
        [8, 12, 15, 11],  // Left outer
        [9, 13, 14, 10],  // Right outer

        [8, 9, 1, 0],  // Front bottom
        [12, 13, 5, 4],  // Back bottom
        [12, 8, 0, 4],  // Left bottom
        [9, 13, 5, 1],  // Right bottom

        [11, 10, 2, 3],  // Front top
        [15, 14, 6, 7],  // Back top
        [15, 11, 3, 7],  // Left top
        [14, 10, 2, 6],  // Right top

        [11, 8, 0, 3],  // Left forward
        [15, 12, 4, 7],  // Left back
        [10, 9, 1, 2],  // Right forward
        [14, 13, 5, 6],  // Right back
    ]

    const normals = [  // todo QC this; it's a guess.
        [0., 0., 1., 0.],
        [0., 0., -1., 0.],
        [0., 1., 0., 0.],
        [0., -1., 0., 0.],
        [-1., 0., 0., 0.],
        [1., 0., 0., 0.],
        [0., 0., 0., 1.],
        [0., 0., 0., -1.],

        [0., 0., 1., 0.],
        [0., 0., -1., 0.],
        [0., 1., 0., 0.],
        [0., -1., 0., 0.],
        [-1., 0., 0., 0.],
        [1., 0., 0., 0.],
        [0., 0., 0., 1.],
        [0., 0., 0., -1.],

        [0., 0., 1., 0.],
        [0., 0., -1., 0.],
        [0., 1., 0., 0.],
        [0., -1., 0., 0.],
        [-1., 0., 0., 0.],
        [1., 0., 0., 0.],
        [0., 0., 0., 1.],
        [0., 0., 0., -1.],
    ]

    return new Shape(nodes, edges, faces, faces_vert, normals, position, orientation, rotation_speed)
}

export function make_hypercube(side_len: number,
                               position: Float32Array, orientation: number[],
                               rotation_speed: number[]): Shape {
    // Convenience function.
    return make_hyperrect([side_len, side_len, side_len, side_len],
        position, orientation, rotation_speed)
}

export function make_origin(len: number, position: Float32Array,
                            orientation: number[], rotation_speed: number[]): Shape {
    // A 4-dimensional cross, for marking the origin.
    const coords = [
        [-1., 0., 0., 0.],
        [1., 0., 0., 0.],
        [0., -1., 0., 0.],
        [0., 1., 0., 0.],

        [0., 0., -1., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., -1.],
        [0., 0., 0., 1.],
    ];

    let nodes = new Map()
    for (let id=0; id < coords.length; id++) {
        let coord = coords[id]
        // todo should have better vector arithmetic
        nodes.set(id, new Node2(new Float32Array([coord[0] * len, coord[1] * len,
            coord[2] * len, coord[3] * len])))
    }

    const edges = [
        new Edge(0, 1),
        new Edge(2, 3),
        new Edge(4, 5),
        new Edge(6, 7),
    ];

    const normals = [  // todo wrong!!
        [0., 0., 1., 0.],
        [0., 0., -1., 0.],
        [0., 1., 0., 0.],
        [0., -1., 0., 0.],
        [-1., 0., 0., 0.],
        [1., 0., 0., 0.],
        [0., 0., 0., 1.],
        [0., 0., 0., -1.],
    ]

    return new Shape(nodes, edges, [], [], normals, position, orientation, rotation_speed)
}
//
// // These functions generate Perlin noise. Taken from https://en.wikipedia.org/wiki/Perlin_noise
// function lerp(a0: number, a1: number, w: number): number {
//     // Function to linearly interpolate between a0 and a1
//     // Weight w should be in the range [0.0, 1.0]
//     return (1.0 - w)*a0 + w*a1;
// }
//
//
// function dotGridGradient(ix: number, iy: number, x: number, y: number): number {
//     // Computes the dot product of the distance and gradient vectors.
//     // Precomputed (or otherwise) gradient vectors at each grid node
//     extern float Gradient[IYMAX][IXMAX][2];
//
//     // Compute the distance vector
//     float dx = x - (float)ix;
//     float dy = y - (float)iy;
//
//     // Compute the dot-product
//     return (dx*Gradient[iy][ix][0] + dy*Gradient[iy][ix][1]);
// }
//
// // Compute Perlin noise at coordinates x, y
// float perlin(float x, float y) {
//
//     // Determine grid cell coordinates
//     int x0 = int(x);
//     int x1 = x0 + 1;
//     int y0 = int(y);
//     int y1 = y0 + 1;
//
//     // Determine interpolation weights
//     // Could also use higher order polynomial/s-curve here
//     float sx = x - (float)x0;
//     float sy = y - (float)y0;
//
//     // Interpolate between grid point gradients
//     float n0, n1, ix0, ix1, value;
//     n0 = dotGridGradient(x0, y0, x, y);
//     n1 = dotGridGradient(x1, y0, x, y);
//     ix0 = lerp(n0, n1, sx);
//     n0 = dotGridGradient(x0, y1, x, y);
//     n1 = dotGridGradient(x1, y1, x, y);
//     ix1 = lerp(n0, n1, sx);
//     value = lerp(ix0, ix1, sy);
//
//     return value;
// }

export function makeTerrain(dims: [number, number], res: number,
                            heightMap: number[][], spissitudeMap: number[][],
                            position: Float32Array): Shape {
    // Make a triangle-based terrain mesh.  dims is an [x, z] tuple.
    // We could make a 4d terrain too... id a volume of u-mappings... or have
    // u and y mappings for each x/z point...
    // dims refers to the size of the terrain. res is the number of cells
    // dividing our terrain in each direction. Perhaps replace this argument with
    // something more along the traditional def of resolution?

    // Note: When visually setting up a heighmap array, the z position
    // appears backwards from what you might expect.

    let nodes = new Map()
    let id = 0
    // Instantiate x and like this so the center of the mesh is at the
    // position argument.
    let z
    let height, spissitude
    let x = -dims[0] / 2.
    for (let i=0; i < res; i++) {  // x
        z = -dims[1] / 2.
        for (let j=0; j < res; j++) {  // z
            height = heightMap[i][j]
            spissitude = spissitudeMap[i][j]
            if (isNaN(height) || isNaN(spissitude)) {
                throw "Missing value(s) in heightmap or spissitude grid."
            }
            // You could change which planes this is over by rearranging
            // these node points.
            nodes.set(id, new Node2(new Float32Array([
                x,
                height,
                z,
                spissitude,
            ])))
            z += dims[1] / res
            id += 1
        }
        x += dims[0] / res
    }

    let edges = [];
    let faces: Face[] = [];  // todo later
    let row_adder = 0;

    // Faces for this terrain are triangles. Don't try to make square faces;
    // they'd really have creases down a diagonal.
    let faces_vert = [];
    // todo need front and right edges of overall terrain.
    for (let i=0; i < res - 1; i++) {
        for (let j=0; j < res - 1; j++) {
            edges.push(new Edge(row_adder + j, row_adder + j + 1));  // edges across constant x
            edges.push(new Edge(row_adder + j, row_adder + j + res));  // edges across constant z

            // two face triangles per grid square. There are two ways to split
            // up the squares into triangles; picking one arbitrarily.
            faces_vert.push([  // shows front right
                row_adder + j,  // back left
                row_adder + j + 1,  // back right
                row_adder + j + res + 1  // front left
            ]);
            faces_vert.push([  // shows front left  not j + res, not j
                row_adder + j,
                row_adder + j + res,  // front right
                row_adder + j + res + 1  // front left
            ]);
        }
        row_adder += res;
    }

    const normals = [  // wrong!
        [0., 0., 1., 0.],
    ]

    return new Shape(nodes, edges, faces, faces_vert, normals, position,
        [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
}

export function makeHypergrid(dims: [number, number, number],
                              res: number,
                              spissitudeMap: number[][][],
                              position: Float32Array): Map<number, Shape> {
    // Position is the center.
    // todo incorporate position.
    let result = new Map()

    let y, z
    let x = -dims[0] / 2.
    for (let i=0; i < res; i++) {  // x
        y = -dims[1] / 2.
        for (let j=0; j < res; j++) {  // y
            z = -dims[2] / 2.
            for (let k=0; k < res; k++) {  // z
                result.set(
                    Math.pow(res, 2) * i + res * j + k,
                    makeCube(.5, new Float32Array([x, y, z, spissitudeMap[i][j][k]]),
                        [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
                )
                z += dims[2] / res
            }
            y += dims[1] / res
        }
        x += dims[0] / res
    }
    return result
}

export function makeCubeHypergrid4d(dims: [number, number, number, number],
                                    res: number,
                                    spissitudeMap: number[][][][],
                                    position: Float32Array): Map<number, Shape> {
    // Position is the center.
    // todo incorporate position.
    let result = new Map()

    let y, z, u
    let x = -dims[0] / 2.
    for (let i=0; i < res; i++) {  // x
        y = -dims[1] / 2.
        for (let j=0; j < res; j++) {  // y
            z = -dims[2] / 2.
            for (let k=0; k < res; k++) {  // z
                u = -dims[3] / 2.
                for (let l=0; l < res; l++) {
                    result.set(
                        Math.pow(res, 3) * i + Math.pow(res, 2) * j + res * k + l,
                        // todo hypercube?
                        makeCube(.5, new Float32Array([x, y, z, u]),
                            [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
                    )
                }
                z += dims[2] / res
            }
            y += dims[1] / res
        }
        x += dims[0] / res
    }
    return result
}

export function make5Cell(radius: number, position: Float32Array, orientation: number[],
                          rotation_speed: number[]): Shape {
    // AKA pentachoron, or tetrahedral pyramid
    // https://en.wikipedia.org/wiki/5-cell
    // todo add to Rust.
    // radius is the distance from the center; all points lie on a hypersphere of
    // with the specified radius. Edge length: sqrt(8/3)
    const coords = [
        [-Math.sqrt(2./3.), -1./3., -Math.sqrt(2./9.), 0.],  // left base
        [Math.sqrt(2./3.), -1./3., -Math.sqrt(2./9.), 0.],  // right base
        [0., -1./3., Math.sqrt(8./9.), 0.],  // Back base
        [0., 1., 0., 0.],  // Top
        [0., 0., 0., 1.],  // middle
    ]

    let nodes = new Map()
    for (let id=0; id < coords.length; id++) {
        let coord = coords[id];
        nodes.set(id, new Node2(new Float32Array([coord[0] * radius/2., coord[1] * radius/2.,
            coord[2] * radius/2., coord[3] * radius/2.])))
    }

    const edges = [
        // Base
        new Edge(0, 1),
        new Edge(1, 2),
        new Edge(2, 0),

        // Connect base to top
        new Edge(0, 3),
        new Edge(1, 3),
        new Edge(2, 1),

        // Connect center to corners
        new Edge(4, 0),
        new Edge(4, 1),
        new Edge(4, 2),
        new Edge(4, 3),
    ]

    let faces = [
        // In same order as faces_vert below
        new Face([edges[0], edges[1], edges[2]]),
        new Face([edges[0], edges[4], edges[3]]),
        new Face([edges[1], edges[5], edges[4]]),
        new Face([edges[2], edges[3], edges[5]]),

        new Face([edges[6], edges[0], edges[7]]),
        new Face([edges[7], edges[1], edges[8]]),
        new Face([edges[8], edges[2], edges[6]]),

        new Face([edges[6], edges[3], edges[9]]),
        new Face([edges[7], edges[4], edges[9]]),
        new Face([edges[8], edges[5], edges[9]]),
    ]

    const faces_vert = [  // Vertex indices for each face.
        [0, 1, 2], // Base
        [0, 1, 3],  // Front
        [1, 2, 3],  // Right
        [2, 0, 3],  // Left

        [4, 0, 1],  // Center front
        [4, 1, 2],  // Center right
        [4, 2, 0],  // Center left

        [4, 0, 3],  // Center left top
        [4, 1, 3],  // Center right top
        [4, 2, 3],  // Center back top
    ]

    const normals = [  // todo fix this!!
        [0., 0., 1., 0.],
        [0., 0., -1., 0.],
        [0., 1., 0., 0.],
        [0., -1., 0., 0.],

        [-1., 0., 0., 0.],
        [1., 0., 0., 0.],
        [0., 0., 0., 1.],

        [0., 0., 0., -1.],
        [0., 0., 0., -1.],
        [0., 0., 0., -1.],
    ]

    return new Shape(nodes, edges, faces, faces_vert, normals, position,
        orientation, rotation_speed)
}

export function make_skybox(len: number, position: Float32Array): Shape {
    return makeCube(len, position, [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
}