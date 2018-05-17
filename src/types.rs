use std::collections::HashMap;

use ndarray::prelude::*;

#[derive(Debug)]
pub struct Pt2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Node {
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
    // Currently, the main use of this is to allow node ids to remain local,
    // preventing duplicates when generating shapes independently.
    pub nodes: HashMap<i32, Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct Camera {
    // Position shifts all points prior to the camera transform; this is what
    // we adjust with move keys.
    pub position: Array1<f64>,

    // θ_3d is in tait-bryan angles. 3 entries for 3d, 6 for 4d.
    pub θ_3d: Array1<f64>,
    pub θ_4d: Array1<f64>,

    pub fov: f64,  // field of view in radians.
    // near and far clipping planes
    pub n: f64,
    pub f: f64,    
}

impl Camera {
    // For now, we create a square window.
    pub fn width(&self) -> f64{
        // Calculate the projected window width, using basic trig.
        2. * self.n * (self.fov / 2.).tan()
    }
}