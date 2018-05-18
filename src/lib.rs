// This module is for use with webgl.

// We use mathematical conventions that may direct upper-case var names, 
// or lower-case constants.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![feature(non_ascii_idents)]

// #[macro_use(array)]
// #[macro_use(stack)]


#[macro_use]
extern crate ndarray;

mod types;
mod transforms;
mod shape_maker;
mod clipping;
mod render_webgl;

use std::os::raw::c_char;
use std::ffi::CString;
use std::collections::HashMap;

#[no_mangle]
pub fn get_data() -> *mut c_char {
    let mut data = HashMap::new();
    data.insert("Alice", "send");
    data.insert("Bob", "recieve");
    data.insert("Carol", "intercept");

    let descriptions = data.iter()
        .map(|(p,a)| format!("{} likes to {} messages", p, a))
        .collect::<Vec<_>>();

    CString::new(descriptions.join(", "))
        .unwrap()
        .into_raw()
}

fn main() {
    // Deliberately blank.
}