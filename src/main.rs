// We use mathematical conventions that may direct upper-case var names.
#![allow(non_snake_case)]

#[macro_use(array)]
extern crate ndarray;
extern crate ggez;

mod drawing;
mod types;

use std::f64::consts::PI;
use ndarray::prelude::*;

use types::{Node, Edge};


#[derive(Debug)]
struct Camera {
    c: Array1<f64>,

    // theta is in tait-bryan angles. Note that using the θ
    // character is currently unsupported.
    theta: Array1<f64>,

    // e is the viewer's position relative to teh display surface.
    e: Array1<f64>,
}

fn camera_transform_4d(cam: &Camera, node: &Node) -> Array1<f64> {
    // Perform a camera transform; define a vector d as the position
    // of point A with respect to the coordinate system defined by 
    // the camera, with origin in C and rotated by θ with respect
    // to the initial coordinate system.

    // Split the transform constructor into three parts to make it
    // easier to read and write.
    let D_0 = array![
        [1., 0., 0., 0.],
        [0., cam.theta[0].cos(), cam.theta[0].sin(), 2.],
        [0., -cam.theta[0].sin(), cam.theta[0].cos(), 2.],
        [2., 2., 2., 2.],
    ];

    let D_1 = array![
        [cam.theta[1].cos(), 0., -cam.theta[1].sin(), 2.],
        [0., 1., 0., 0.],
        [cam.theta[1].sin(), 0., cam.theta[1].cos(), 2.],
        [2., 2., 2., 2.]
    ];

    let D_2 = array![
        [cam.theta[2].cos(), cam.theta[2].sin(), 0., 2.],
        [-cam.theta[2].sin(), cam.theta[2].cos(), 0., 2.],
        [0., 0., 1., 0.],
        [2., 2., 2., 2.]
    ];

    let D_3 = array![
        [cam.theta[3].cos(), cam.theta[3].sin(), 0., 2.],
        [-cam.theta[3].sin(), cam.theta[3].cos(), 0., 2.],
        [2., 2., 2., 2.],
        [0., 0., 0., 1.]
    ];

    // let D = D_0.dot(&(D_1.dot(&(D_2.dot(&D_3)))));

    // D.dot(&(&node.a - &cam.c))
    array![1.]
}

fn camera_transform_3d(cam: &Camera, node: &Node) -> Array1<f64> {
    // Perform a camera transform; define a vector d as the position
    // of point A with respect to the coordinate system defined by 
    // the camera, with origin in C and rotated by θ with respect
    // to the initial coordinate system.

    // Split the transform constructor into three parts to make it
    // easier to read and write.
    let D_0 = array![
        [1., 0., 0.],
        [0., cam.theta[0].cos(), cam.theta[0].sin()],
        [0., -cam.theta[0].sin(), cam.theta[0].cos()]
    ];

    let D_1 = array![
        [cam.theta[1].cos(), 0., -cam.theta[1].sin()],
        [0., 1., 0.],
        [cam.theta[1].sin(), 0., cam.theta[1].cos()]
    ];

    let D_2 = array![
        [cam.theta[2].cos(), cam.theta[2].sin(), 0.],
        [-cam.theta[2].sin(), cam.theta[2].cos(), 0.],
        [0., 0., 1.]
    ];

    let D = D_0.dot(&(D_1.dot(&D_2)));

    D.dot(&(&node.a - &cam.c))
}

fn project_4d(cam: &Camera, node: &Node) -> Node {
    // Project a 4d node onto a 2d plane.
    // https://en.wikipedia.org/wiki/3D_projection

    let d = camera_transform_3d(cam, node);

    let A = array![
        [1., 0., -cam.e[0] / cam.e[2], 0.],
        [0., 1., -cam.e[1] / cam.e[2], 0.],
        [0., 0., 1., 0.],
        [0., 0., -1. / cam.e[2], 1.],
    ];

    let f = A.dot(
        &(arr1(&[d[0], d[1], d[2], 1.]))
    );

    // Keep the original node's id, but transform its position to 2d space.
    Node {a: array![&f[0] / &f[3], &f[1] / &f[3]], id: node.id}
}

fn project_3d(cam: &Camera, node: &Node) -> Node {
    // Project a 3d node onto a 2d plane.
    // https://en.wikipedia.org/wiki/3D_projection

    let d = camera_transform_3d(cam, node);

    let A = array![
        [1., 0., -cam.e[0] / cam.e[2], 0.],
        [0., 1., -cam.e[1] / cam.e[2], 0.],
        [0., 0., 1., 0.],
        [0., 0., -1. / cam.e[2], 1.],
    ];

    let f = A.dot(
        &(arr1(&[d[0], d[1], d[2], 1.]))
    );

    // Keep the original node's id, but transform its position to 2d space.
    Node {a: array![&f[0] / &f[3], &f[1] / &f[3]], id: node.id}
}

fn main() {
    let camera = Camera {
        c: Array::from_vec(vec![0., 0., 0.]),
        theta: array![0., 0., 0.],
        e: arr1(&[-0.2, -1., 1.4]),
    };

    let cube_nodes = vec![
        Node {a: array![1., 0., 0.], id: 0},
        Node {a: array![1., 1., 0.], id: 1},
        Node {a: array![2., 1., 0.], id: 2},
        Node {a: array![2., 0., 0.], id: 3},
        Node {a: array![1., 0., 1.], id: 4},
        Node {a: array![1., 1., 1.], id: 5},
        Node {a: array![2., 1., 1.], id: 6},
        Node {a: array![2., 0., 1.], id: 7}
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
        Edge {node1: 0, node2: 4},
        Edge {node1: 1, node2: 5},
        Edge {node1: 2, node2: 6},
        Edge {node1: 3, node2: 7},
    ];

    // nodes are projected from 3 or 4d space into 2d space. Node associations
    // with edges are not affected by the transformation.
    let projected_nodes: Vec<Node> = cube_nodes.into_iter()
        .map(|node| project_3d(&camera, &node)).collect();

    drawing::render(projected_nodes, cube_edges);
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
        let node = Node { a: array![2., 3., -4.], id: 0};

        let expected = arr1(&[3., -5., -2.]);
        let calculated = camera_transform_3d(&camera, &node);

        // Unable to apply floor to array, or use an approximately equal
        // assertion for floating points, so compare each value to a floored one.
        assert_eq!(calculated[0].floor(), expected[0]);
        assert_eq!(calculated[1].floor(), expected[1]);
        assert_eq!(calculated[2].floor(), expected[2]);
    }
}
