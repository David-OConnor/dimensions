// We use mathematical conventions that may direct upper-case var names.
#![allow(non_snake_case)]

#[macro_use(array)]
extern crate ndarray;
extern crate ggez;

mod types;
mod drawing;
mod transforms;

use std::f64::consts::PI;
use ndarray::prelude::*;

use types::{Node, Edge, Camera};

const TAU: f64 = 2. * PI;

fn run_3d() -> (Vec<Node>, Vec<Edge>) {
    let camera = Camera {
        c: Array::from_vec(vec![-0.5, 0., 0.]),
        theta: array![0., 0., 0.],
        e: arr1(&[0., 0., 5.]),
    };

    let cube_nodes = vec![
        Node {a: array![0., 0., 0.], id: 0},
        Node {a: array![0., 1., 0.], id: 1},
        Node {a: array![1., 1., 0.], id: 2},
        Node {a: array![1., 0., 0.], id: 3},
        
        Node {a: array![0., 0., 1.], id: 4},
        Node {a: array![0., 1., 1.], id: 5},
        Node {a: array![1., 1., 1.], id: 6},
        Node {a: array![1., 0., 1.], id: 7}
    ];

    let cube_edges = vec![
        Edge {node1: 0, node2: 1},
        Edge {node1: 1, node2: 2},
        Edge {node1: 2, node2: 3},
        Edge {node1: 3, node2: 0},
        
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

    (cube_nodes, cube_edges)
}


fn run_4d() -> (Vec<Node>, Vec<Edge>) {
    let camera = Camera {
        c: Array::from_vec(vec![-0.5, 0., 0., 0.]),
        theta: array![0., 0., 0., 0.],
        e: arr1(&[0., 0., 5., 0.]),
    };

    let hypercube_nodes = vec![
        Node {a: array![0., 0., 0., 0.], id: 0},
        Node {a: array![0., 1., 0., 0.], id: 1},
        Node {a: array![1., 1., 0., 0.], id: 2},
        Node {a: array![1., 0., 0., 0.], id: 3},
        
        Node {a: array![0., 0., 1., 0.], id: 4},
        Node {a: array![0., 1., 1., 0.], id: 5},
        Node {a: array![1., 1., 1., 0.], id: 6},
        Node {a: array![1., 0., 1., 0.], id: 7},
        
        Node {a: array![0., 0., 0., 1.], id: 8},
        Node {a: array![0., 1., 0., 2.], id: 9},
        Node {a: array![1., 1., 0., 3.], id: 10},
        Node {a: array![1., 0., 0., 4.], id: 11},
        
        Node {a: array![0., 0., 1., 5.], id: 12},
        Node {a: array![0., 1., 1., 6.], id: 13},
        Node {a: array![1., 1., 1., 7.], id: 14},
        Node {a: array![1., 0., 1., 8.], id: 15}
    ];

    let hypercube_edges = vec![
        // "inner" cube bottom
        Edge {node1: 0, node2: 1},
        Edge {node1: 1, node2: 2},
        Edge {node1: 2, node2: 3},
        Edge {node1: 3, node2: 0},
        
        // "inner" cube top
        Edge {node1: 4, node2: 5},
        Edge {node1: 5, node2: 6},
        Edge {node1: 6, node2: 7},
        Edge {node1: 7, node2: 4},
        
        // Bridge inner cube
        Edge {node1: 0, node2: 4},
        Edge {node1: 1, node2: 5},
        Edge {node1: 2, node2: 6},
        Edge {node1: 3, node2: 7},

        // "outer" cube bottom
        Edge {node1: 8, node2: 9},
        Edge {node1: 9, node2: 10},
        Edge {node1: 10, node2: 11},
        Edge {node1: 11, node2: 8},
        
        // "outer" cube top
        Edge {node1: 12, node2: 13},
        Edge {node1: 13, node2: 14},
        Edge {node1: 14, node2: 15},
        Edge {node1: 15, node2: 12},
        
        // Bridge outer cube
        Edge {node1: 8, node2: 12},
        Edge {node1: 9, node2: 13},
        Edge {node1: 10, node2: 14},
        Edge {node1: 11, node2: 15},

        // Bridge inner to outer bottom
        Edge {node1: 0, node2: 8},
        Edge {node1: 1, node2: 9},
        Edge {node1: 2, node2: 10},
        Edge {node1: 3, node2: 11},
        
        // Bridge inner to outer top
        Edge {node1: 4, node2: 12},
        Edge {node1: 5, node2: 13},
        Edge {node1: 6, node2: 14},
        Edge {node1: 7, node2: 15},
    ];

    // // nodes are projected from 4d space into 2d space, in two steps. 
    // // Node associations with edges are not affected by the transformation.
    // let projected_nodes_3d: Vec<Node> = &hypercube_nodes.into_iter()
    //     .map(|node| transforms::project_4d(&camera, &node)).collect();

    // let projected_nodes_2d: Vec<Node> = projected_nodes_3d.into_iter()
    //     .map(|node| transforms::project_3d(&camera, &node)).collect();

    (hypercube_nodes, hypercube_edges)
}

fn main() {
    const _FOV: f64 = 80.;  // Degrees.
    
    let (nodes, edges) = run_3d(); 

    drawing::run(nodes, edges);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cam_transform() {
        let camera = Camera {
            c: Array::from_vec(vec![0., 0., 0.]),
            theta: array![PI/4., 0., PI/2.],
            e: arr1(&[1., 2., 0.]),
        };
        let node = Node {a: array![2., 3., -4.], id: 0};

        let expected = arr1(&[3., -5., -2.]);
        let calculated = transforms::camera_transform_3d(&camera, &node);

        // Unable to apply floor to array, or use an approximately equal
        // assertion for floating points, so compare each value to a floored one.
        assert_eq!(calculated[0].floor(), expected[0]);
        assert_eq!(calculated[1].floor(), expected[1]);
        assert_eq!(calculated[2].floor(), expected[2]);
    }
}
