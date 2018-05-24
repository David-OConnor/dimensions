export class Node2 {
    a: number[]
    constructor(a: number[]) {
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
}

export class Camera {
    // See Rust's Camera struct for information.
    position: number[]
    θ_3d: number[]
    θ_4d: number[]
    fov_vert: number
    fov_hor: number
    fov_strange: number
    clip_near: number
    clip_far: number
    clip_strange: number

    constructor(position: number[], θ_3d: number[], θ_4d: number[],
                fov_vert: number, fov_hor: number, fov_strange: number,
                clip_near: number, clip_far: number, clip_strange: number) {
        this.position = position
        this.θ_3d = θ_3d
        this.θ_4d = θ_4d
        this.fov_vert = fov_vert
        this.fov_hor = fov_hor
        this.fov_strange = fov_strange
        this.clip_near = clip_near
        this.clip_far = clip_far
        this.clip_strange = clip_strange

    }
}

export class Shape {
    nodes: Map<number, Node2>
    edges: Edge[]
    faces: Face[]
    position: number[]
    scale: number
    orientation: number[]
    rotationSpeed: number[]

    constructor(nodes: Map<number, Node2>, edges: Edge[], faces: Face[],
                position: number[], scale: number, orientation: number[],
                rotationSpeed: number[]) {
        this.nodes = nodes
        this.edges = edges
        this.faces = faces
        this.position = position
        this.scale = scale
        this.orientation = orientation
        this.rotationSpeed = rotationSpeed
    }
}

export interface ShapeArgs {
    // Ref struct of same name in lib.rs.
    name: string
    lens: number[]
    position: number[]
    scale: number
    orientation: number[]
    rotation_speed: number[]
}

export interface MainState {
    shapes: Map<number, Shape>
}

export interface ProgramInfo {
    program: any
    attribLocations: any
    uniformLocations: any
}