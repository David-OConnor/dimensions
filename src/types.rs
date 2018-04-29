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