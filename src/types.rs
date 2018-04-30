use ndarray::prelude::*;

#[derive(Debug)]
pub struct Node {
    pub a: Array1<f64>,
    pub id: i32,
}

#[derive(Debug)]
pub struct Edge {
    pub node1: i32,  // The node's id
    pub node2: i32,
}

#[derive(Debug)]
pub struct Camera {
    pub c: Array1<f64>,

    // theta is in tait-bryan angles. Note that using the θ
    // character is currently unsupported.
    pub theta: Array1<f64>,

    // e is the viewer's position relative to teh display surface.
    pub e: Array1<f64>,
}