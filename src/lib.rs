// This file is the entry point for WASM-WebGL rendering. It contains code that
// passes information to JS, via WASM. It includes
// structs and related code that are similar to existing ones, but only include
// wasm-bindgen-friendly types, for import and export from JS.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]
#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

#[macro_use]
extern crate ndarray;

#[macro_use]
extern crate wasm_bindgen;

//extern crate stdweb;

//extern crate serde;
//#[macro_use]
//extern crate serde_json;
//#[macro_use]
//extern crate serde_derive;


mod shape_maker;
mod transforms;
mod types;

use std::collections::HashMap;

use ndarray::prelude::*;
use wasm_bindgen::prelude::*;
//use stdweb::js_export;

use types::{Camera, Shape};



//#[wasm_bindgen]
//#[derive(Debug, Serialize)]
#[derive(Debug)]
pub struct ProjectedPt3d {
    // This includes the id tuple as separate items, since wasm_bindgen can't
    // handle HashMaps  or JS Maps.
    shape_id: i32,
    node_id: i32,
    x: f64,
    y: f64,
    z: f64,
}

//js_serializable!( ProjectedPt3d );

//
//#[derive(Debug)]
//#[wasm_bindgen]
//pub struct ShapeArgs {
//    // Used for hypercubes etc
//    name: String,  // eg "cube", "hypercube"
//    // len of 1 for cubes/origin, 3 for 3d boxes, 4 for 4d boxes etc.
//    // eg vec![x_len, y_len, z_len] for a 3d box.
//    lens: Vec<f64>,
//    position: Vec<f64>,
//    scale: f64,
//    orientation: Vec<f64>,
//    rotation_speed: Vec<f64>,
//}
//
//fn to_shapes(shape_argss: Vec<ShapeArgs>) -> HashMap<i32, Shape> {
//    let mut result = HashMap::new();
//
//    for (id, args) in shape_argss.into_iter().enumerate() {
//        let shape = match args.name.as_ref() {
//            "box" => shape_maker::make_box(
//                args.lens[0], args.lens[1], args.lens[2],
//                Array::from_vec(args.position),
//                args.scale,
//                Array::from_vec(args.orientation),
//                Array::from_vec(args.rotation_speed)
//            ),
//            "cube" =>shape_maker::make_cube(
//                args.lens[0],
//                Array::from_vec(args.position),
//                args.scale,
//                Array::from_vec(args.orientation),
//                Array::from_vec(args.rotation_speed)
//            ),
//            "pyramid" =>shape_maker::make_rectangular_pyramid(
//                args.lens[0], args.lens[1], args.lens[2],
//                Array::from_vec(args.position),
//                args.scale,
//                Array::from_vec(args.orientation),
//                Array::from_vec(args.rotation_speed)
//            ),
//            "origin" => shape_maker::make_origin(
//                args.lens[0],
//                Array::from_vec(args.position),
//                args.scale,
//                Array::from_vec(args.orientation),
//                Array::from_vec(args.rotation_speed)
//            ),
//            "hypercube" => shape_maker::make_hypercube(
//                args.lens[0],
//                Array::from_vec(args.position),
//                args.scale,
//                Array::from_vec(args.orientation),
//                Array::from_vec(args.rotation_speed)
//            ),
//            _ => panic!["Shape named passed from JS must match a shape\
//        described in shape_maker.rs."]
//        };
//        result.insert(id as i32, shape);
//    }
//    result
//}

//
////#[wasm_bindgen]
////#[derive(Debug)]
//pub struct CameraBindgen {
//    // This camera replaces ndarrays with vectors, for bindgen compatibility.
//    pub position: Vec<f64>,
//
//    // θ_3d is in tait-bryan angles. 3 entries for 3d, 6 for 4d.
//    pub theta_3d: Vec<f64>,
//    pub theta_4d: Vec<f64>,
//
//    pub fov_hor: f64,
//    pub fov_vert: f64,
//    pub fov_strange: f64,
//    pub clip_near: f64,
//    pub clip_far: f64,
//    pub clip_strange: f64
//}
//
//impl CameraBindgen {
//    fn to_camera(&self) -> Camera {
//        Camera {
//            position:Array::from_vec(self.position.clone()),
//            θ_3d: Array::from_vec(self.theta_3d.clone()),
//            θ_4d: Array::from_vec(self.theta_4d.clone()),
//            fov_hor: self.fov_hor,
//            fov_vert: self.fov_vert,
//            fov_strange: self.fov_strange,
//            clip_near: self.clip_near,
//            clip_far: self.clip_far,
//            clip_strange: self.clip_strange,
//        }
//    }
//}

fn project_shapes_4d_to_3d(shapes: HashMap<i32, Shape>, camera: &Camera,
                           R_4d: &Array2<f64>)
                           -> Vec<ProjectedPt3d> {
    // Used for passing 3d shapes to WebGL; differs from project_shapes_3d in
    // that it skips the projection from 4d to 3d, and outputs in a
    // bindgen-friendly format.
    assert![R_4d.rows() == 5 && R_4d.cols() == 5];

    let mut result = Vec::new();

    for (shape_id, shape) in &shapes {
        let positioned_nodes = transforms::position_shape(shape);
        for (node_id, node) in &positioned_nodes {
            let projected = transforms::project(camera, R_4d, &node, true);
            result.push(ProjectedPt3d {
                shape_id: *shape_id,
                node_id: *node_id,
                x: projected[0],
                y: projected[1],
                z: projected[2],
            });
        }
    }
    result
}




//#[js_export]
//#[no_mangle]
#[wasm_bindgen]
pub extern fn render_from_js() -> u32 {
//                      shape_args: ShapeArgs) {
//                      shape_argss: Vec<ShapeArgs>) -> JsValue {

//    println!("Camera: {:?}", camera);
//    println!("Shape_argss: {:?}", shape_argss);

//    let shapes = to_shapes(shape_argss);
//    let shapes = to_shapes(vec![shape_args]);

//    println!("Cam: {}", &camera);
//    println!("Args: {}", &shape_argss);
//    let camera_rust = camera.to_camera();
//
//    let result = project_shapes_4d_to_3d(
//        shapes,
//        &camera_rust,
//        &transforms::rotate_4d(&camera_rust.θ_4d)
//    );
//    JsValue::from_serde(&result).unwrap()

    let mut test = HashMap::new();
    test.insert("Whatd", 1);

    test.insert("YO", 2);

//    serde::serialize(test);

    String::from("Huge success");
    3

//    JsValue::from_serde(&test).unwrap()
}

#[no_mangle]
pub extern fn add_one(a: u32) -> u32 {
    a + 1
}
