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

    // theta is in tait-bryan angles. Note that using the θ
    // character is currently unsupported.
    pub theta: Array1<f64>,

    pub fov: f64,  // radians.
}