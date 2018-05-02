use ndarray::prelude::*;

use types::{Node, Edge, Shape}

// todo you're going to get duplicate node ids here unless you deal with it!

pub fn make_cube(start: Array1<f64>, x_len: f64, 
                 y_len: f64, z_len: f64, id: i32) -> Shape {
    // Make a rectangular prism.  Use negative lengths to draw in the opposite
    // direction.

    let nodes = [
        // Front
        Node{a: start, id: 0},
        Node{a: array![start[0] + x_len, start[1], start[2]], id: 1},
        Node{a: array![start[0] + x_len, start[1] + y_len, start[2]], id: 2},
        Node{a: array![start[0], start[1] + y_len, start[2]], id: 3},

        Node{a: array![start[0], start[1], star[2] + z_len], id: 4},
        Node{a: array![start[0] + x_len, start[1], start[2] + z_len], id: 5},
        Node{a: array![start[0] + x_len, start[1] + y_len, start[2] + z_len], id: 6},
        Node{a: array![start[0], start[1] + y_len, start[2] + z_len], id: 7},
    ];

    let edges = [
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

    let nodes = [
        // Front
        Node{a: start, id: 0},
        Node{array![start[0] + x_len, start[1], start[2]], id: 1},
        Node{array![start[0] + x_len, start[1] + y_len, start[2]], id: 2},
        Node{array![start[0], start[1] + y_len, start[2]], id: 3},

        // Top
        Node{a: array![start[0] + x_len / 2., start[1] + y_len / 2., start[2] + z_len] id: 4},

    ];

    let edges = [
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
    make_box(start, side_len, side_len, side_len)
}