// We use mathematical conventions that may direct upper-case var names.
#![allow(non_snake_case)]

#[macro_use(array)]
extern crate ndarray;
extern crate ggez;

mod types;
mod drawing;
mod transforms;
mod shape_maker;

use std::f64::consts::PI;
use ndarray::prelude::*;

use types::{Node, Edge, Camera};

const _TAU: f64 = 2. * PI;

fn main() {
    const _FOV: f64 = 80.;  // Degrees.
    
    let shapes = vec![
        shape_maker::make_cube(array![0., 0., 0.], 1., 0),
        shape_maker::make_box(array![2., 1., 0.], 1.5, 0.5, 2.5, 1),
        shape_maker::make_rectangular_pyramid(array![-1., -1., -3.], 2., 1.5, 0.5, 2),
    ];

    drawing::run(shapes);
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
