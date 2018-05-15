use ndarray::prelude::*;

#[derive(Debug)]
pub struct Pt2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug)]
pub struct Node {
    pub a: Array1<f64>,
    pub id: i32,
}

impl Node {
    pub fn make_4d(&self) -> Node {
        assert!(self.a.len() == 3);

        Node {a: stack![Axis(1), array![0.]], id: self.id}
    }
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
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub id: i32,
}

#[derive(Debug)]
pub struct Camera {
    // Position shifts all points prior to the camera transform; this is what
    // we adjust with move keys.
    pub position: Array1<f64>,

    // θ is in tait-bryan angles. Note that using the θ
    // character is currently unsupported.
    pub θ: Array1<f64>,

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