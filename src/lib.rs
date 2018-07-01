// This file is the entry point for WASM-WebGL rendering. It contains code that
// passes information to JS, via WASM. It includes
// structs and related code that are similar to existing ones, but only include
// wasm-bindgen-friendly types, for import and export from JS.

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]
#![feature(proc_macro, wasm_custom_section, wasm_import_module)]

#![allow(dead_code)]  // todo remove this later
#![warn(unused_variables)] // todo remove this later

//extern crate stdweb;
// Can't use procedural macros with #[macro_use].
//use stdweb::js_export;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

//#[js_export]
#[wasm_bindgen]
pub extern fn test() -> String {
    String::from("It's hard to overstate my satisfaction.")
}


