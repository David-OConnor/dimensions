// This file mirrors shape_maker.rs  Uses _ naming conventions in some case for
// interoperability. Uses semicolons in many places, other like other parts
// of the frontend.

import {Shape, Node2, Edge, Face} from "./types"

export function make_box(lens: [number, number, number],
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

    let normals = new Map()
    normals.set(0, new Float32Array([-lens[0] / d, -lens[1] / d, -lens[2] / d, 0.]))
    normals.set(1, new Float32Array([lens[0] / d, -lens[1] / d, -lens[2] / d, 0.]))
    normals.set(2, new Float32Array([lens[0] / d, lens[1] / d, -lens[2] / d, 0.]))
    normals.set(3, new Float32Array([-lens[0] / d, lens[1] / d, -lens[2] / d, 0.]))

    normals.set(4, new Float32Array([-lens[0] / d, -lens[1] / d, lens[2] / d, 0.]))
    normals.set(5, new Float32Array([lens[0] / d, -lens[1] / d, lens[2] / d, 0.]))
    normals.set(6, new Float32Array([lens[0] / d, lens[1] / d, lens[2] / d, 0.]))
    normals.set(7, new Float32Array([-lens[0] / d, lens[1] / d, lens[2] / d, 0.]))

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

    return new Shape(nodes, edges, faces, faces_vert, normals, position,
                     orientation, rotation_speed)
}

export function make_cube(side_len: number,
                          position: Float32Array, orientation: number[],
                          rotation_speed: number[]): Shape {
    // Convenience function.
    // We'll still treat the center as the center of the base portion.
    return make_box([side_len, side_len, side_len], position, orientation, rotation_speed)
}

export function make_rectangular_pyramid(lens: [number, number, number],
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
    ];

    let nodes = new Map()
    for (let id=0; id < coords.length; id++) {
        let coord = coords[id]
        nodes.set(id, new Node2(new Float32Array([coord[0] * lens[0]/2, coord[1] * lens[1]/2,
            coord[2] * lens[2]/2, coord[3]/2])))
    }

    // Divide the vertex position by its length to make normalized vectors.
    let d = Math.sqrt(Math.pow(lens[0] / 2, 2) + Math.pow(lens[2] / 2., 2))

    let normals = new Map()
    normals.set(0, new Float32Array([-lens[0] / d, 0., -lens[2] / d, 0.]))
    normals.set(1, new Float32Array([lens[0] / d, 0., -lens[2] / d, 0.]))
    normals.set(2, new Float32Array([lens[0] / d, 0., -lens[2] / d, 0.]))
    normals.set(3, new Float32Array([-lens[0] / d, 0., -lens[2] / d, 0.]))

    normals.set(4, new Float32Array([0., lens[1], 0., 0.]))

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
    ];

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
    ];

    const faces_vert = [  // Vertex indices for each face.
        [0, 1, 2, 3],  // Base
        [0, 1, 4],  // Front
        [1, 2, 4],  // Right
        [2, 3, 4],  // Back
        [3, 0, 4],  // Left
    ];

    return new Shape(nodes, edges, faces, faces_vert, normals, position, orientation, rotation_speed)
}

export function make_house(lens: [number, number, number],
                           position: Float32Array,
                           orientation: number[],
                           rotation_speed: number[]): Shape {
    const empty_array = [0., 0., 0., 0., 0., 0.];

    // We'll modify base in-place, then return it.
    let base = make_box(lens, position, orientation, rotation_speed);

    let roof = make_rectangular_pyramid(
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

    let normals = new Map()
    normals.set(0, new Float32Array([-lens[0] / d, -lens[1] / d, -lens[2] / d, -lens[3] / d]))
    normals.set(1, new Float32Array([lens[0] / d, -lens[1] / d, -lens[2] / d, -lens[3] / d]))
    normals.set(2, new Float32Array([lens[0] / d, lens[1] / d, -lens[2] / d, -lens[3] / d]))
    normals.set(3, new Float32Array([-lens[0] / d, lens[1] / d, -lens[2] / d, -lens[3] / d]))

    normals.set(4, new Float32Array([-lens[0] / d, -lens[1] / d, lens[2] / d, -lens[3] / d]))
    normals.set(5, new Float32Array([lens[0] / d, -lens[1] / d, lens[2] / d, -lens[3] / d]))
    normals.set(6, new Float32Array([lens[0] / d, lens[1] / d, lens[2] / d, -lens[3] / d]))
    normals.set(7, new Float32Array([-lens[0] / d, lens[1] / d, lens[2] / d, -lens[3] / d]))
    
    normals.set(8, new Float32Array([-lens[0] / d, -lens[1] / d, -lens[2] / d, lens[3] / d]))
    normals.set(9, new Float32Array([lens[0] / d, -lens[1] / d, -lens[2] / d, lens[3] / d]))
    normals.set(10, new Float32Array([lens[0] / d, lens[1] / d, -lens[2] / d, lens[3] / d]))
    normals.set(11, new Float32Array([-lens[0] / d, lens[1] / d, -lens[2] / d, lens[3] / d]))

    normals.set(12, new Float32Array([-lens[0] / d, -lens[1] / d, lens[2] / d, lens[3] / d]))
    normals.set(13, new Float32Array([lens[0] / d, -lens[1] / d, lens[2] / d, lens[3] / d]))
    normals.set(14, new Float32Array([lens[0] / d, lens[1] / d, lens[2] / d, lens[3] / d]))
    normals.set(15, new Float32Array([-lens[0] / d, lens[1] / d, lens[2] / d, lens[3] / d]))
    
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
    ];

    const faces: Face[] = [];
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

    ];

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

    let normals = new Map()

    const edges = [
        new Edge(0, 1),
        new Edge(2, 3),
        new Edge(4, 5),
        new Edge(6, 7),
    ];

    return new Shape(nodes, edges, [], [], normals, position, orientation, rotation_speed)
}

export function make_terrain(dims: [number, number], res: number,
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

    // todo: Add this to rust.

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
            // todo you could change which planes this is over by rearranging
            // todo these node points.
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

    let normals = new Map()  // todo

    let edges = [];
    let faces: Face[] = [];  // todo later
    let row_adder = 0;

    let faces_vert = [];
    // todo need front and right edges of overall terrain.
    for (let i=0; i < res - 1; i++) {
        for (let j=0; j < res - 1; j++) {
            edges.push(new Edge(row_adder + j, row_adder + j + 1));  // edges across constant x
            edges.push(new Edge(row_adder + j, row_adder + j + res));  // edges across constant z

            // Build from the back-left corner of each face.
            faces_vert.push([
                row_adder + j,  // back left
                row_adder + j + 1,  // back right
                row_adder + j + res,  // front right
                row_adder + j + res + 1  // front left
            ]);
        }
        row_adder += res;
    }

    return new Shape(nodes, edges, faces, faces_vert, normals, position,
                     [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
}

export function make_cube_hypergrid(dims: [number, number, number],
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
                    make_cube(.5, new Float32Array([x, y, z, spissitudeMap[i][j][k]]),
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

// export function make_hypergrid(dims: [number, number, number],
//                                spissitudeMap: number[][][],
//                                position: Float32Array): Shape {
//
// }

export function make_5cell(radius: number, position: Float32Array, orientation: number[],
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

    let normals = new Map()  // todo

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
        // In same order as faces below
        new Face([edges[0], edges[1], edges[2]]),
        new Face([edges[0], edges[4], edges[3]]),
        new Face([edges[1], edges[5], edges[4]]),
        new Face([edges[2], edges[3], edges[5]]),

        new Face([edges[6], edges[0], edges[7]]),
        new Face([edges[7], edges[1], edges[8]]),
        new Face([edges[18], edges[2], edges[6]]),

        new Face([edges[6], edges[3], edges[9]]),
        new Face([edges[7], edges[4], edges[9]]),
        new Face([edges[8], edges[5], edges[9]]),
    ]

    const faces_vert = [  // Vertex indices for each face.
        [0, 1, 2] , // Base
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

    return new Shape(nodes, edges, faces, faces_vert, normals, position,
        orientation, rotation_speed)
}

export function make_skybox(len: number, position: Float32Array): Shape {
    return make_cube(len, position, [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
}