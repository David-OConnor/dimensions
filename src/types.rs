use std::collections::HashMap;

use ndarray::prelude::*;

#[derive(Debug)]
pub struct Pt2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Node {
    // a may be relative or absolute.
    pub a: Array1<f64>,
}

// We derive clone on edge for when copying it, unchanged, into a new shape
// when transforming.
#[derive(Debug, Clone)]
pub struct Edge {
    pub node1: i32,  // The node's id
    pub node2: i32,
}

#[derive(Debug)]
pub struct Shape {
    // todo macro constructor that lets you ommit position, rotation, scale.
    // Shape nodes and rotation are relative to an origin of 0.
    pub nodes: HashMap<i32, Node>,
    pub edges: Vec<Edge>,
    pub position: Array1<f64>,
    pub scale: f64,
    pub rotation: Array1<f64>,  // Rotation has 6 items; one for each of the 4d hyperplanes.
    pub rotation_speed: Array1<f64>,  // 6 items, as with rotation.  Radians/s ?
}

#[derive(Debug)]
pub struct Camera {
    // Position shifts all points prior to the camera transform; this is what
    // we adjust with move keys.
    pub position: Array1<f64>,

    // θ_3d is in tait-bryan angles. 3 entries for 3d, 6 for 4d.
    pub θ_3d: Array1<f64>,
    pub θ_4d: Array1<f64>,

    pub fov_hor: f64,  // field of view in radians.
    pub fov_vert: f64, // vertical FOV. Unused currently.
    // near and far clipping planes
    pub n: f64,
    pub f: f64,    
}

impl Camera {
    // For now, we create a square window.
    pub fn width(&self) -> f64{
        // Calculate the projected window width, using basic trig.
        2. * self.n * (self.fov_hor / 2.).tan()
    }
}

impl Camera {
    pub fn view_size(&self, far: bool) -> (f64, f64){
        // Calculate the projected window width and height, using basic trig.
        let dist = match far {
            true => self.f,
            false => self.n,
        };

        let width = 2. * dist * (self.fov_hor / 2.).tan();
        let height = 2. * dist * (self.fov_vert / 2.).tan();
        (width, height)
    }
}