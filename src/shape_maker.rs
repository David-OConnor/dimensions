use ndarray::prelude::*;

use types::{Node, Edge, Shape};

// todo you're going to get duplicate node ids here unless you deal with it!

pub fn make_box(start: Array1<f64>, x_len: f64, 
                 y_len: f64, z_len: f64, id: i32) -> Shape {
    // Make a rectangular prism.  Use negative lengths to draw in the opposite
    // direction.

    let nodes = vec![
        // Front
        Node{a: array![start[0], start[1], start[2]], id: 0},
        Node{a: array![start[0] + x_len, start[1], start[2]], id: 1},
        Node{a: array![start[0] + x_len, start[1] + y_len, start[2]], id: 2},
        Node{a: array![start[0], start[1] + y_len, start[2]], id: 3},

        Node{a: array![start[0], start[1], start[2] + z_len], id: 4},
        Node{a: array![start[0] + x_len, start[1], start[2] + z_len], id: 5},
        Node{a: array![start[0] + x_len, start[1] + y_len, start[2] + z_len], id: 6},
        Node{a: array![start[0], start[1] + y_len, start[2] + z_len], id: 7},
    ];

    let edges = vec![
        // Front
        Edge {node1: 0, node2: 1},
        Edge {node1: 1, node2: 2},
        Edge {node1: 2, node2: 3},
        Edge {node1: 3, node2: 0},
        
        // Back
        Edge {node1: 4, node2: 5},
        Edge {node1: 5, node2: 6},
        Edge {node1: 6, node2: 7},
        Edge {node1: 7, node2: 4},
        
        // Bridger
        Edge {node1: 0, node2: 4},
        Edge {node1: 1, node2: 5},
        Edge {node1: 2, node2: 6},
        Edge {node1: 3, node2: 7},
    ];

    Shape {nodes: nodes, edges: edges, id}
}

pub fn make_rectangular_pyramid(start: Array1<f64>, x_len: f64, 
                                y_len: f64, z_len: f64, id: i32) -> Shape {

    let nodes = vec![
        // Front
        Node{a: array![start[0], start[1], start[2]], id: 0},
        Node{a: array![start[0] + x_len, start[1], start[2]], id: 1},
        Node{a: array![start[0] + x_len, start[1] + y_len, start[2]], id: 2},
        Node{a: array![start[0], start[1] + y_len, start[2]], id: 3},

        // Top
        Node{a: array![start[0] + x_len / 2., start[1] + y_len / 2., start[2] + z_len], id: 4},

    ];

    let edges = vec![
        // Front
        Edge {node1: 0, node2: 1},
        Edge {node1: 1, node2: 2},
        Edge {node1: 2, node2: 3},
        Edge {node1: 3, node2: 0},

        // Bridger
        Edge {node1: 0, node2: 4},
        Edge {node1: 1, node2: 4},
        Edge {node1: 2, node2: 4},
        Edge {node1: 3, node2: 4},
    ];

    Shape {nodes: nodes, edges: edges, id}
}


pub fn make_cube(start: Array1<f64>, side_len: f64, id: i32) -> Shape {
    // Convenience function.
    make_box(start, side_len, side_len, side_len, id)
}

pub fn make_hypercube(start: Array1<f64>, side_len: f64, id: i32) -> Shape {
    // Make a 4d hypercube.

     let nodes = vec![
        // Front inner
        Node {a: array![0., 0., 0., 0.], id: 0},
        Node {a: array![0., 1., 0., 0.], id: 1},
        Node {a: array![1., 1., 0., 0.], id: 2},
        Node {a: array![1., 0., 0., 0.], id: 3},
        
        // Back inner
        Node {a: array![0., 0., 1., 0.], id: 4},
        Node {a: array![0., 1., 1., 0.], id: 5},
        Node {a: array![1., 1., 1., 0.], id: 6},
        Node {a: array![1., 0., 1., 0.], id: 7},
        
        // Front outer
        Node {a: array![0., 0., 0., 1.], id: 8},
        Node {a: array![0., 1., 0., 2.], id: 9},
        Node {a: array![1., 1., 0., 3.], id: 10},
        Node {a: array![1., 0., 0., 4.], id: 11},
        
        // Back outer
        Node {a: array![0., 0., 1., 5.], id: 12},
        Node {a: array![0., 1., 1., 6.], id: 13},
        Node {a: array![1., 1., 1., 7.], id: 14},
        Node {a: array![1., 0., 1., 8.], id: 15}
    ];

    let edges = vec![
        // Front inner
        Edge {node1: 0, node2: 1},
        Edge {node1: 1, node2: 2},
        Edge {node1: 2, node2: 3},
        Edge {node1: 3, node2: 0},
        
        // Back inner
        Edge {node1: 4, node2: 5},
        Edge {node1: 5, node2: 6},
        Edge {node1: 6, node2: 7},
        Edge {node1: 7, node2: 4},
        
        // Connect front to back inner
        Edge {node1: 0, node2: 4},
        Edge {node1: 1, node2: 5},
        Edge {node1: 2, node2: 6},
        Edge {node1: 3, node2: 7},

        // Front outer
        Edge {node1: 8, node2: 9},
        Edge {node1: 9, node2: 10},
        Edge {node1: 10, node2: 11},
        Edge {node1: 11, node2: 8},
        
        // Back outer
        Edge {node1: 12, node2: 13},
        Edge {node1: 13, node2: 14},
        Edge {node1: 14, node2: 15},
        Edge {node1: 15, node2: 12},
        
        // Connect front to back outer
        Edge {node1: 8, node2: 12},
        Edge {node1: 9, node2: 13},
        Edge {node1: 10, node2: 14},
        Edge {node1: 11, node2: 15},

        // Connect front inner to front outer
        Edge {node1: 0, node2: 8},
        Edge {node1: 1, node2: 9},
        Edge {node1: 2, node2: 10},
        Edge {node1: 3, node2: 11},
        
        // Connect back inner to back outer
        Edge {node1: 4, node2: 12},
        Edge {node1: 5, node2: 13},
        Edge {node1: 6, node2: 14},
        Edge {node1: 7, node2: 15},
    ];

    Shape {nodes: nodes, edges: edges, id}
}