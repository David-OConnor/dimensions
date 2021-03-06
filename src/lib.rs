// This file is the entry point for WASM-WebGL rendering. It contains code that
// passes information to JS, via WASM. It includes
// structs and related code that are similar to existing ones, but only include
// wasm-bindgen-friendly types, for import and export from JS.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]
#![feature(use_extern_macros, proc_macro_span, proc_macro_raw_ident)]
#![feature(wasm_custom_section, wasm_import_module)]
#![feature(const_vec_new)]

#![allow(dead_code)]  // todo remove this later
#![warn(unused_variables)] // todo remove this later

#[macro_use]
extern crate ndarray;
extern crate rand;
//extern crate simdnoise;
extern crate noise;
extern crate wasm_bindgen;
//extern crate yew;


#[macro_use]
extern crate serde_derive;

mod util;
mod scenes;
mod shape_maker;
mod transforms;
mod types;

use std::collections::HashMap;

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;
//use yew::prelude::*;

use types::{Camera, CameraBg, Shape, ShapeBg, SceneBg};

// WIDTH and HEIGHT should match WebGL canvas size.
const WIDTH: u32 = 1024;
const HEIGHT: u32 = 768;

#[wasm_bindgen]
pub fn scene_lib() -> JsValue {
    let aspect = WIDTH as f32 / HEIGHT as f32;

    // todo this is duped from render_vulkano.
    let mut scene_lib = HashMap::new();
    scene_lib.insert(0, scenes::hypercube_scene(aspect));
    scene_lib.insert(1, scenes::fivecell_scene(aspect));
    scene_lib.insert(10, scenes::twentyfourcell_scene(aspect));
    scene_lib.insert(2, scenes::spherinder_scene(aspect));
    scene_lib.insert(3, scenes::cube_scene(aspect));
    scene_lib.insert(4, scenes::pyramid_scene(aspect));
    scene_lib.insert(5, scenes::world_scene(aspect));
    scene_lib.insert(6, scenes::grid_scene(aspect));
    scene_lib.insert(7, scenes::grid_scene_4d(aspect));
    scene_lib.insert(8, scenes::plot_scene(aspect));
    scene_lib.insert(9, scenes::origin_scene(aspect));

    let scene_lib: HashMap<u32, SceneBg> = scene_lib.iter()
        .map(|(id, scene)| (*id, scene.to_bg())).collect();

    JsValue::from_serde(&scene_lib).unwrap()
}

fn mat_as_js(mat: [[f32; 4]; 4]) -> Vec<f32> {
    // Prep the array for JS, which uses flat Float32arrays passed as Vecs instead of
    // 2d arrays.
    let mut result = Vec::new();
    for row in &mat {
        result.append(&mut row.to_vec());
    }
    result
}

#[wasm_bindgen]
pub fn view_mat(θ: Vec<f32>) -> Vec<f32> {
    let mat = transforms::make_view_mat4(&Array::from_vec(θ));
    mat_as_js(mat)
}

#[wasm_bindgen]
pub fn model_mat(orientation: Vec<f32>, scale: f32) -> Vec<f32> {
    let mat = transforms::make_model_mat4(&Array::from_vec(orientation), scale);
    mat_as_js(mat)
}

#[wasm_bindgen]
pub fn proj_mat(position: Vec<f32>, θ: Vec<f32>, fov: f32, aspect: f32, aspect_4: f32,
                 near: f32, far: f32, fourd_proj_dist: f32) -> Vec<f32> {
    // We can't pass the camera directly due to bindgen limitations.
    let cam = Camera {
        position: Array::from_vec(position), θ: Array::from_vec(θ),
        fov, aspect, aspect_4, near, far, fourd_proj_dist
    };

    let mat = transforms::make_proj_mat_gl(&cam);
    mat_as_js(mat)
}

#[wasm_bindgen]
pub fn rotator(θ: Vec<f32>) -> Vec<f32> {
    // Note: We don't use this since it's faster to simply create these in JS.
    let mat = transforms::make_rotator4(&Array::from_vec(θ));
    mat_as_js(mat)
}

//struct Model { }
//
//enum Msg {
//    DoIt,
//}
//
//impl Component for Model {
//    // Some details omitted. Explore the examples to see more.
//
//    type Message = Msg;
//    type Properties = ();
//
//    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
//        Model { }
//    }
//
//    fn update(&mut self, msg: Self::Message) -> ShouldRender {
//        match msg {
//            Msg::DoIt => {
//                // Update your model on events
//                true
//            }
//        }
//    }
//}
//
//impl Renderable<Model> for Model {
//    fn view(&self) -> Html<Self> {
//        html! {
//            // Render your model here
//            <button onclick=|_| Msg::DoIt,>{ "Click me!" }</button>
//        }
//    }
//}
//
//fn main() {
//    yew::initialize();
//    App::<Model>::new().mount_to_body();
//    yew::run_loop();
//}
//



