use ndarray::prelude::*;

use types::{Node, Edge, Shape};

// We'll define y as vertical, and z as forward/back.  All shapes are given
// four coordinates. Leave 

pub fn make_box(center: &Array1<f64>, x_len: f64, 
                y_len: f64, z_len: f64, id: i32) -> Shape {
    // Make a rectangular prism.  Use negative lengths to draw in the opposite
    // direction.

    let nodes = vec![
        // Front
        Node{a: center.clone(), id: 0},
        Node{a: array![center[0] + x_len, center[1], center[2], center[3]], id: 1},
        Node{a: array![center[0] + x_len, center[1] + y_len, center[2], center[3]], id: 2},
        Node{a: array![center[0], center[1] + y_len, center[2], center[3]], id: 3},

        Node{a: array![center[0], center[1], center[2] + z_len, center[3]], id: 4},
        Node{a: array![center[0] + x_len, center[1], center[2] + z_len, center[3]], id: 5},
        Node{a: array![center[0] + x_len, center[1] + y_len, center[2] + z_len, center[3]], id: 6},
        Node{a: array![center[0], center[1] + y_len, center[2] + z_len, center[3]], id: 7},
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

    Shape {nodes, edges, id}
}

pub fn make_rectangular_pyramid(center: &Array1<f64>, x_len: f64, 
                                z_len: f64, height: f64, id: i32) -> Shape {

    let nodes = vec![
        // Base
        Node{a: center.clone(), id: 0},
        Node{a: array![center[0] + x_len, center[1], center[2], center[3]], id: 1},
        Node{a: array![center[0] + x_len, center[1], center[2] + z_len, center[3]], id: 2},
        Node{a: array![center[0], center[1], center[2] + z_len, center[3]], id: 3},

        // Top
        Node{a: array![center[0] + x_len / 2., center[1] + height, center[2] + z_len / 2., center[3]], id: 4},

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

    Shape {nodes, edges, id}
}

pub fn make_house(center: &Array1<f64>, x_len: f64, 
                  y_len: f64, z_len: f64, id: i32) -> Shape {
    let mut base = make_box(
        &array![center[0], center[1], center[2], center[3]], x_len, y_len, z_len, id
    );
    let mut roof = make_rectangular_pyramid(
        // Let the roof overhang the base by a little.
        &array![center[0] - x_len * 0.1, center[1] + y_len, center[2] - x_len * 0.1, center[3]],
        x_len * 1.2, 
        z_len * 1.2, 
        y_len / 3.,  // Make the roof height a portion of the base height. 
        id
    );

    // Now that we've made the shapes, recompose them to be one shape.
    // todo make this a separate, (reusable) func?
    let id_addition = base.nodes.len() as i32;
    for node in &mut roof.nodes {
        node.id += id_addition;  // There are 8 nodes in the base.
    }
    for edge in &mut roof.edges {
        edge.node1 += id_addition;
        edge.node2 += id_addition;
    }
    base.nodes.append(&mut roof.nodes);
    base.edges.append(&mut roof.edges);

    base
}

pub fn make_cube(center: &Array1<f64>, side_len: f64, id: i32) -> Shape {
    // Convenience function.
    make_box(center, side_len, side_len, side_len, id)
}

pub fn make_origin(center: &Array1<f64>, len: f64, id: i32) -> Shape {
    // A 3-dimensional cross, for marking the origin.
    let nodes = vec![
        Node {a: array![center[0] - len / 2., center[1], center[2], center[3]], id: 0},
        Node {a: array![center[0] + len / 2., center[1], center[2], center[3]], id: 1},
        Node {a: array![center[0], center[1] - len / 2., center[2], center[3]], id: 2},
        Node {a: array![center[0], center[1] + len / 2., center[2], center[3]], id: 3},
        Node {a: array![center[0], center[1], center[2] - len / 2., center[3]], id: 4},
        Node {a: array![center[0], center[1], center[2] + len / 2., center[3]], id: 5},
    ];

    let edges = vec![
        Edge {node1: 0, node2: 1},
        Edge {node1: 2, node2: 3},
        Edge {node1: 4, node2: 5},
    ];

    Shape {nodes, edges, id}
}

pub fn make_street(center: &Array1<f64>, _direction: &Array1<f64>, 
                   width: f64, id: i32) -> Shape {
    // Make a street extending very far into the distance in both directions.
    // Direction is the vector the street points.

    // todo implement direction.
    let nodes = vec![
        // Left
        Node {a: array![center[0] - width / 2., center[1], -99.], id: 0},
        Node {a: array![center[0] - width / 2., center[1], 99.], id: 1},

        // Right
        Node {a: array![center[0] + width / 2., center[1], -99.], id: 2},
        Node {a: array![center[0] + width / 2., center[1], 99.], id: 3},
    ];

    let edges = vec![
        Edge {node1: 0, node2: 1},
        Edge {node1: 2, node2: 3},
    ];

    Shape {nodes, edges, id}
}

pub fn make_hyperrect(center: Array1<f64>, x_len: f64, y_len: f64, z_len: f64,
                      u_len: f64, id: i32) -> Shape {
    // Make a 4d hypercube.

     let nodes = vec![
        // Front inner
        Node {a: array![center[0], center[1], center[2], center[3]], id: 0},
        Node {a: array![center[0], center[1] + y_len, center[2], center[3]], id: 1},
        Node {a: array![center[0] + x_len, center[1] + y_len, center[2], center[3]], id: 2},
        Node {a: array![center[0] + x_len, center[1], center[2], center[3]], id: 3},
        
        // Back inner
        Node {a: array![center[0], center[1], center[2] + z_len, center[3]], id: 4},
        Node {a: array![center[0], center[1] + y_len, center[2] + z_len, center[3]], id: 5},
        Node {a: array![center[0] + x_len, center[1] + y_len, center[2] + z_len, 0.], id: 6},
        Node {a: array![center[0] + x_len, center[1], center[2] + z_len, center[3]], id: 7},
        
        // Front outer
        Node {a: array![center[0], center[1], center[2], center[3] + u_len], id: 8},
        Node {a: array![center[0], center[1] + y_len, center[2], center[3] + u_len], id: 9},
        Node {a: array![center[0] + x_len, center[1] + y_len, center[2], center[3] + u_len], id: 10},
        Node {a: array![center[0] + x_len, center[1], center[2], center[3] + u_len], id: 11},
        
        // Back outer
        Node {a: array![center[0], center[1], center[2] + z_len, center[3] + u_len], id: 12},
        Node {a: array![center[0], center[1] + y_len, center[2] + z_len, 6.], id: 13},
        Node {a: array![center[0] + x_len, center[1] + y_len, center[2] + z_len, center[3] + u_len], id: 14},
        Node {a: array![center[0] + x_len, 0., center[2] + z_len, center[3] + u_len], id: 15}
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

    Shape {nodes, edges, id}
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
            id: 0,
        };

        assert_eq!(make_cube(array![1., 2., -1.], 2.0, 0), exptected);
    }
}