use std::collections::HashMap;
use std::f32::consts::PI;

use ndarray::prelude::*;

use transforms;
use types::{Vertex, Normal, Shape};

const τ: f32 = 2. * PI;

// We'll define y as vertical, and z as forward/back.  All shapes are given
// four coordinates. Leave

// Nodes are set up here so that 0 is at their center; this is used for scaling,
// rotation, and positioning in the world.

pub fn make_box(lens: (f32, f32, f32),
                position: Array1<f32>, orientation: Array1<f32>,
                rotation_speed: Array1<f32>, opacity: f32) -> Shape {
    // Make a rectangular prism.  Use negative lengths to draw in the opposite
    // direction.

    let coords = [
        // Front
        [-1., -1., -1., 0.],
        [1., -1., -1., 0.],
        [1., 1., -1., 0.],
        [-1., 1., -1., 0.],

        // Back
        [-1., -1., 1., 0.],
        [1., -1., 1., 0.],
        [1., 1., 1., 0.],
        [-1., 1., 1., 0.],
    ];

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] * lens.0 / 2., coord[1] * lens.1 / 2.,
            coord[2] * lens.2 / 2., coord[3] / 2.
        ));
    }

    let faces_vert = vec![  // Vertex indices for each face.
        array![0, 1, 2, 3],  // Front
        array![4, 5, 6, 7],  // Back
        array![3, 2, 6, 7],  // Top
        array![0, 1, 5, 4],  // Bottom
        array![0, 4, 7, 3],  // Left
        array![1, 5, 6, 2],  // Right
    ];

    //  Normals correspond to faces.
    let normals = vec![
        Normal::new(0., 0., -1., 0.),
        Normal::new(0., 0., 1., 0.),
        Normal::new(0., 1., 0., 0.),
        Normal::new(0., -1., 0., 0.),
        Normal::new(-1., 0., 0., 0.),
        Normal::new(1., 0., 0., 0.),
    ];

    Shape::new(vertices, faces_vert, normals, position, orientation, rotation_speed, opacity)
}

pub fn make_rectangular_pyramid(lens: (f32, f32, f32),
                                position: Array1<f32>, orientation: Array1<f32>,
                                rotation_speed: Array1<f32>, opacity: f32) -> Shape {
    let coords = [
        // Base  (Center of this shape is the center of the base square)
        [-1., 0., -1., 0.],
        [1., 0., -1., 0.],
        [1., 0., 1., 0.],
        [-1., 0., 1., 0.],

        // Top
        [0., 1., 0., 0.],
    ];

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] / 2. * lens.0, coord[1] / 2. * lens.1,
            coord[2] / 2. * lens.2, coord[3] / 2.
        ));
    }

    let faces_vert = vec![  // Vertex indices for each face.
                            array![0, 1, 2, 3],  // Base
                            array![0, 1, 4],  // Front
                            array![1, 2, 4],  // Right
                            array![2, 3, 4],  // Back
                            array![3, 0, 4],  // Left
    ];
    
    // Normals correspond to faces.
    // Note that these don't need to be normalized here; the shader will do it.
    let normals = vec![
        Normal::new(0., -1., 0., 0.),
        Normal::new(0., lens.2, -lens.1, 0.),
        Normal::new(-lens.2, lens.1, 0., 0.),
        Normal::new(0., lens.2, lens.1, 0.),
        Normal::new(lens.2, lens.1, 0., 0.),
    ];

    Shape::new(vertices, faces_vert, normals, position, orientation,
               rotation_speed, opacity)
}

pub fn make_house(lens: (f32, f32, f32),
                  position: Array1<f32>,
                  orientation: Array1<f32>,
                  rotation_speed: Array1<f32>, opacity: f32) -> Shape {
    let empty_array = array![0., 0., 0., 0., 0., 0.];

    // We'll modify base in-place, then return it.
    let mut base = make_box(lens, position, orientation, rotation_speed, opacity);

    let roof = make_rectangular_pyramid(
        // Let the roof overhang the base by a little.
        // Make the roof height a portion of the base height.
        (lens.0 * 1.2, lens.1 / 3., lens.2 * 1.2),
        empty_array.clone(), empty_array.clone(), empty_array.clone(), opacity
    );

    // Now that we've made the shapes, recompose them to be one shape.
    // todo make this a separate, (reusable) func?1
    let id_addition = base.vertices.len() as u32;

    for (id, vertex) in &roof.vertices {
        // For the roof, modify the ids to be unique.
        base.vertices.insert(
            id + id_addition,
            // Raise the roof.
            Vertex::new(vertex.position.0, vertex.position.1 + lens.1 / 2.,
                        vertex.position.2, vertex.position.3
            ));
    }

//    for face in &roof.faces_vert {
//        let updated_fv = Vec::new();
//        for vertex in face {
//            updated_fv.push(vertex + id_addition);
//        }
//        base.faces_vert.push(Array1::from_vec(updated_fv));
//    }
    // todo

    for normal in &roof.normals {
        base.normals.push(normal.clone());
    }

    base.make_tris();

    base
}

pub fn make_cube(side_len: f32,  position: Array1<f32>, orientation: Array1<f32>,
                 rotation_speed: Array1<f32>, opacity: f32) -> Shape {
    // Convenience function.
    // We'll still treat the center as the center of the base portion.
    make_box((side_len, side_len, side_len), position, orientation,
             rotation_speed, opacity)
}

pub fn make_5cell(radius: f32,
                   position: Array1<f32>, orientation: Array1<f32>,
                   rotation_speed: Array1<f32>, opacity: f32) -> Shape {
    let coords = [
        [-(2./3. as f32).sqrt(), -1./3., -(2./9. as f32).sqrt(), 0.],  // left base
        [(2./3. as f32).sqrt(), -1./3., -(2./9. as f32).sqrt(), 0.],  // right base
        [0., -1./3., (8./9. as f32).sqrt(), 0.],  // Back base
        [0., 1., 0., 0.],  // Top
        [0., 0., 0., 1.],  // middle
    ];

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] * radius/2., coord[1] * radius/2., coord[2] * radius/2., coord[3] * radius/2.
        ));
    }

    let faces_vert = vec![  // Vertex indices for each face.
        array![0, 1, 2], // Base
        array![0, 1, 3],  // Front
        array![1, 2, 3],  // Right
        array![2, 0, 3],  // Left

        array![4, 0, 1],  // Center front
        array![4, 1, 2],  // Center right
        array![4, 2, 0],  // Center left

        array![4, 0, 3],  // Center left top
        array![4, 1, 3],  // Center right top
        array![4, 2, 3],  // Center back top
    ];

    let normals = vec![  // todo fix this.
        Normal::new(0., 0., 1., 0.),
        Normal::new(0., 0., -1., 0.),
        Normal::new(0., 1., 0., 0.),
        Normal::new(0., -1., 0., 0.),

        Normal::new(-1., 0., 0., 0.),
        Normal::new(1., 0., 0., 0.),
        Normal::new(0., 0., 0., 1.),

        Normal::new(0., 0., 0., -1.),
        Normal::new(0., 0., 0., -1.),
        Normal::new(0., 0., 0., -1.),
    ];

    Shape::new(vertices, faces_vert, normals, position, orientation, rotation_speed, opacity)
}

pub fn make_hyperrect(lens: (f32, f32, f32, f32),
                      position: Array1<f32>, orientation: Array1<f32>,
                      rotation_speed: Array1<f32>, opacity: f32) -> Shape {
    // Make a 4d hypercube.

    let coords = [
        // Front inner
        [-1., -1., -1., -1.],
        [1., -1., -1., -1.],
        [1., 1., -1., -1.],
        [-1., 1., -1., -1.],

        // Back inner
        [-1., -1., 1., -1.],
        [1., -1., 1., -1.],
        [1., 1., 1., -1.],
        [-1., 1., 1., -1.],

        // Front outer
        [-1., -1., -1., 1.],
        [1., -1., -1., 1.],
        [1., 1., -1., 1.],
        [-1., 1., -1., 1.],

        // Back outer
        [-1., -1., 1., 1.],
        [1., -1., 1., 1.],
        [1., 1., 1., 1.],
        [-1., 1., 1., 1.],
    ];

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] * lens.0 / 2., coord[1] * lens.1 / 2.,
            coord[2] * lens.2 / 2., coord[3] * lens.3 / 2.
        ));
    }
    
    let faces_vert = vec![  // Vertex indices for each face.
        array![0, 1, 2, 3],  // Front inner
        array![4, 5, 6, 7],  // Back inner
        array![3, 2, 6, 7],  // Top inner
        array![0, 1, 5, 4],  // Bottom inner
        array![0, 4, 7, 3],  // Left inner
        array![1, 5, 6, 2],  // Right inner

        array![8, 9, 10, 11],  // Front outer
        array![12, 13, 14, 15],  // Back outer
        array![11, 10, 14, 15],  // Top outer
        array![8, 9, 13, 12],  // Bottom outer
        array![8, 12, 15, 11],  // Left outer
        array![9, 13, 14, 10],  // Right outer

        array![8, 9, 1, 0],  // Front bottom
        array![12, 13, 5, 4],  // Back bottom
        array![12, 8, 0, 4],  // Left bottom
        array![9, 13, 5, 1],  // Right bottom

        array![11, 10, 2, 3],  // Front top
        array![15, 14, 6, 7],  // Back top
        array![15, 11, 3, 7],  // Left top
        array![14, 10, 2, 6],  // Right top

        array![11, 8, 0, 3],  // Left forward
        array![15, 12, 4, 7],  // Left back
        array![10, 9, 1, 2],  // Right forward
        array![14, 13, 5, 6],  // Right back
    ];

    // todo QC this; it's a guess.  Attempting to ignore w for this.
    let normals = vec![
        Normal::new(0., 0., -1., 0.),
        Normal::new(0., 0., 1., 0.),
        Normal::new(0., 1., 0., 0.),
        Normal::new(0., -1., 0., 0.),
        Normal::new(-1., 0., 0., 0.),
        Normal::new(1., 0., 0., 0.),

        Normal::new(0., 0., -1., 0.),
        Normal::new(0., 0., 1., 0.),
        Normal::new(0., 1., 0., 0.),
        Normal::new(0., -1., 0., 0.),
        Normal::new(-1., 0., 0., 0.),
        Normal::new(1., 0., 0., 0.),

        Normal::new(0., 0., -1., 0.),
        Normal::new(0., 0., 1., 0.),
        Normal::new(0., 1., 0., 0.),
        Normal::new(0., -1., 0., 0.),
        Normal::new(-1., 0., 0., 0.),
        Normal::new(1., 0., 0., 0.),

        Normal::new(0., 0., -1., 0.),
        Normal::new(0., 0., 1., 0.),
        Normal::new(0., 1., 0., 0.),
        Normal::new(0., -1., 0., 0.),
        Normal::new(-1., 0., 0., 0.),
        Normal::new(1., 0., 0., 0.),

    ];

    Shape::new(vertices, faces_vert, normals, position, orientation, rotation_speed, opacity)
}

pub fn make_hypercube(side_len: f32,
                      position: Array1<f32>, orientation: Array1<f32>,
                      rotation_speed: Array1<f32>, opacity: f32) -> Shape {
    // Convenience function.
    make_hyperrect((side_len, side_len, side_len, side_len),
                   position, orientation, rotation_speed, opacity)
}


pub fn make_terrain(dims: (f32, f32), res: u32,
                    height_map: Array2<f32>, spissitude_map: Array2<f32>,
                    position: Array1<f32>, opacity: f32) -> Shape {
    // Make a triangle-based terrain mesh.  dims is an [x, z] tuple.
    // We could make a 4d terrain too... id a volume of u-mappings... or have
    // w and y mappings for each x/z point...
    // dims refers to the size of the terrain. res is the number of cells
    // dividing our terrain in each direction. Perhaps replace this argument with
    // something more along the traditional def of resolution?

    // todo include some of your code streamlining from make_spherinder;
    // todo better yet: Combine these two with a helper func.

    let mut vertices = HashMap::new();
    let mut normals = Vec::new();

    let mut id = 0;

    let mut active_ind = 0;
    // Faces for this terrain are triangles. Don't try to make square faces;
    // they'd really have creases down a diagonal.
    let mut faces_vert = Vec::new();

    // Instantiate x and like this so the center of the mesh is at the
    // position argument.
    let mut x = -dims.0 / 2.;
    for i in 0..res {  // x
        let mut z = -dims.1 / 2.;
        for j in 0..res {  // z
            let height = height_map[[i as usize, j as usize]];
            let spissitude = spissitude_map[[i as usize, j as usize]];
            // You could change which planes this is over by rearranging
            // these node points.
            vertices.insert(id, Vertex::new(
                x + position[0],
                height + position[1],
                z + position[2],
                spissitude + position[3],
            ));
            z += dims.1 / res as f32;
            id += 1;
        }
        x += dims.0 / res as f32;
    }

    for i in 0..res - 1 {
        for j in 0..res - 1 {
            // The order we build these triangles and normals is subject to trial+error.
            // two face triangles per grid square. There are two ways to split
            // up the squares into triangles; picking one arbitrarily.
            faces_vert.push(
                array![  // shows front right
                    active_ind + j,  // back left
                    active_ind + j + 1,  // back right
                    active_ind + j + res + 1  // front left
                ]
            );

            let line1 = vertices[&(active_ind + j + 1)].subtract(&vertices[&(active_ind + j)]);
            let line2 = vertices[&(active_ind + j + res + 1)].subtract(&vertices[&(active_ind + j)]);

            // Note: This isn't normalized; we handle that in the shader, for now.
            normals.push(line1.cross(&line2));

            faces_vert.push(
                array![  // shows front left  not j + res, not j
                    active_ind + j,
                    active_ind + j + res,  // front right
                    active_ind + j + res + 1  // front left
                ]
            );
            let line1 = vertices[&(active_ind + j)].subtract(&vertices[&(active_ind + j + res)]);
            let line2 = vertices[&(active_ind + j + res + 1)].subtract(&vertices[&(active_ind + j)]);
            normals.push(line1.cross(&line2));
        }
        active_ind += res;
    }

    return Shape::new(vertices, faces_vert, normals, position,
        array![0., 0., 0., 0., 0., 0.], array![0., 0., 0., 0., 0., 0.], opacity)
}

pub fn make_hypergrid(dims: (f32, f32, f32), res: u32,
                                    spissitude_map: Array3<f32>,
                                    position: Array1<f32>, opacity: f32) -> HashMap<u32, Shape> {
    // Position is the center.
    // todo incorporate position.
    let mut result = HashMap::new();

    let mut x = -dims.0 / 2.;
    for i in 0..res {  // x
        let mut y = -dims.1 / 2.;
        for j in 0..res {  // y
            let mut z = -dims.2 / 2.;
            for k in 0..res {  // z
                result.insert(
                    res.pow(2) * i + res * j + k,
                    make_cube(0.5, array![x, y, z, spissitude_map[[i as usize, j as usize, k as usize]]],
                              array![0., 0., 0., 0., 0., 0.], array![0., 0., 0., 0., 0., 0.], opacity)
                );
                z += dims.2 / res as f32
            }
            y += dims.1 / res as f32
        }
        x += dims.0 / res as f32
    }
    return result
}

pub fn make_arrow(lens: (f32, f32), res: u32,
                  position: Array1<f32>, orientation: Array1<f32>,
                  rotation_speed: Array1<f32>, opacity: f32) -> Shape {

    let mut body = make_sphereinder(lens, res, position.clone(),
                                      orientation.clone(), rotation_speed.clone(), opacity);
    let point = make_5cell(lens.1 * 4., Array::zeros(4), Array::zeros(6),
                           Array::zeros(6), 0.);
    // todo combine code here with make_house; eg in a helper func.

        // Now that we've made the shapes, recompose them to be one shape.
    // todo make this a separate, (reusable) func?1
    let id_addition = body.vertices.len() as u32;

    // todo assume long axis is w for now.
    for (id, vertex) in &point.vertices {
        // For the roof, modify the ids to be unique.
        body.vertices.insert(
            id + id_addition,
            Vertex::new(vertex.position.0, vertex.position.1,
                        vertex.position.2, vertex.position.3 + lens.0
            ));
    }

    for face in &point.faces_vert {
        body.faces_vert.push(face + id_addition);
    }

    for normal in &point.normals {
        body.normals.push(normal.clone());
    }


    body.make_tris();

    body
}

pub fn make_sphereinder(lens: (f32, f32), res: u32,
                         position: Array1<f32>,
                         orientation: Array1<f32>, rotation_speed: Array1<f32>, opacity: f32) -> Shape {
    // This is a 4d cylinder analog that extends spheres along a line in the direction
    // not used by the spheres.

    // Each sphere has res longitude vertices, and res/2 latitude vertices.

    // We iterate over longitude twice as much as latitude, so the former must
    // divide by 2.
    assert_eq!(res % 2, 0);

    // uses a crude 'UV' sphere, with uneven face sizes.  Simple algorithm, but
    // not as smooth as other methods.  The code is similar to that used in
    // the terrain mesh.

    // num vertices per sphere.  + 2 for top and bottom.
    let svc = res * (res / 2) + 2;

    // todo tops and bottoms only need one vertex each, not a full set.
    let mut vertices = HashMap::new();
    let mut id = 0;

    // We build vertices and vaces for both spheres in one pass.
    for i in 0..res {
        // ISO standard definitions of θ and φ. The reverse is common too.
        let φ = τ * (i as f32 / res as f32);  // longitude, 0 to τ
        for j in 0..res / 2 {
            let θ = τ * (j as f32 / res as f32);  // latitude, 0 to τ/2
            // These could correlate to diff combos of x/y/z/w.
            let a = lens.1 * θ.sin() * φ.cos();
            let b = lens.1 * θ.sin() * φ.sin();
            let c = lens.1 * θ.cos();

            vertices.insert(id, Vertex::new(a, b, c, 0.));
            vertices.insert(svc + id, Vertex::new(a, b, c, lens.0));

            id += 1;
        }
    }
    // Add top and bottom vertices.
    vertices.insert(id, Vertex::new(0., lens.1, 0., 0.));
    vertices.insert(id + svc, Vertex::new(0., lens.1, 0., 0.));
    id += 1;
    vertices.insert(id, Vertex::new(0., -lens.1, 0., 0.));
    vertices.insert(id + svc, Vertex::new(0., -lens.1, 0., 0.));

    let mut faces_vert = Vec::new();
    let mut lon_i = 0;
    let mut normals = Vec::new();

    // These are four-sided faces; let Shape.make_tris divide them.  There are
    // res-1 lon faces, and res/2 - 1 lat faces, plus faces 2x res/2 - 1 connecting to the
    // top and bottom. (Not including the faces connecting the two spheres).
    for i in 0..res {
        let mut lon_adjuster = 0;
        if i == res - 1 {
            // Num lat vertices * num lon verts between thsi and origin,
            // - the original lon adjustor of res/2.
            lon_adjuster = res/2 * (res - 1) - res/2;
        }

        for j in 0..res/2 - 1 {
            // We increment lat with j. We increment lon with res/2.
            // When at the last longitude, we wrap to the beginning.  This factor
            // is required for the lon adjusted points.

            for sp_selector in &[0, svc] {
                faces_vert.push(
                    array![
                        sp_selector + lon_i + j,  // current point
                        sp_selector + lon_i + j + 1,  // up one lat
                        sp_selector + lon_i + j + res/2 + 1 - lon_adjuster,  // up one lat and one lon
                        sp_selector + lon_i + j + res/2 - lon_adjuster,  // up one lon
                    ]
                );
                // We're ignoring w in normals, for now.
                let line1 = vertices[&(sp_selector + lon_i + j)].subtract(
                    &vertices[&(sp_selector + lon_i + j + res/2 - lon_adjuster)]);
                let line2 = vertices[&(sp_selector + lon_i + j + res/2 + 1 - lon_adjuster)].subtract(
                    &vertices[&(sp_selector + lon_i + j)]);
                normals.push(line1.cross(&line2));
            }

            // Add caps if at the beginning or end or lat iteration.
            if j == 0 {  // Bottom cap
                for sp_selector in &[0, svc] {
                    faces_vert.push(  // origin sphere
                        array![
                            sp_selector + lon_i + j,  // current point
                            sp_selector + lon_i + j + res/2 - lon_adjuster,  // up one lon
                            sp_selector + svc - 1  // Bottom point
                        ]
                    );
                    let line1 = vertices[&(sp_selector + lon_i + j)].subtract(
                        &vertices[&(sp_selector + lon_i + j + res/2 - lon_adjuster)]);
                    let line2 = vertices[&(sp_selector + svc - 1)].subtract(
                        &vertices[&(sp_selector + lon_i + j)]);
                    normals.push(line1.cross(&line2));
                }
            } else if j == res/2 - 2{  // Top cap

            }

            faces_vert.push(  // origin sphere to end sphere along lat
                array![
                    lon_i + j,
                    lon_i + j + 1,
                    svc + lon_i + j,
                    svc + lon_i + j + 1,
                ]
            );
            faces_vert.push(  // origin sphere to end sphere along lon
                array![
                    lon_i + j + res/2 + 1 - lon_adjuster,
                    lon_i + j + res/2 - lon_adjuster,
                    svc + lon_i + j + res/2 + 1 - lon_adjuster,
                    svc + lon_i + j + res/2 - lon_adjuster,
                ]
            );

//            faces_vert.push(  // origin sphere to end sphere part 3  ??
//                array![
//                    lon_i + j,
//                    lon_i + res/2 - lon_adjuster,
//                    svc + lon_i + j,
//                    svc + lon_i + res/2 - lon_adjuster,
//                ]
//            );
//            faces_vert.push(  // origin sphere to end sphere part 4 ??
//                array![
//                    lon_i + j + 1,
//                    lon_i + j + res/2 + 1 - lon_adjuster,
//                    svc + lon_i + j + 1,
//                    svc + lon_i + j + res/2 + 1 - lon_adjuster,
//                ]
//            );

            // todo I suspect we're missing some faces.


            // todo temp for faces connecting the spheres.
            normals.push(Normal::new(0., 0., 0., 0.));
            normals.push(Normal::new(0., 0., 0., 0.));
//            normals.push(Normal::new(0., 0., 0., 0.));
//            normals.push(Normal::new(0., 0., 0., 0.));

        }

        lon_i += res / 2;
    }

    // Add the faces spanning the space between the two spheres.

    Shape::new(vertices, faces_vert, normals, position, orientation, rotation_speed, opacity)
}


pub fn make_origin(lens: (f32, f32), res: u32,  position: Array1<f32>,
                    orientation: Array1<f32>, rotation_speed: Array1<f32>, opacity: f32) -> Shape {

    let mut w = make_arrow(lens, res, position, orientation, rotation_speed, opacity);
    let x = make_arrow(lens, res, Array::zeros(4), Array::zeros(6), Array::zeros(6), 0.);
    let y = make_arrow(lens, res, Array::zeros(4), Array::zeros(6), Array::zeros(6), 0.);
    let z = make_arrow(lens, res, Array::zeros(4), Array::zeros(6), Array::zeros(6), 0.);

    let mut param_set = vec![
        (w.vertices.len() as u32, array![0., 0., 0., τ/4., 0., 0.], x),
        (2 * w.vertices.len() as u32, array![0., 0., 0., 0., τ/4., 0.], y),
        (3 * w.vertices.len() as u32, array![0., 0., 0., 0., 0., τ/4.], z),
    ];

    for (id_addition, θ, shape) in &param_set {
        let R = transforms::make_rotator4(&θ);
        for (id, vertex) in &shape.vertices {
            // For the roof, modify the ids to be unique.
            let rotated_vert = transforms::dot_mv4(R, [vertex.position.0,
                vertex.position.1, vertex.position.2, vertex.position.3]);
            w.vertices.insert(
                id + id_addition,
                Vertex::new(rotated_vert[0], rotated_vert[1], rotated_vert[2], rotated_vert[3]));

        }

        for face in &shape.faces_vert {
            w.faces_vert.push(face + *id_addition);
        }

        for normal in &shape.normals {  // todo rotate normals!
            w.normals.push(normal.clone());
        }

    }

    w.make_tris();

    w
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube() {

    }
}