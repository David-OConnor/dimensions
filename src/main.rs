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

#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

#[macro_use]
extern crate ndarray;
extern crate rand;
extern crate simdnoise;
extern crate noise;
#[macro_use]
extern crate serde_derive;

// Vulkano
#[macro_use]
extern crate vulkano;
#[macro_use]
extern crate vulkano_shader_derive;
extern crate vulkano_win;
extern crate wasm_bindgen;
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
mod util;

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
