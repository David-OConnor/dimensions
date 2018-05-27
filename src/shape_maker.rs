use std::collections::HashMap;

use ndarray::prelude::*;

use types::{Node, Edge, Face, Shape};

// We'll define y as vertical, and z as forward/back.  All shapes are given
// four coordinates. Leave

// Nodes are set up here so that 0 is at their center; this is used for scaling,
// rotation, and positioning in the world.

pub fn make_box(x_len: f64, y_len: f64, z_len: f64,
                position: Array1<f64>, scale: f64, orientation: Array1<f64>,
                rotation_speed: Array1<f64>) -> Shape {
    // Make a rectangular prism.  Use negative lengths to draw in the opposite
    // direction.

    let coords = [
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

    let mut nodes = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        nodes.insert(id as i32, Node {
            a: array![coord[0] * x_len, coord[1] * y_len, coord[2] * z_len, coord[3]]
        });
    }

    let edges = vec![
        // Front
        Edge {node0: 0, node1: 1},
        Edge {node0: 1, node1: 2},
        Edge {node0: 2, node1: 3},
        Edge {node0: 3, node1: 0},

        // Back
        Edge {node0: 4, node1: 5},
        Edge {node0: 5, node1: 6},
        Edge {node0: 6, node1: 7},
        Edge {node0: 7, node1: 4},

        // Bridger
        Edge {node0: 0, node1: 4},
        Edge {node0: 1, node1: 5},
        Edge {node0: 2, node1: 6},
        Edge {node0: 3, node1: 7},
    ];

    let faces = vec![
        // Front
        Face {edges: vec![edges[0].clone(), edges[1].clone(), edges[2].clone(), edges[3].clone()]},
        // Back
        Face {edges: vec![edges[4].clone(), edges[5].clone(), edges[6].clone(), edges[7].clone()]},
        // Top
        Face {edges: vec![edges[2].clone(), edges[10].clone(), edges[6].clone(), edges[11].clone()]},
        // Bottom
        Face {edges: vec![edges[0].clone(), edges[9].clone(), edges[4].clone(), edges[8].clone()]},
        // Left
        Face {edges: vec![edges[3].clone(), edges[8].clone(), edges[7].clone(), edges[11].clone()]},
        // Right
        Face {edges: vec![edges[1].clone(), edges[9].clone(), edges[5].clone(), edges[10].clone()]},
    ];

    Shape {nodes, edges, faces, position, scale, orientation, rotation_speed}
}

pub fn make_rectangular_pyramid(x_len: f64,
                                z_len: f64, height: f64,
                                position: Array1<f64>, scale: f64, orientation: Array1<f64>,
                                rotation_speed: Array1<f64>) -> Shape {
    let coords = [
        // Base
        [-1., 0., -1., 0.],
        [1., 0., -1., 0.],
        [1., 0., 1., 0.],
        [-1., 0., 1., 0.],

        // Top
        [0., 1., 0., 0.],
    ];

    let mut nodes = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        nodes.insert(id as i32, Node {
            a: array![coord[0] * x_len, coord[1] * height, coord[2] * z_len, coord[3]]
        });
    }

    let edges = vec![
        // Front
        Edge {node0: 0, node1: 1},
        Edge {node0: 1, node1: 2},
        Edge {node0: 2, node1: 3},
        Edge {node0: 3, node1: 0},

        // Bridger
        Edge {node0: 0, node1: 4},
        Edge {node0: 1, node1: 4},
        Edge {node0: 2, node1: 4},
        Edge {node0: 3, node1: 4},
    ];

    let faces = vec![
        // Base
        Face {edges: vec![edges[0].clone(), edges[1].clone(), edges[2].clone(), edges[3].clone()]},
        // Front
        Face {edges: vec![edges[0].clone(), edges[4].clone(), edges[5].clone()]},
        // Right
        Face {edges: vec![edges[1].clone(), edges[5].clone(), edges[6].clone()]},
        // Back
        Face {edges: vec![edges[2].clone(), edges[6].clone(), edges[7].clone()]},
        // Left
        Face {edges: vec![edges[3].clone(), edges[7].clone(), edges[4].clone()]},
    ];

    Shape {nodes, edges, faces, position, scale, orientation, rotation_speed}
}

// pub fn make_house(x_len: f64,
//                   y_len: f64, z_len: f64, position: Array1<f64>, orientation: Array1<f64>,
//                                rotation_speed: Array1<f64>) -> Shape {
//     let mut base = make_box(&(center.clone()), x_len, y_len, z_len);

//     let mut roof = make_rectangular_pyramid(
//         // Let the roof overhang the base by a little.
//         &array![center[0] - x_len * 0.1, center[1] + y_len, center[2] - x_len * 0.1, center[3]],
//         x_len * 1.2, 
//         z_len * 1.2, 
//         y_len / 3.,  // Make the roof height a portion of the base height. 
//     );

//     // Now that we've made the shapes, recompose them to be one shape.
//     // todo make this a separate, (reusable) func?
//     let id_addition = base.nodes.len() as i32;

//     let mut combined_nodes = HashMap::new();
//     let mut combined_edges: Vec<Edge> = Vec::new();

//     for (id, node) in &base.nodes {
//         combined_nodes.insert(id as i32, node);
//     }
//     for (id, node) in &roof.nodes {
//         // For the roof, modify the ids to be unique.
//         combined_nodes.insert(&((id + id_addition) as i32), node);
//     }
//     for edge in &mut base.edges {
//         combined_edges.append(edge);
//     }
//     for edge in &roof.edges {
//         combined_edges.append(Edge {
//             node0: edge.node0 + id_addition,
//             node1: edge.node1 + id_addition
//         });
//     }

//     Shape {nodes, edges, position, scale, orientation, rotation_speed}
// }

pub fn make_cube(side_len: f64,
                 position: Array1<f64>, scale: f64, orientation: Array1<f64>,
                 rotation_speed: Array1<f64>) -> Shape {
    // Convenience function.
    make_box(side_len, side_len, side_len, position, scale, orientation, rotation_speed)
}

pub fn make_origin(len: f64, position: Array1<f64>, scale: f64, orientation: Array1<f64>,
                   rotation_speed: Array1<f64>) -> Shape {
    // A 4-dimensional cross, for marking the origin.
    let coords = [
        [-1., 0., 0., 0.],
        [1., 0., 0., 0.],
        [0., -1., 0., 0.],
        [0., 1., 0., 0.],

        [0., 0., -1., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., -1.],
        [0., 0., 0., 1.],
    ];

    let mut nodes = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        nodes.insert(id as i32, Node {
            a: Array::from_vec(coord.to_vec()) * len
        });
    }

    let edges = vec![
        Edge {node0: 0, node1: 1},
        Edge {node0: 2, node1: 3},
        Edge {node0: 4, node1: 5},
        Edge {node0: 6, node1: 7},
    ];

    Shape {nodes, edges, faces: vec![], position, scale, orientation, rotation_speed}
}

//pub fn make_street(width: f64, position: Array1<f64>, scale: f64, orientation: Array1<f64>,
//                   rotation_speed: Array1<f64>) -> Shape {
//    // Make a street extending very far into the distance in both directions.
//    // Direction is the vector the street points.
//
//    let mut nodes = HashMap::new();
//
//    // Left
//    nodes.insert(0, Node {a: array![center[0] - width / 2., center[1], -99.]});
//    nodes.insert(1, Node {a: array![center[0] - width / 2., center[1], 99.]});
//
//    // Right
//    nodes.insert(2, Node {a: array![center[0] + width / 2., center[1], -99.]});
//    nodes.insert(3, Node {a: array![center[0] + width / 2., center[1], 99.]});
//
//    let edges = vec![
//        Edge {node0: 0, node1: 1},
//        Edge {node0: 2, node1: 3},
//    ];
//
//    Shape {nodes, edges, faces, position, scale, orientation, rotation_speed}
//}

pub fn make_hyperrect(x_len: f64, y_len: f64, z_len: f64, u_len: f64,
                      position: Array1<f64>, scale: f64, orientation: Array1<f64>,
                      rotation_speed: Array1<f64>) -> Shape {
    // Make a 4d hypercube.

    let coords = [
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

    let mut nodes = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        nodes.insert(id as i32, Node {
            a: array![coord[0] * x_len, coord[1] * y_len, coord[2] * z_len, coord[3] * u_len]
        });
    }

    let edges = vec![
        // Front inner
        Edge {node0: 0, node1: 1},
        Edge {node0: 1, node1: 2},
        Edge {node0: 2, node1: 3},
        Edge {node0: 3, node1: 0},

        // Back inner
        Edge {node0: 4, node1: 5},
        Edge {node0: 5, node1: 6},
        Edge {node0: 6, node1: 7},
        Edge {node0: 7, node1: 4},

        // Connect front to back inner
        Edge {node0: 0, node1: 4},
        Edge {node0: 1, node1: 5},
        Edge {node0: 2, node1: 6},
        Edge {node0: 3, node1: 7},

        // Front outer
        Edge {node0: 8, node1: 9},
        Edge {node0: 9, node1: 10},
        Edge {node0: 10, node1: 11},
        Edge {node0: 11, node1: 8},

        // Back outer
        Edge {node0: 12, node1: 13},
        Edge {node0: 13, node1: 14},
        Edge {node0: 14, node1: 15},
        Edge {node0: 15, node1: 12},

        // Connect front to back outer
        Edge {node0: 8, node1: 12},
        Edge {node0: 9, node1: 13},
        Edge {node0: 10, node1: 14},
        Edge {node0: 11, node1: 15},

        // Connect front inner to front outer
        Edge {node0: 0, node1: 8},
        Edge {node0: 1, node1: 9},
        Edge {node0: 2, node1: 10},
        Edge {node0: 3, node1: 11},

        // Connect back inner to back outer
        Edge {node0: 4, node1: 12},
        Edge {node0: 5, node1: 13},
        Edge {node0: 6, node1: 14},
        Edge {node0: 7, node1: 15},
    ];

    let faces = vec![
    ];

    Shape {nodes, edges, faces, position, scale, orientation, rotation_speed}
}

pub fn make_hypercube(side_len: f64,
                      position: Array1<f64>, scale: f64, orientation: Array1<f64>,
                      rotation_speed: Array1<f64>) -> Shape {
    // Convenience function.
    make_hyperrect(side_len, side_len, side_len, side_len,
                   position, scale, orientation, rotation_speed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube() {

    }
}