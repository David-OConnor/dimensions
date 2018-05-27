// This file is the entry point for the standalone Rust renderer.

// We use mathematical conventions that may direct upper-case var names,
// or lower-case constants.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]

#[macro_use]
extern crate ndarray;
extern crate ggez;

mod types;
mod render_ggez;
mod transforms;
mod shape_maker;
mod clipping;

use std::collections::HashMap;

fn main() {
    let empty_6 = array![0., 0., 0., 0., 0., 0.];

    let shape_vec = vec![
        shape_maker::make_cube(1., array![-0., 0., 3.0, 0.], 1., empty_6.clone(),
                               array![0.0, 0., 0., 0., 0., 0.]),


//        shape_maker::make_box(1.5, 0.5, 2.5, array![6., 0., 5., 0.], 1., empty_6.clone(), empty_6.clone()),
        shape_maker::make_rectangular_pyramid(2., 1.5, 0.5, array![-5., -0., 5.0, 0.], 1.,
                                              empty_6.clone(), array![0.001, -0.002, 0., 0., 0., 0.]),
        // shape_maker::make_house(&array![-3., 0., -3., 0.], 1., 1., 1.),

        // shape_maker::make_street(&array![0., 0., 2.], array![0., 0., 0.], 1.),

        shape_maker::make_origin(1., array![0., 0., 0., 0.], 1., empty_6.clone(), empty_6.clone()),
        shape_maker::make_hypercube(1., array![6.5, 0., -5.0, 0.], 1.,
                                    empty_6.clone(),
                                    array![0., 0., 0., 0.005, 0.005, 0.005]),
//
        shape_maker::make_hypercube(1., array![0., 0., 0.0, 0.], 1.,
                                    empty_6.clone(),
                                    array![0.0, 0., 0., 0.005, 0.0, 0.0]),
    ];

    let mut shapes = HashMap::new();
    for (id, shape) in shape_vec.into_iter().enumerate() {
        shapes.insert(id as i32, shape);
    }

    render_ggez::run(shapes, false);
    // gfx_render::main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cam_transform() {
    }
}
