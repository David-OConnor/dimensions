// This file mirrors shape_maker.rs  Uses _ naming conventions in some case for
// interoperability. Uses semicolons in many places, other like other parts
// of the frontend.

import {Shape, Node2, Edge, Face, Vec5} from "./interfaces";

export function make_box(lens: [number, number, number],
                         position: Vec5, orientation: number[],
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
        nodes.set(id, new Node2(new Vec5([coord[0] * lens[0]/2, coord[1] * lens[1]/2,
            coord[2] * lens[2]/2, coord[3]])));
    }

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

    return new Shape(nodes, edges, faces, faces_vert, position,
        orientation, rotation_speed)
}

export function make_cube(side_len: number,
                          position: Vec5, orientation: number[],
                          rotation_speed: number[]): Shape {
    // Convenience function.
    // We'll still treat the center as the center of the base portion.
    return make_box([side_len, side_len, side_len], position, orientation, rotation_speed)
}

export function make_rectangular_pyramid(lens: [number, number, number],
                                         position: Vec5, orientation: number[],
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
        let coord = coords[id];
        nodes.set(id, new Node2(new Vec5([coord[0] * lens[0]/2, coord[1] * lens[1]/2,
            coord[2] * lens[2]/2, coord[3]/2])));
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

    return new Shape(nodes, edges, faces, faces_vert, position, orientation, rotation_speed)
}

export function make_house(lens: [number, number, number],
                           position: Vec5,
                           orientation: number[],
                           rotation_speed: number[]): Shape {
    const empty_array = [0., 0., 0., 0., 0., 0.];

    // We'll modify base in-place, then return it.
    let base = make_box(lens, position, orientation, rotation_speed);

    let roof = make_rectangular_pyramid(
        // Let the roof overhang the base by a little.
        // Make the roof height a portion of the base height.
        [lens[0] * 1.2, lens[1] / 3., lens[2] * 1.2],
        new Vec5([0, 0, 0, 0]), empty_array, empty_array
    );

    // Now that we've made the shapes, recompose them to be one shape.
    // todo make this a separate, (reusable) func?1
    const base_node_count = base.nodes.size;

    roof.nodes.forEach(
        (node, id, map) => {
            let lifted_node = node;
            // Raise the roof
            lifted_node.a.vals[1] += lens[1] / 2.
            base.nodes.set(
                id + base_node_count,
                lifted_node
            )
        }
    )

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
                               position: Vec5, orientation: number[],
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
        nodes.set(id, new Node2(new Vec5([coord[0] * lens[0]/2, coord[1] * lens[1]/2,
            coord[2] * lens[2]/2, coord[3] * lens[3]/2])))
    }

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

    return new Shape(nodes, edges, faces, faces_vert, position, orientation, rotation_speed)
}

export function make_hypercube(side_len: number,
                               position: Vec5, orientation: number[],
                               rotation_speed: number[]): Shape {
    // Convenience function.
    return make_hyperrect([side_len, side_len, side_len, side_len],
        position, orientation, rotation_speed)
}

export function make_origin(len: number, position: Vec5,
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
        nodes.set(id, new Node2(new Vec5([coord[0] * len, coord[1] * len,
            coord[2] * len, coord[3] * len])))
    }
    const edges = [
        new Edge(0, 1),
        new Edge(2, 3),
        new Edge(4, 5),
        new Edge(6, 7),
    ];

    return new Shape(nodes, edges, [], [], position, orientation, rotation_speed)
}

export function make_terrain(dims: [number, number], res: [number, number],
                             heightMap: number[][], spissitudeMap: number[][], position: Vec5): Shape {
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
    for (let i=0; i < res[0]; i++) {  // x
         z = -dims[1] / 2.
        for (let j=0; j < res[1]; j++) {  // z
            height = heightMap[i][j]
            spissitude = spissitudeMap[i][j]
            if (isNaN(height) || isNaN(spissitude)) {
                throw "Missing value(s) in heightmap or spissitude grid."
            }
            // todo you could change which planes this is over by rearranging
            // todo these node points.
            nodes.set(id, new Node2(new Vec5([
                x,
                height,
                z,
                spissitude,
            ])))
            z += dims[1] / res[1]
            id += 1
        }
        x += dims[0] / res[0]
    }

    let edges = [];
    let faces_vert = [];
    let row_adder = 0;

    // todo need front and right edges of overall terrain.
    for (let i=0; i < res[0] - 1; i++) {
        for (let j=0; j < res[1] - 1; j++) {
            edges.push(new Edge(row_adder + j, row_adder + j + 1));  // edges across constant x
            edges.push(new Edge(row_adder + j, row_adder + j + res[0]));  // edges across constant z

            // Build from the back-left corner of each face.
            faces_vert.push([
                row_adder + j,  // back left
                row_adder + j + 1,  // back right
                row_adder + j + res[0],  // front right
                row_adder + j + res[0] + 1  // front left
            ]);
        }
        row_adder += res[0];
    }

    let faces: Face[] = [];  // todo later

    return new Shape(nodes, edges, faces, faces_vert, position,
        [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
}

// export function make_cube_hypergrid(dims: [number, number, number],
//                                     spissitudeMap: number[][][],
//                                     position: Vec5): Map<number, Shape> {
//
// }

export function make_skybox(len: number, position: Vec5): Shape {
    return make_cube(len, position, [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
}