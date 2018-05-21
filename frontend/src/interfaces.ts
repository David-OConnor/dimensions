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

export interface MainState {
    shapes: Map<number, Shape>
}

export interface ProgramInfo {
    program: any
    attribLocations: any
    uniformLocations: any
}