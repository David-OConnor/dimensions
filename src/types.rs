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

impl Node {
    pub fn augmented(&self) -> Array1<f64> {
        // For use with translation matrices, and others that have
        // the same dimension.
        array![self.a[0], self.a[1], self.a[2], self.a[3], 1.]
    }
}

// We derive clone, so we can clone edges when creating faces.
#[derive(Debug, Clone)]
pub struct Edge {
    pub node0: i32,  // The node's id
    pub node1: i32,
}

#[derive(Debug)]
pub struct Face {
    // Edges should lie in a plane, and be in an order that links them together.
    pub edges: Vec<Edge>
}

impl Face {
    pub fn surface_normal(&self) -> Array1<f64> {
        assert![self.edges.len() >= 3];

        array![]
    }
}

#[derive(Debug)]
pub struct Shape {
    // todo macro constructor that lets you ommit position, rotation, scale.
    // Shape nodes and rotation are relative to an origin of 0.
    pub nodes: HashMap<i32, Node>,
    pub edges: Vec<Edge>,
    pub faces: Vec<Face>,
    pub position: Array1<f64>,
    pub scale: f64,
    pub orientation: Array1<f64>,  // Orientation has 6 items; one for each of the 4d hyperplanes.
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
    pub fov_strange: f64,
    // near, far, and strange for our 3d and 4d frustrums.  Strange is an
    // experimental extension into the 4th dimension.
    pub clip_near: f64,
    pub clip_far: f64,
    pub clip_strange: f64
}

impl Camera {
    // For now, we create a square window.
    pub fn width(&self) -> f64{
        // Calculate the projected window width, using basic trig.
        2. * self.clip_near * (self.fov_hor / 2.).tan()
    }

    pub fn view_size(&self, far: bool) -> (f64, f64){
        // Calculate the projected window width and height, using basic trig.
        let dist = match far {
            true => self.clip_far,
            false => self.clip_near,
        };

        let width = 2. * dist * (self.fov_hor / 2.).tan();
        let height = 2. * dist * (self.fov_vert / 2.).tan();
        (width, height)
    }
}

pub struct Quaternion {
    // Borrowed heavily from cgmath.
    // Ref: https://github.com/brendanzab/cgmath/blob/master/src/quaternion.rs
    // https://en.wikipedia.org/wiki/Quaternion
    // Perhaps this should be called a Quinternion ?

    // Should we represent this as separate scalar and vector parts, or a single 4(5?)
    // vector?

    // The scalar (real) part of the quaternion.
    pub s: f64,
    // The vector (imaginary) part of the quaternion.
    pub v: Array1<f64>,
}

impl Quaternion {
    pub fn conjugate(self) -> Quaternion {
        Quaternion { s: self.s, v: -self.v }
    }

    pub fn product(&self, other: &Quaternion) -> Quaternion {
        let a = self.s * other.s - self.v[0] * other.v[0] - self.v[1] * other.v[1] - self.v[2] * other.v[2];
        let i = self.s * other.v[0] + self.v[0] * other.s + self.v[1] * other.v[2] - self.v[2] * other.v[1];
        let j = self.s * other.v[1] - self.v[0] * other.v[2] + self.v[1] * other.s + self.v[2] * other.v[0];
        let k = self.s * other.v[2] + self.v[0] * other.v[1] - self.v[1] * other.v[0] + self.v[2] * other.s;

        Quaternion { s: a, v: array![i, j, k] }
    }
}

