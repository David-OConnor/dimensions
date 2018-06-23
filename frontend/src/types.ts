// Similar to types.rs.

export class Node2 {
    a: Float32Array
    constructor(a: Float32Array) {
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
    edges: Edge[]  // todo Edges is currently unused.
    faces: Face[]  // todo faces is currently unused.
    // faces_vert corresponds to faces, but uses vertices rather than edges.  This corresponds
    // to WebGl's implementation of faces.  We can't implicitly create this by
    // iterating over over faces/edges due to edges being used in multiple directions.
    // Vertex indices for each face.
    faces_vert: number[][]
    // normals is in the style of faces_vert; eg duplicate the vertices for each
    // face. Vut instead of
    normals: number[][]
    position: Float32Array
    scale: number
    orientation: number[]
    rotation_speed: number[]
    // PerFaceVertices and tris are similar; in the indexed order. One is straight
    // from the faces, the other is divided into tris.
    perFaceVertices: Float32Array
    tris: number[]

    constructor(nodes: Map<number, Node2>, edges: Edge[], faces: Face[],
                faces_vert: number[][], normals: number[][],
                position: Float32Array, orientation: number[],
                rotation_speed: number[]) {
        this.nodes = nodes
        this.edges = edges
        this.faces = faces
        this.faces_vert = faces_vert
        this.normals = normals
        this.position = position
        this.scale = 1
        this.orientation = orientation
        this.rotation_speed = rotation_speed
        this.perFaceVertices = this.makePerFaceVertices()
        this.tris = []
    }
    //
    // getPerFaceVertices(): Float32Array {
    //     // get cached triangles if avail; if not, create and cache.
    //     if (!this.perFaceVertices.length) {
    //         this.makePerFaceVertices()
    //     }
    //     return this.perFaceVertices
    // }

    makePerFaceVertices(): Float32Array {
        let result = [], vertex
        for (let face of this.faces_vert) {
            for (let vertex_i of face) {
                vertex = (this.nodes.get(vertex_i) as any).a
                for (let i=0; i < 4; i++) {  // Iterate through each coord.
                    result.push(vertex[i])
                }
            }
        }
        return new Float32Array(result)
    }

    get_tris(): number[] {
        // get cached triangles if avail; if not, create and cache.
        if (!this.tris.length) {
            this.make_tris()
        }
        return this.tris
    }

    make_tris() {
        // Divide faces into triangles of indices. These indices aren't of node
        // ids; rather of cumulative node ids; eg how they'll appear in an index buffer.
        // Result is a 1d array; Float32array-style.
        let result = []
        let current_i = 0
        for (let face of this.faces_vert) {
            if (face.length === 3) {
                // Only one triangle.
                result.push(current_i)
                result.push(current_i + 1)
                result.push(current_i + 2)
            } else if (face.length === 4) {
                // First triangle
                result.push(current_i)
                result.push(current_i + 1)
                result.push(current_i + 2)
                // Second triangle
                result.push(current_i)
                result.push(current_i + 2)
                result.push(current_i + 3)
            } else if (face.length === 2) {
                throw "Faces must have len 3 or more."
            } else {
                throw "Error: Haven't implemented faces with vertex counds higher than four."
            }
            current_i += face.length
        }
        this.tris = result
    }

    numFaceVerts(): number {
        // Find the number of vertices used in drawing faces.  Ie for a box,
        // it's 6 faces x 4 vertices/face.
        return this.faces_vert.reduce((acc, face) => acc + face.length, 0)
    }
}

export class Camera {
    // See Rust's Camera struct for information.
    position: Float32Array
    θ: number[]
    fov: number
    aspect: number
    aspect_4: number
    near: number
    far: number
    strange: number

    constructor(position: Float32Array, θ: number[],
                fov: number, aspect: number, aspect_4: number,
                near: number, far: number, strange: number) {
        this.position = position
        this.θ = θ
        this.fov = fov
        this.aspect = aspect
        this.aspect_4 = aspect_4
        this.near = near
        this.far = far
        this.strange = strange
    }
}

export interface Scene {
    id: number,
    shapes: Map<number, Shape>,
    camStart: Camera,
    camType: string,  // 'single', 'fps', or 'ffree'
    ambientLightColor: Float32Array,
    ambientLightDirection: Float32Array,
    colorMax: number, // distance thresh for max 4d-color indicator.
}

export interface MainState {
    shapes: Map<number, Shape>
    scene: number
    shape: number
}

export interface ProgramInfo {
    program: any
    attribLocations: any
    uniformLocations: any
}