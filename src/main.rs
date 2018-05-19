// We use mathematical conventions that may direct upper-case var names, 
// or lower-case constants.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]

// #[macro_use(array)]
// #[macro_use(stack)]

#[macro_use]
extern crate ndarray;
extern crate ggez;

// GFX crate imports here.
// #[macro_use]
// // extern crate gfx;
// extern crate gfx_window_glutin;
// extern crate glutin;
// extern crate env_logger;
// extern crate gfx_hal as hal;
// #[cfg(feature = "vulkan")]
// extern crate gfx_backend_vulkan as back;
// #[cfg(feature = "metal")]
// extern crate gfx_backend_metal as back;
// #[macro_use]
// extern crate gfx_render as gfx;
// extern crate winit;
// extern crate image;

mod types;
mod render_ggez;
mod transforms;
mod shape_maker;
mod clipping;
// mod render_gfx;

use std::collections::HashMap;

fn main() {
    let empty_6 = array![0., 0., 0., 0., 0., 0.];

    let shape_vec = vec![
        shape_maker::make_cube(1., array![-1.5, 0., -1.5, 0.], empty_6.clone(), empty_6.clone()),
        shape_maker::make_box(1.5, 0.5, 2.5, array![2., 0., 0., 0.], empty_6.clone(), empty_6.clone()),
        shape_maker::make_rectangular_pyramid(2., 1.5, 0.5, array![-2., -0., 2.0, 0.],
                                              empty_6.clone(), empty_6.clone()),
        // shape_maker::make_house(&array![-3., 0., -3., 0.], 1., 1., 1.),

        // Marker rectangles: FL, FR, BR, BL
        shape_maker::make_box(2., 0.2, 1., array![-4., 0., 4., 0.], empty_6.clone(), empty_6.clone()),
        shape_maker::make_box(1., 2., 1., array![-4., 0., 3., 0.], empty_6.clone(), empty_6.clone()),

        shape_maker::make_box(2., 0.2, 1., array![4., 0., 4., 0.], empty_6.clone(), empty_6.clone()),
        shape_maker::make_box(1., 1., 1., array![5., 0., 3., 0.], empty_6.clone(), empty_6.clone()),

        shape_maker::make_box(2., 0.2, 1., array![4., 0., -4., 0.], empty_6.clone(), empty_6.clone()),
        shape_maker::make_box(1., 0.5, 1., array![5., 0., -3., 0.], empty_6.clone(), empty_6.clone()),

        shape_maker::make_box(1., 0.5, 1., array![-4., 0., -4., 0.], empty_6.clone(), empty_6.clone()),
        shape_maker::make_box(1., 0.2, 1., array![-4., 0., -3., 0.], empty_6.clone(), empty_6.clone()),

        // shape_maker::make_street(&array![0., 0., 2.], array![0., 0., 0.], 1.),

        shape_maker::make_origin(1., array![0., 0., 0., 0.], empty_6.clone(), empty_6.clone()),
        shape_maker::make_hypercube(1., array![1.5, 0., -2.0, 0.], empty_6.clone(), empty_6.clone()),
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
