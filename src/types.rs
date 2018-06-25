use std::collections::HashMap;

use ndarray::prelude::*;

use vulkano;

#[derive(Debug)]
pub struct Pt2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    // a may be relative or absolute.
//    pub position: [f32; 4],
    pub position: (f32, f32, f32, f32),
}
impl_vertex!(Vertex, position);

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, u: f32) -> Vertex {
        Vertex{ position: (x, y, z, u) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VertAndExtras {
    // Used to pass attributes that go with each vertex.
    // todo: Shape and cam don't need to be recalced for each vertex...
    pub position: (f32, f32, f32, f32),
    pub shape_posit: (f32, f32, f32, f32),
    pub normal: (f32, f32, f32, f32),
}
impl_vertex!(VertAndExtras, position, shape_posit, normal);

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32, f32)
}
impl_vertex!(Normal, normal);

impl Normal {
    pub fn new(x: f32, y: f32, z: f32, u: f32) -> Normal {
//        Vertex{ position: [x, y, z, u] }
        Normal{ normal: (x, y, z, u) }
    }
}

// We derive clone, so we can clone edges when creating faces.
#[derive(Clone, Debug)]
pub struct Edge {
    pub node0: u32,  // The node's id
    pub node1: u32,
}

#[derive(Clone, Debug)]
pub struct Face {
    // Edges should lie in a plane, and be in an order that links them together.
    pub edges: Vec<Edge>
}

impl Face {
    pub fn surface_normal(&self) -> Array1<f32> {
        assert![self.edges.len() >= 3];

        array![]
    }
}

#[derive(Clone, Debug)]
pub struct Shape {
    // todo macro constructor that lets you ommit position, rotation, scale.
    // Shape nodes and rotation are relative to an origin of 0.
    pub vertices: HashMap<u32, Vertex>,
    pub edges: Vec<Edge>,  // todo Edges is currently unused.
    pub faces: Vec<Face>,  // todo Faces is currently unused.
    pub faces_vert: Vec<Array1<u32>>,
    pub normals: Vec<Normal>,
    pub position: Array1<f32>,
    pub scale: f32,
    pub orientation: Array1<f32>,  // Orientation has 6 items; one for each of the 4d hyperplanes.
    pub rotation_speed: Array1<f32>,  // 6 items, as with rotation.  Radians/s ?
    pub per_face_vertices: Vec<Vertex>,
    pub tris: Array1<u32>,
}

impl Shape {
    pub fn new(nodes: HashMap<u32, Vertex>, edges: Vec<Edge>, faces: Vec<Face>,
               faces_vert: Vec<Array1<u32>>, normals: Vec<Normal>,
               position: Array1<f32>, orientation: Array1<f32>,
               rotation_speed: Array1<f32>) -> Shape {

        // todo use this, like in JS.
        let per_face_vertices = {
            let mut shape_vertices = vec![];
            for face in &faces_vert {
                for id in face {
                    shape_vertices.push(nodes[id]);
                }
            }
            shape_vertices
        };

        Shape{ vertices: nodes, edges, faces, faces_vert, normals, position, scale: 1., orientation,
               rotation_speed, per_face_vertices, tris: array![]}
    }


//    pub fn getTris(&mut self) -> &Array1<u32> {
//        // get cached triangles if avail; if not, create and cache.
//        if !self.tris.len() > 0 {
//            self.makeTris()
//        }
//        &self.tris
//    }
//    pub fn getTris(&self) -> &Array1<u32> {
//        // get cached triangles if avail; if not, create and cache.
//        &self.tris
//    }

    pub fn make_tris(&mut self) {
        // Divide faces into triangles of indices. These indices aren't of node
        // ids; rather of cumulative node ids; eg how they'll appear in an index buffer.
        // Result is a 1d array; Float32array-style.
        let mut result = Vec::new();
        let mut current_i = 0;

        for face in &self.faces_vert {
            match face.len() {
                3 => {
                // Only one triangle.
                result.push(current_i as u32);
                result.push(current_i as u32 + 1);
                result.push(current_i as u32 + 2);
            },
                4 => {
                // First triangle
                result.push(current_i as u32);
                result.push(current_i as u32 + 1);
                result.push(current_i as u32 + 2);
                // Second triangle
                result.push(current_i as u32);
                result.push(current_i as u32 + 2);
                result.push(current_i as u32 + 3);
            },
                2 => panic!("Faces must have length 3 or more."),
                _ => panic!("Error: Haven't implemented faces with vertex counds higher than four.")

            }
            current_i += face.len();
        }
        self.tris = Array::from_vec(result)
    }

    pub fn num_face_verts(&self) -> u32 {
        // Find the number of vertices used in drawing faces.  Ie for a box,
        // it's 6 faces x 4 vertices/face.
        self.faces_vert.iter().fold(0, |acc, face| acc + face.len() as u32)
    }
}

#[derive(Clone, Debug)]
pub struct Camera {
    // Position shifts all points prior to the camera transform; this is what
    // we adjust with move keys.
    pub position: Array1<f32>,
    pub θ: Array1<f32>,

    pub fov: f32,  // Vertical field of view in radians.
    pub aspect: f32,  // width / height.
    pub aspect_4: f32,  // fourth dim / height.
    // near, far, and strange for our 3d and 4d frustrums.  Strange is an
    // experimental extension into the 4th dimension.
    pub near: f32,
    pub far: f32,
    pub strange: f32
}

impl Camera {
    pub fn view_size(&self, far: bool) -> (f32, f32){
        // Calculate the projected window width and height, using basic trig.
        let dist = if far { self.far } else { self.near };

        let width = 2. * dist * (self.fov * self.aspect / 2.).tan();
        let height = 2. * dist * (self.fov / 2.).tan();
        (width, height)
    }
}

#[derive(Clone, Debug)]
pub enum CameraType {
    Single,  // No camera changes; rotate the shape instead
    // Move foward, back, left, right, and look around. No roll look.  Not sure
    // which 4d rotations/movement to allow or block.
    FPS,
    Free, // No restriction on movement
}

#[derive(Debug)]
pub struct Scene {
    pub id: u32,
    pub shapes: HashMap<u32, Shape>,
    pub cam_start: Camera,
    pub cam_type: CameraType,
    pub ambient_light_color: [f32; 4],
    pub diffuse_light_color: [f32; 4],
    pub diffuse_light_direction: [f32; 4],
    pub color_max: f32, // distance thresh for max 4d-color indicator.
}

pub struct _Quaternion {
    // Borrowed heavily from cgmath.
    // Ref: https://github.com/brendanzab/cgmath/blob/master/src/quaternion.rs
    // https://en.wikipedia.org/wiki/Quaternion
    // Perhaps this should be called a Quinternion ?

    // Should we represent this as separate scalar and vector parts, or a single 4(5?)
    // vector?

    // The scalar (real) part of the quaternion.
    pub s: f32,
    // The vector (imaginary) part of the quaternion.
    pub v: Array1<f32>,
}

impl _Quaternion {
    pub fn conjugate(self) -> _Quaternion {
        _Quaternion { s: self.s, v: -self.v }
    }

    pub fn product(&self, other: &_Quaternion) -> _Quaternion {
        let a = self.s * other.s - self.v[0] * other.v[0] - self.v[1] * other.v[1] - self.v[2] * other.v[2];
        let i = self.s * other.v[0] + self.v[0] * other.s + self.v[1] * other.v[2] - self.v[2] * other.v[1];
        let j = self.s * other.v[1] - self.v[0] * other.v[2] + self.v[1] * other.s + self.v[2] * other.v[0];
        let k = self.s * other.v[2] + self.v[0] * other.v[1] - self.v[1] * other.v[0] + self.v[2] * other.s;

        _Quaternion { s: a, v: array![i, j, k] }
    }
}

