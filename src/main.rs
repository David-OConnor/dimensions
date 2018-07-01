// This file is the entry point for the standalone Rust renderer.

// We use mathematical conventions that may direct upper-case var names,
// or lower-case constants.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]  // todo remove this later
#![allow(unused_variables)] // todo remove this later
#![allow(unused_imports)] // todo remove this later
#![feature(non_ascii_idents)]
#![feature(vec_remove_item)]

// todo temp:
#![feature(core_intrinsics)]

extern crate cgmath;
extern crate simdnoise;
#[macro_use]
extern crate ndarray;

// Vulkano
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate winit;

use std::collections::HashMap;

mod clipping;
mod input;
mod scenes;
mod shaders;
mod shape_maker;
mod types;
mod transforms;
mod render_vulcano;

fn main() {
    render_vulcano::render();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cam_transform() {
    }
}
