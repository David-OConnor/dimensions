// We use mathematical conventions that may direct upper-case var names, 
// or lower-case constants.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]

#[macro_use(array)]
#[macro_use(stack)]

extern crate ndarray;
extern crate ggez;

#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;

mod types;
mod drawing;
mod transforms;
mod shape_maker;
mod clipping;
mod gfx_render;

use std::collections::HashMap;

fn main() {
    let shape_vec = vec![
        shape_maker::make_cube(&array![-1.5, 0., -1.5, 0.], 1.),
        shape_maker::make_box(&array![2., 0., 0., 0.], 1.5, 0.5, 2.5),
        shape_maker::make_rectangular_pyramid(&array![-2., -0., 2.0, 0.], 2., 1.5, 0.5),
        shape_maker::make_house(&array![-3., 0., -3., 0.], 1., 1., 1.),

        // Marker rectangles: FL, TR, BR, BL
        shape_maker::make_box(&array![-4., 0., 4., 0.], 2., 0.2, 1.),
        shape_maker::make_box(&array![-4., 0., 3., 0.], 1., 2., 1.),

        shape_maker::make_box(&array![4., 0., 4., 0.], 2., 0.2, 1.),
        shape_maker::make_box(&array![4., 0., 3., 0.], 1., 1., 1.),

        shape_maker::make_box(&array![4., 0., -4., 0.], 2., 0.2, 1.),
        shape_maker::make_box(&array![4., 0., -3., 0.], 1., 0.5, 1.),

        shape_maker::make_box(&array![-4., 0., -4., 0.], 2., 0.2, 1.),
        shape_maker::make_box(&array![-4., 0., -3., 0.], 1., 0.2, 1.),

        // shape_maker::make_street(&array![0., 0., 2.], array![0., 0., 0.], 1.),

        shape_maker::make_origin(&array![0., 0., 0., 0.], 1.),
        shape_maker::make_hypercube(&array![1.5, 0., -2.0, 0.], 1.),
    ];

    let shapes = HashMap::new();
    for (id, shape) in shape_vec.into_iter().enumerate() {
        shapes.insert(id, shape);
    }

    drawing::run(shapes, false);
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
