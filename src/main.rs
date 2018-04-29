// We use mathematical conventions that may direct upper-case var names.
#![allow(non_snake_case)]

#[macro_use(array)]
extern crate ndarray;
// extern crate graphics;
extern crate ggez;

use std::f64::consts::PI;

use ndarray::prelude::*;

mod drawing;


#[derive(Debug)]
struct Node {
    a: Array1<f64>,
    id: i32,
}

#[derive(Debug)]
struct Edge {
    node1: i32,  // The node's id
    node2: i32,
}

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

fn project(cam: &Camera, node: &Node) -> Array1<f64> {
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

    array![&f[0] / &f[3], &f[1] / &f[3]]
}

fn main() {
    let camera = Camera {
        c: Array::from_vec(vec![0., 0., 0.]),
        theta: array![0., 0., 0.],
        e: arr1(&[-0.2, -1., 1.4]),
    };

    let cube = vec![
        Node {a: array![1., 0., 0.], id: 0},
        Node {a: array![1., 1., 0.], id: 1},
        Node {a: array![2., 1., 0.], id: 2},
        Node {a: array![2., 0., 0.], id: 3},
        Node {a: array![1., 0., 1.], id: 4},
        Node {a: array![1., 1., 1.], id: 5},
        Node {a: array![2., 1., 1.], id: 6},
        Node {a: array![2., 0., 1.], id: 7}
    ];

    println!("Cube: {:?}", &cube);

    let projection: Vec<Array1<f64>> = cube.into_iter()
        .map(|node| project(&camera, &node)).collect();

    drawing::render(&projection);
   
    println!("Projection: {:?}", projection);
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
