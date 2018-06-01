import {NdArray} from 'numjs'

export class Node2 {
    a: NdArray
    constructor(a: NdArray) {
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
    position: NdArray
    scale: number
    orientation: NdArray
    rotation_speed: NdArray

    constructor(nodes: Map<number, Node2>, edges: Edge[], faces: Face[],
                position: NdArray, scale: number, orientation: NdArray,
                rotation_speed: NdArray) {
        this.nodes = nodes
        this.edges = edges
        this.faces = faces
        this.position = position
        this.scale = scale
        this.orientation = orientation
        this.rotation_speed = rotation_speed
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

export class Camera {
    // See Rust's Camera struct for information.
    position: NdArray
    θ_3d: NdArray
    θ_4d: NdArray
    fov: number
    aspect: number
    aspect_4: number
    near: number
    far: number
    strange: number

    constructor(position: NdArray, θ_3d: NdArray, θ_4d: NdArray,
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