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

export class Shape {
    nodes: Map<number, Node2>
    edges: Edge[]
    constructor(nodes: Map<number, Node2>, edges: Edge[]) {
        this.nodes = nodes
        this.edges = edges
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