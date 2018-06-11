// This file is the entry point for the standalone Rust renderer.

// We use mathematical conventions that may direct upper-case var names,
// or lower-case constants.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]  // todo remove this later
#![allow(unused_variables)] // todo remove this later
#![allow(unused_imports)] // todo remove this later
#![feature(non_ascii_idents)]

#[macro_use]
extern crate ndarray;

// Vulkano
extern crate cgmath;
extern crate image;
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate winit;
extern crate vulkano_win;

use vulkano_win::VkSurfaceBuild;

use vulkano::device::Device;
use vulkano::instance::Instance;
use vulkano::swapchain;
use vulkano::swapchain::PresentMode;
use vulkano::swapchain::SurfaceTransform;
use vulkano::swapchain::Swapchain;
use vulkano::swapchain::AcquireError;
use vulkano::swapchain::SwapchainCreationError;
use vulkano::sync::now;
use vulkano::sync::GpuFuture;

//use cgmath::Matrix4;
//use cgmath::SquareMatrix;
//use cgmath::Vector3;

use std::mem;

use std::collections::HashMap;

mod clipping;
mod shape_maker;
mod types;
mod transforms;
mod render_vulcano;

fn main() {
    let empty_6 = array![0., 0., 0., 0., 0., 0.];

    let shape_vec = vec![
        shape_maker::make_cube(1., array![-3., 0., 3.0, 0.], empty_6.clone(),
                               array![0.0, 0., 0., 0., 0., 0.]),
        // copycat cube in a dif 4d posit!
        shape_maker::make_hypercube(1., array![-3., 0., 3.0, 3.], empty_6.clone(),
                       array![0.0, 0., 0., 0., 0., 0.]),


//        shape_maker::make_box(1.5, 0.5, 2.5, array![6., 0., 5., 0.], empty_6.clone(), empty_6.clone()),
        shape_maker::make_rectangular_pyramid((1., 1., 1.), array![-3., 2., -3., 0.],
                                              empty_6.clone(), array![0.001, -0.002, 0., 0., 0., 0.]),
        shape_maker::make_house((1., 1., 1.), array![3., 0., 3., 0.], empty_6.clone(),
                                array![0.0, 0., 0., 0., 0., 0.]),

        // shape_maker::make_street(&array![0., 0., 2.], array![0., 0., 0.], 1.),

        shape_maker::make_origin(1., array![0., 0., 0., 0.], empty_6.clone(), empty_6.clone()),
        shape_maker::make_hypercube(1., array![6.5, 0., -5.0, 0.],
                                    empty_6.clone(),
                                    array![0., 0., 0., 0.005, 0.005, 0.005]),

        shape_maker::make_hypercube(1., array![0., 0., 0.0, 0.],
                                    empty_6.clone(),
                                    array![0.0, 0., 0., 0.005, 0.0, 0.0]),
    ];

    let mut shapes = HashMap::new();
    for (id, shape) in shape_vec.into_iter().enumerate() {
        shapes.insert(id as u32, shape);
    }

//    render_ggez::run(shapes);
    render_vulcano::main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cam_transform() {
    }
}
