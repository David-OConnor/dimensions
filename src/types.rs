use std::collections::HashMap;

use ndarray::prelude::*;

#[derive(Debug)]
pub struct Pt2D {
    pub x: f32,
    pub y: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub position: (f32, f32, f32, f32),
}

impl Vertex {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vertex {
        Vertex{ position: (x, y, z, w) }
    }

    pub fn subtract(&self, other: &Vertex) -> Vertex {
        Vertex::new(self.position.0 - other.position.0, self.position.1 - other.position.1,
                    self.position.2 - other.position.2, self.position.3 - other.position.3,)
    }

    pub fn cross(&self, other: &Vertex) -> Normal {
        // Ignores the u component; cross product isn't defined for len-4 vectors.
        Normal::new(
            self.position.1 * other.position.2 - self.position.2 * other.position.1,
            self.position.2 * other.position.0 - self.position.0 * other.position.2,
            self.position.0 * other.position.1 - self.position.1 * other.position.0,
            0.
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub struct VertAndExtras {
    // Used to pass attributes that go with each vertex to the shader.
    // We do the impl_vertex in render_vulkano, so we don't need to import vulkano
    // in the wasm target.
    pub position: (f32, f32, f32, f32),
    pub shape_posit: (f32, f32, f32, f32),
    pub normal: (f32, f32, f32, f32),
    pub specular_intensity: f32,
}

#[derive(Copy, Clone, Debug)]
pub struct Normal {
    pub normal: (f32, f32, f32, f32)
}

impl Normal {
    // Only really uses the 3d part of the shape, for now.
    pub fn new(x: f32, y: f32, z: f32, u: f32) -> Normal {
        Normal{ normal: (x, y, z, u) }
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vertices: HashMap<u32, Vertex>,
    pub faces_vert: Vec<Array1<u32>>,  // Indicies of vertexes.
    pub normals: Vec<Normal>,  // Normals only use the 3d component; not defined for 4d, yet. ?
    pub tris: Array1<u32>,
}

impl Mesh {
    pub fn new(vertices: HashMap<u32, Vertex>,
               faces_vert: Vec<Array1<u32>>, normals: Vec<Normal>) -> Mesh {

        let mut result = Mesh {vertices, faces_vert, normals, tris: array![]};
        result.make_tris();
        result
    }

        pub fn make_tris(&mut self) {
        // Divide faces into triangles of indices. These indices aren't of node
        // ids; rather of cumulative node ids; eg how they'll appear in an index buffer.
        // Result is a 1d array.
        // Important: Faces must be defined in an order of consecutive edges.
        // If we modify/add faces, we must re-run this.
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
    
    pub fn to_bg(&self) -> MeshBg {
        let mut vertices = HashMap::new();
        for (id, vert) in &self.vertices {
            vertices.insert(*id, vec![vert.position.0, vert.position.1,
                                     vert.position.2, vert.position.3]);
        }

        let normals: Vec<Vec<f32>> = self.normals.iter()
            .map(|norm| vec![norm.normal.0, norm.normal.1, norm.normal.2,
                             norm.normal.3]).collect();

        MeshBg {
            vertices,
            faces_vert: self.faces_vert.iter().map(|face| face.to_vec()).collect(),
            normals,
            tris: self.tris.to_vec(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
//#[wasm_bindgen]
pub struct MeshBg {
    // Uses only types accepted by WASM Bindgen. No pub fields for vecs.
    vertices: HashMap<u32, Vec<f32>>,
    faces_vert: Vec<Vec<u32>>,  // Indicies of vertexes.
    normals: Vec<Vec<f32>>,  // Normals only use the 3d component; not defined for 4d, yet. ?
    tris: Vec<u32>,
}

#[derive(Clone, Debug, Serialize)]
//#[wasm_bindgen]
pub struct ShapeBg {
    // See note on MeshBg.
    mesh: MeshBg,
    position: Vec<f32>,
    scale: f32,
    orientation: Vec<f32>,  // Orientation has 6 items; one for each of the 4d hyperplanes.
    rotation_speed: Vec<f32>,  // 6 items, as with rotation.  Radians/s ?
    opacity: f32,
    specular_intensity: f32,
}

#[derive(Clone, Debug)]
pub struct Shape {
    // todo macro constructor that lets you ommit position, rotation, scale.
    // Shape nodes and rotation are relative to an origin of 0.
    pub mesh: Mesh,
    pub position: Array1<f32>,
    pub scale: f32,
    pub orientation: Array1<f32>,  // Orientation has 6 items; one for each of the 4d hyperplanes.
    pub rotation_speed: Array1<f32>,  // 6 items, as with rotation.  Radians/s ?
    pub opacity: f32,
    pub specular_intensity: f32,
}

impl Shape {
    pub fn new(mesh: Mesh, position: Array1<f32>, orientation: Array1<f32>,
               rotation_speed: Array1<f32>, opacity: f32) -> Shape {

        Shape{ mesh, position, scale: 1., orientation, rotation_speed,
            opacity, specular_intensity: 1. }
    }
    
    pub fn to_bg(&self) -> ShapeBg {
        ShapeBg {
            mesh: self.mesh.to_bg(),
            position: self.position.to_vec(),
            scale: self.scale,
            orientation: self.orientation.to_vec(),
            rotation_speed: self.rotation_speed.to_vec(),
            opacity: self.opacity,
            specular_intensity: self.specular_intensity,
        }
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
    pub fourd_proj_dist: f32,
}

impl Camera {
    pub fn view_size(&self, far: bool) -> (f32, f32){
        // Calculate the projected window width and height, using basic trig.
        let dist = if far { self.far } else { self.near };

        let width = 2. * dist * (self.fov * self.aspect / 2.).tan();
        let height = 2. * dist * (self.fov / 2.).tan();
        (width, height)
    }

    pub fn to_bg(&self) -> CameraBg {
        CameraBg {
            position: self.position.to_vec(),
            θ: self.θ.to_vec(),
            fov: self.fov,
            aspect: self.aspect,
            aspect_4: self.aspect_4,
            near: self.near,
            far: self.far,
            fourd_proj_dist: self.fourd_proj_dist
        }
    }
}

#[derive(Clone, Debug, Serialize)]
//#[wasm_bindgen]
pub struct CameraBg {
    // See note on MeshBg.
    position: Vec<f32>,
    θ: Vec<f32>,
    fov: f32,
    aspect: f32,
    aspect_4: f32,
    near: f32,
    far: f32,
    fourd_proj_dist: f32,
}

#[derive(Clone, Debug, Serialize)]
//#[wasm_bindgen]
pub enum CameraType {
    Single,  // No camera changes; rotate the shape instead
    // Move foward, back, left, right, and look around. No roll look.  Not sure
    // which 4d rotations/movement to allow or block.
    FPS,
    Free, // No restriction on movement
}

#[derive(Clone, Debug)]
pub struct LightSource {
    // A point light source
    pub position: [f32; 4],
    pub color: [f32; 4],
    pub intensity: f32,
}

impl LightSource {
    pub fn to_bg(&self) -> LightSourceBg {
        LightSourceBg {
            position: self.position.to_vec(),
            intensity: self.intensity,
            color: self.color.to_vec(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
//#[wasm_bindgen]
pub struct LightSourceBg {
    position: Vec<f32>,
    intensity: f32,
    color: Vec<f32>,
}

#[derive(Clone, Debug)]
pub struct Lighting {
    pub ambient_intensity: f32,
    pub diffuse_intensity: f32,
    pub ambient_color: [f32; 4],
    pub diffuse_color: [f32; 4],
    // Direction doesn't have to be normalized; we do that in the shader.
    pub diffuse_direction: [f32; 4],
    pub sources: Vec<LightSource>,
}

impl Lighting {
    pub fn to_bg(&self) -> LightingBg {
        LightingBg {
            ambient_intensity: self.ambient_intensity,
            diffuse_intensity: self.diffuse_intensity,
            ambient_color: self.ambient_color.to_vec(),
            diffuse_color: self.diffuse_color.to_vec(),
            diffuse_direction: self.diffuse_direction.to_vec(),
            sources: self.sources.iter().map(|source| source.to_bg()).collect(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
//#[wasm_bindgen]
pub struct LightingBg {
    ambient_intensity: f32,
    diffuse_intensity: f32,
    ambient_color: Vec<f32>,
    diffuse_color: Vec<f32>,
    // Direction doesn't have to be normalized; we do that in the shader.
    diffuse_direction: Vec<f32>,
    sources: Vec<LightSourceBg>,
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub shapes: HashMap<u32, Shape>,
    pub cam: Camera,
    pub cam_type: CameraType,
    pub lighting: Lighting,
    pub color_max: f32, // distance thresh for max 4d-color indicator.
    pub sensitivities: (f32, f32, f32),  // move, rotate, zoom
}

impl Scene {
    pub fn to_bg(&self) -> SceneBg {
        let mut shapes = HashMap::new();
        for (id, shape) in &self.shapes {
            shapes.insert(*id, shape.to_bg());
        }

        SceneBg {
            shapes,
            cam: self.cam.to_bg(),
            cam_type: self.cam_type.clone(),
            lighting: self.lighting.to_bg(),
            color_max: self.color_max,
            sensitivities: vec![self.sensitivities.0, self.sensitivities.1,
                                  self.sensitivities.2],
        }
    }
}

#[derive(Clone, Debug, Serialize)]
//#[wasm_bindgen]
pub struct SceneBg {
    // We serialize this, it doesn't need to be bindben-capable directly,
    // atm. Bindgen doesn't support public fields, requiring gettets/setters,
    // driving this decision.
    pub shapes: HashMap<u32, ShapeBg>,
    pub cam: CameraBg,
    pub cam_type: CameraType,
    pub lighting: LightingBg,
    pub color_max: f32, // distance thresh for max 4d-color indicator.
    pub sensitivities: Vec<f32>,  // move, rotate, zoom
}

