// This file is the entry point for WASM-WebGL rendering. It contains code that
// passes information to JS, via WASM. It includes
// structs and related code that are similar to existing ones, but only include
// wasm-bindgen-friendly types, for import and export from JS.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]
#![feature(proc_macro, wasm_custom_section, wasm_import_module)]
#![feature(const_vec_new)]

#![allow(dead_code)]  // todo remove this later
#![warn(unused_variables)] // todo remove this later

#[macro_use]
extern crate ndarray;
extern crate rand;
//extern crate simdnoise;
//extern crate noise;
extern crate wasm_bindgen;
//extern crate yew;


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;


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

    let mut scene_lib = HashMap::new();
    scene_lib.insert(0, scenes::hypercube_scene(aspect));
    scene_lib.insert(1, scenes::fivecell_scene(aspect));
    scene_lib.insert(2, scenes::spherinder_scene(aspect));
    scene_lib.insert(3, scenes::cube_scene(aspect));
    scene_lib.insert(4, scenes::pyramid_scene(aspect));
//    scene_lib.insert(5, scenes::world_scene(aspect));
    scene_lib.insert(6, scenes::grid_scene(aspect));
    scene_lib.insert(7, scenes::plot_scene(aspect));
    scene_lib.insert(8, scenes::origin_scene(aspect));

    let scene_lib: HashMap<u32, SceneBg> = scene_lib.iter()
        .map(|(id, scene)| (*id, scene.to_bg())).collect();

    JsValue::from_serde(&scene_lib).unwrap()
}


#[wasm_bindgen]
pub fn camera() -> JsValue {
    // todo pass whole scenes instead

    let cam = Camera {
            position: array![1., 1., 1., 1.],
            θ: array![1., 1., 1., 1., 2., 3.],
            fov: 1.,
            aspect: 1.,
            aspect_4: 1.,
            near: 1.,
            far: 1.,
            fourd_proj_dist: 1.,
        }.to_bg();

    JsValue::from_serde(&cam).unwrap()
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



