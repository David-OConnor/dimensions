use std::collections::HashMap;

use ndarray::prelude::*;

use types::{Vertex, Edge, Face, Shape};

// We'll define y as vertical, and z as forward/back.  All shapes are given
// four coordinates. Leave

// Nodes are set up here so that 0 is at their center; this is used for scaling,
// rotation, and positioning in the world.

pub fn make_box(lens: (f32, f32, f32),
                position: Array1<f32>, orientation: Array1<f32>,
                rotation_speed: Array1<f32>) -> Shape {
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

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] * lens.0 / 2., coord[1] * lens.1 / 2.,
            coord[2] * lens.2 / 2., coord[3] / 2.
        ));
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

    let faces_vert = vec![  // Vertex indices for each face.
                            array![0, 1, 2, 3],  // Front
                            array![4, 5, 6, 7],  // Back
                            array![3, 2, 6, 7],  // Top
                            array![0, 1, 5, 4],  // Bottom
                            array![0, 4, 7, 3],  // Left
                            array![1, 5, 6, 2],  // Right
    ];

    Shape::new(vertices, edges, faces, faces_vert, position, orientation, rotation_speed)
}

pub fn make_rectangular_pyramid(lens: (f32, f32, f32),
                                position: Array1<f32>, orientation: Array1<f32>,
                                rotation_speed: Array1<f32>) -> Shape {
    let coords = [
        // Base
        [-1., 0., -1., 0.],
        [1., 0., -1., 0.],
        [1., 0., 1., 0.],
        [-1., 0., 1., 0.],

        // Top
        [0., 1., 0., 0.],
    ];

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] / 2. * lens.0, coord[1] / 2. * lens.1,
            coord[2] / 2. * lens.2, coord[3] / 2.
        ));
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

    let faces_vert = vec![  // Vertex indices for each face.
                            array![0, 1, 2, 3],  // Base
                            array![0, 1, 4],  // Front
                            array![1, 2, 4],  // Right
                            array![2, 3, 4],  // Back
                            array![3, 0, 4],  // Left
    ];

    Shape::new(vertices, edges, faces, faces_vert, position, orientation, rotation_speed)
}

 pub fn make_house(lens: (f32, f32, f32),
                   position: Array1<f32>,
                   orientation: Array1<f32>,
                   rotation_speed: Array1<f32>) -> Shape {
     let empty_array = array![0., 0., 0., 0., 0., 0.];

     // We'll modify base in-place, then return it.
     let mut base = make_box(lens, position, orientation, rotation_speed);

     let roof = make_rectangular_pyramid(
         // Let the roof overhang the base by a little.
         // Make the roof height a portion of the base height.
         (lens.0 * 1.2, lens.1 / 3., lens.2 * 1.2),
         empty_array.clone(), empty_array.clone(), empty_array.clone()
     );

     // Now that we've made the shapes, recompose them to be one shape.
     // todo make this a separate, (reusable) func?1
     let id_addition = base.vertices.len() as u32;

     for (id, vertex) in &roof.vertices {
         // For the roof, modify the ids to be unique.
         base.vertices.insert(
             id + id_addition,
             // Raise the roof.
             Vertex::new(vertex.position.0, vertex.position.1 + lens.1 / 2.,
                         vertex.position.2, vertex.position.3
             ));
     }
     for edge in &roof.edges {
         base.edges.push(Edge {
             node0: edge.node0 + id_addition,
             node1: edge.node1 + id_addition
         });
     }
     for face in &roof.faces {
         base.faces.push(face.clone());
     }
     base
 }

pub fn make_cube(side_len: f32,
                 position: Array1<f32>, orientation: Array1<f32>,
                 rotation_speed: Array1<f32>) -> Shape {
    // Convenience function.
    // We'll still treat the center as the center of the base portion.
    make_box((side_len, side_len, side_len), position, orientation, rotation_speed)
}

pub fn make_origin(len: f32, position: Array1<f32>, orientation: Array1<f32>,
                   rotation_speed: Array1<f32>) -> Shape {
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

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] * len / 2., coord[1] * len / 2.,
            coord[2] * len / 2.,coord[3] * len / 2.
        ));
    }

    let edges = vec![
        Edge {node0: 0, node1: 1},
        Edge {node0: 2, node1: 3},
        Edge {node0: 4, node1: 5},
        Edge {node0: 6, node1: 7},
    ];

    let faces = vec![];
    let faces_vert = vec![array![]]; // todo temp

    Shape::new(vertices, edges, faces, faces_vert, position, orientation, rotation_speed)
}

//pub fn make_street(width: f32, position: Array1<f32>, scale: f32, orientation: Array1<f32>,
//                   rotation_speed: Array1<f32>) -> Shape {
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
//    Shape {nodes, edges, faces, position, 1., orientation, rotation_speed}
//}

pub fn make_hyperrect(lens: (f32, f32, f32, f32),
                      position: Array1<f32>, orientation: Array1<f32>,
                      rotation_speed: Array1<f32>) -> Shape {
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

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] * lens.0 / 2., coord[1] * lens.1 / 2.,
            coord[2] * lens.2 / 2., coord[3] * lens.3 / 2.
        ));
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

    let faces = vec![];
    let faces_vert = vec![  // Vertex indices for each face.
        array![0, 1, 2, 3],  // Front inner
        array![4, 5, 6, 7],  // Back inner
        array![3, 2, 6, 7],  // Top inner
        array![0, 1, 5, 4],  // Bottom inner
        array![0, 4, 7, 3],  // Left inner
        array![1, 5, 6, 2],  // Right inner

        array![8, 9, 10, 11],  // Front outer
        array![12, 13, 14, 15],  // Back outer
        array![11, 10, 14, 15],  // Top outer
        array![8, 9, 13, 12],  // Bottom outer
        array![8, 12, 15, 11],  // Left outer
        array![9, 13, 14, 10],  // Right outer

        array![8, 9, 1, 0],  // Front bottom
        array![12, 13, 5, 4],  // Back bottom
        array![12, 8, 0, 4],  // Left bottom
        array![9, 13, 5, 1],  // Right bottom

        array![11, 10, 2, 3],  // Front top
        array![15, 14, 6, 7],  // Back top
        array![15, 11, 3, 7],  // Left top
        array![14, 10, 2, 6],  // Right top

        array![11, 8, 0, 3],  // Left forward
        array![15, 12, 4, 7],  // Left back
        array![10, 9, 1, 2],  // Right forward
        array![14, 13, 5, 6],  // Right back

    ];

    Shape::new(vertices, edges, faces, faces_vert, position, orientation, rotation_speed)
}

pub fn make_hypercube(side_len: f32,
                      position: Array1<f32>, orientation: Array1<f32>,
                      rotation_speed: Array1<f32>) -> Shape {
    // Convenience function.
    make_hyperrect((side_len, side_len, side_len, side_len),
                   position, orientation, rotation_speed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube() {

    }
}