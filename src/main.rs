// We use mathematical conventions that may direct upper-case var names.
#![allow(non_snake_case)]

#[macro_use(array)]
extern crate ndarray;

use ndarray::prelude::*;


struct Node {
    a: Array1<f64>,
    id: i32,
}


struct Edge {
    node1: i32,  // The node's id
    node2: i32,
}

struct Camera {
    c: Array1<f64>,

    // theta is in tait-bryan angles. Note that using the θ
    // character is currently unsupported.
    theta: Array1<f64>,

    // e is the viewer's position relative to teh display surface.
    e: Array1<f64>,
}

fn project(cam: Camera, node: Node) -> Array1<f64> {
    // Project a 4d node onto a 2d plane.
    // https://en.wikipedia.org/wiki/3D_projection

    let D_1 = array![
        [1., 0., 0.],
        [0., cam.theta[0].cos(), cam.theta[1].sin()],
        [0., -cam.theta[0].sin(), cam.theta[0].cos()]
    ];

    let D_2 = array![
        [cam.theta[1].cos(), 0., -cam.theta[1].sin()],
        [0., 1., 0.],
        [cam.theta[1].sin(), 0., cam.theta[1].cos()]
    ];

    let D_3 = array![
        [cam.theta[2].cos(), cam.theta[2].sin(), 0.],
        [-cam.theta[2].sin(), cam.theta[2].cos(), 0.],
        [0., 0., 1.]
    ];

    let D = D_1.dot(&(D_2.dot(&D_3)));

    //  d is the position of the node a in the coordinate system
    // defined by the camera, wiht origin in C and rotated by theta.
    let d = D.dot(&(node.a - cam.c));

    let A = array![
        [1., 0., -cam.e[0] / cam.e[2], 0.],
        [0., 1., -cam.e[1] / cam.e[2], 0.],
        [0., 0., 1., 0.],
        [0., 0., -1. / cam.e[2], 1.],
    ];
    let f = A.dot(&d);

    array![&f[0] / &f[3], &f[1] / &f[3]]
}

fn main() {
    let camera = Camera {
        c: Array::from_vec(vec![0., 0., 0.]),
        theta: array![0., 0., 0.],
        e: arr1(&[0., 0., 0.]),
    };

    let cube = vec![
        Node {a: array![1., 0., 0.], id: 0}
        Node {a: array![1., 1., 0.], id: 1}
        Node {a: array![2., 1., 0.], id: 2}
        Node {a: array![2., 0., 0.], id: 3}
        Node {a: array![1., 0., 1.], id: 4}
        Node {a: array![1., 1., 1.], id: 5}
        Node {a: array![2., 1., 1.], id: 6}
        Node {a: array![2., 0., 1.], id: 7}
    ];

    let projection: Vec<Array1<f64>> = cube.map(|node| project(camera, node)).collect();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
