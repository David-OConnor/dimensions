
// Note: This custom matrix and vector types shouldn't be needed; but haven't
// found a suitable npm package for Vec5s. (Existing ones seem broken).
export class Vec5 {
    vals: number[]

    constructor(vals: number[]) {
        if (vals.length === 4) {
            this.vals = vals
            this.vals.push(1)  // Augment
        } else if (vals.length === 5) {
            this.vals = vals
        } else {
            throw "Must have 4 or 5 elements."
        }

    }

    add(other: Vec5): Vec5 {
        const newVals = [
            this.vals[0] + other.vals[0],
            this.vals[1] + other.vals[1],
            this.vals[2] + other.vals[2],
            this.vals[3] + other.vals[3],
            this.vals[4] + other.vals[4]
        ]
        return new Vec5(newVals)
    }

    toGl(): Float32Array {
        // Convert to the linear, 4-element Float32Array WebGl uses.
        // u is discarded.
        return new Float32Array([this.vals[0], this.vals[1], this.vals[2], 1])
    }
}

export class Array5 {
    vals: number[][]

    constructor(vals: number[][]) {
        if (vals.length !== 5) {
            throw ("Array5 must be 5x5")
        }

        this.vals = vals
    }

    dotM(other: Array5): Array5 {
        // Dot with another 5x5 matrix.
        let result = [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0]
        ]

        for (let i0=0; i0 < 5; i0++) {
            for (let i1 = 0; i1 < 5; i1++) {
                for (let j = 0; j < 5; j++) {
                    result[i0][i1] += this.vals[i0][j] * other.vals[j][i1]
                }

            }
        }
        return new Array5(result)
    }

    dotV(other: Vec5): Vec5 {
        let result = [0, 0, 0, 0, 0]

        for (let i=0; i < 5; i++) {
            for (let j=0; j < 5; j++) {
                result[i] += this.vals[i][j] + other.vals[j]
            }
        }

        return new Vec5(result)
    }

    toGl(): Float32Array {
        // Convert to the linear, 16-element (4x4) Float32Array WebGl uses.
        // u is discarded.
        return new Float32Array([
            this.vals[0][0], this.vals[0][1], this.vals[0][2], this.vals[0][4],
            this.vals[1][0], this.vals[1][1], this.vals[1][2], this.vals[1][4],
            this.vals[2][0], this.vals[2][1], this.vals[2][2], this.vals[2][4],
            // The 4th row, corresponding to u, is discarded. As is the fourth col.
            this.vals[4][0], this.vals[4][1], this.vals[4][2], this.vals[4][4]
        ])
    }
}

export class Node2 {
    a: Vec5
    constructor(a: Vec5) {
        this.a = a
    }
}

export class Edge {
    node0: number
    node1: number
    constructor(node0: number, node1: number) {
        this.node0 = node0
        this.node1 = node1
    }
}

export class Face {
    edges: Edge[]
    constructor(edges: Edge[]) {
        this.edges = edges
    }
}

export class Shape {
    nodes: Map<number, Node2>
    edges: Edge[]
    faces: Face[]
    position: Vec5
    scale: number
    orientation: number[]
    rotation_speed: number[]

    constructor(nodes: Map<number, Node2>, edges: Edge[], faces: Face[],
                position: Vec5, scale: number, orientation: number[],
                rotation_speed: number[]) {
        this.nodes = nodes
        this.edges = edges
        this.faces = faces
        this.position = position
        this.scale = scale
        this.orientation = orientation
        this.rotation_speed = rotation_speed
    }
}
//
// export interface ShapeArgs {
//     // Ref struct of same name in lib.rs.
//     name: string
//     lens: number[]
//     position: number[]
//     scale: number
//     orientation: number[]
//     rotation_speed: number[]
// }

export class Camera {
    // See Rust's Camera struct for information.
    position: Vec5
    θ_3d: number[]
    θ_4d: number[]
    fov: number
    aspect: number
    aspect_4: number
    near: number
    far: number
    strange: number

    constructor(position: Vec5, θ_3d: number[], θ_4d: number[],
                fov: number, aspect: number, aspect_4: number,
                near: number, far: number, strange: number) {
        this.position = position
        this.θ_3d = θ_3d
        this.θ_4d = θ_4d
        this.fov = fov
        this.aspect = aspect
        this.aspect_4 = aspect_4
        this.near = near
        this.far = far
        this.strange = strange

    }
}

export interface MainState {
    shapes: Map<number, Shape>
}

export interface ProgramInfo {
    program: any
    attribLocations: any
    uniformLocations: any
}