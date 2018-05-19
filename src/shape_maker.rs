use std::collections::HashMap;

use ndarray::prelude::*;

use types::{Node, Edge, Shape};

// We'll define y as vertical, and z as forward/back.  All shapes are given
// four coordinates. Leave

// Nodes are set up here so that 0 is at their center; this is used for scaling,
// rotation, and positioning in the world.

pub fn make_box(x_len: f64, y_len: f64, z_len: f64,
                position: Array1<f64>, scale: f64, rotation: Array1<f64>,
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
            a: array![coord[0] * x_len, coord[1] * y_len, coord[2].z_len, coord[3]]
        });
    }

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

    Shape {nodes, edges, position, scale, rotation, rotation_speed}
}

pub fn make_rectangular_pyramid(x_len: f64,
                                z_len: f64, height: f64,
                                position: Array1<f64>, scale: f64, rotation: Array1<f64>,
                                rotation_speed: Array1<f64>) -> Shape {
    let coords = [
        // Base
        [-1., -1., 0., 0.],
        [1., -1., 0., 0.],
        [1., 1., 0., 0.],
        [-1., 1., 0., 0.],

        // Top
        [0., 0., 1., 0.],
    ];

    let mut nodes = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        nodes.insert(id as i32, Node {
            a: array![coord[0] * x_len, coord[1] * y_len, coord[2].height, coord[4]]
        });
    }

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

    Shape {nodes, edges, position, scale, rotation, rotation_speed}
}

// pub fn make_house(x_len: f64,
//                   y_len: f64, z_len: f64, position: Array1<f64>, rotation: Array1<f64>,
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
//             node1: edge.node1 + id_addition,
//             node2: edge.node2 + id_addition
//         });
//     }

//     Shape {nodes, edges, position, scale, rotation, rotation_speed}
// }

pub fn make_cube(side_len: f64,
                 position: Array1<f64>, rotation: Array1<f64>,
                 rotation_speed: Array1<f64>) -> Shape {
    // Convenience function.
    make_box(side_len, side_len, side_len, position, scale: f64, rotation, rotation_speed)
}

pub fn make_origin(len: f64, position: Array1<f64>, rotation: Array1<f64>,
                 rotation_speed: Array1<f64>) -> Shape {
    // A 4-dimensional cross, for marking the origin.
    assert_eq![center.len(), 4];

    let coords = [
        [-1., 0., 0., 0.],
        [1., 0., 0., 0.],
        [0., -1., 0., 0.],
        [0., 1., 0., 0.],
        
        [0., 0., 1., 0.],
        [0., 0., -1., 0.],
        [0., 0., 0., -1.],
        [0., 0., 0., 1.],
    ];

    let mut nodes = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        nodes.insert(id as i32, Node {
            a: Array::from_vec(coord) * len
        });
    }

    let edges = vec![
        Edge {node1: 0, node2: 1},
        Edge {node1: 2, node2: 3},
        Edge {node1: 4, node2: 5},
        Edge {node1: 6, node2: 7},
    ];

    Shape {nodes, edges, position, scale, rotation, rotation_speed}
}

pub fn make_street(width: f64, position: Array1<f64>, scale: f64, rotation: Array1<f64>,
                   rotation_speed: Array1<f64>) -> Shape {
    // Make a street extending very far into the distance in both directions.
    // Direction is the vector the street points.

    let mut nodes = HashMap::new();

    // Left
    nodes.insert(0, Node {a: array![center[0] - width / 2., center[1], -99.]});
    nodes.insert(1, Node {a: array![center[0] - width / 2., center[1], 99.]});

    // Right
    nodes.insert(2, Node {a: array![center[0] + width / 2., center[1], -99.]});
    nodes.insert(3, Node {a: array![center[0] + width / 2., center[1], 99.]});

    let edges = vec![
        Edge {node1: 0, node2: 1},
        Edge {node1: 2, node2: 3},
    ];

    Shape {nodes, edges, position, scale, rotation, rotation_speed}
}

pub fn make_hyperrect(x_len: f64, y_len: f64, z_len: f64,
                      u_len: f64, position: Array1<f64>, scale: f64, rotation: Array1<f64>,
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
            a: array![coord[0] * x_len, coord[1] * y_len, coord[2].z_len, coord[3].u_len]
        });
    }

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

    Shape {nodes, edges, position, scale, rotation, rotation_speed}
}

pub fn make_hypercube(side_len: f64,
                      position: Array1<f64>, rotation: Array1<f64>,
                      rotation_speed: Array1<f64>) -> Shape {
    // Convenience function.
    make_hyperrect(center, side_len, side_len, side_len, side_len,
                   position, rotation, rotation_speed)
}

#[cfg(test)]
mod tests {
    use::super::*;

    #[test]
    fn test_cube() {

        let expected = Shape{
            nodes: {vec![
                Node {a: array![1., 2., -1.], id: 0},
                Node {a: array![1., 2., -1.] , id: 1},
                Node {a: , id: 2},
                Node {a: , id: 3},
                
                Node {a: , id: 4},
                Node {a: , id: 5},
                Node {a: , id: 6},
                Node {a: , id: 7},
            ]},
            edges: {vec![
                Edge {node1: 0, node2: 1},
                Edge {node1: 1, node2: 2},
                Edge {node1: 2, node2: 3},
                Edge {node1: 3, node2: 0},
                
                Edge {node1: 4, node2: 5},
                Edge {node1: 5, node2: 6},
                Edge {node1: 6, node2: 7},
                Edge {node1: 7, node2: 4},
                
                Edge {node1: 0, node2: 4},
                Edge {node1: 1, node2: 5},
                Edge {node1: 2, node2: 6},
                Edge {node1: 3, node2: 7},
            ]},
        };

        assert_eq!(make_cube(array![1., 2., -1.], 2.0, 0), exptected);
    }
}