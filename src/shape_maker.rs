use std::collections::HashMap;
use std::f32::consts::PI;

use ndarray::prelude::*;

use transforms;
use types::{Vertex, Mesh, Normal, Shape};
use util;

const τ: f32 = 2. * PI;

// We'll define y as vertical, and z as forward/back.  All shapes are given
// four coordinates. Leave

// Nodes are set up here so that 0 is at their center; this is used for scaling,
// rotation, and positioning in the world.

pub fn combine_meshes(mut base: Mesh, meshes: Vec<(Mesh, [f32; 4])>) -> Mesh{
    // The array in the meshes tuple is position offset for that shape.
    let mut id_addition = base.vertices.len() as u32;
    for (mesh, offset) in &meshes {
        for (id, vertex) in &mesh.vertices {
            // For the roof, modify the ids to be unique.
            base.vertices.insert(
                id + id_addition,
                Vertex::new(vertex.position.0 + offset[0], vertex.position.1 + offset[1],
                            vertex.position.2 + offset[2], vertex.position.3 + offset[3]
                )
            );
        }

        for face in &mesh.faces_vert {
            base.faces_vert.push(face + id_addition);
        }

        for normal in &mesh.normals {  // todo rotate normals!
            base.normals.push(normal.clone());
        }

        id_addition += mesh.vertices.len() as u32;
    }

    base.make_tris();
    base
}

pub fn box_(lens: (f32, f32, f32)) -> Mesh {
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

    Mesh::new(vertices, faces_vert, normals)
}

pub fn rect_pyramid(lens: (f32, f32, f32)) -> Mesh {
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

    Mesh::new(vertices, faces_vert, normals)
}

pub fn house(lens: (f32, f32, f32)) -> Mesh {
    // We'll modify base in-place, then return it.
    let base = box_(lens);

    let roof = rect_pyramid(
        // Let the roof overhang the base by a little.
        // Make the roof height a portion of the base height.
        (lens.0 * 1.2, lens.1 / 3., lens.2 * 1.2),
    );

    combine_meshes(base, vec![(roof, [0., lens.1 / 2., 0., 0.])])
}

pub fn cube(side_len: f32) -> Mesh {
    // Convenience function.
    // We'll still treat the center as the center of the base portion.
    box_((side_len, side_len, side_len))
}

pub fn fivecell(radius: f32) -> Mesh {
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

    Mesh::new(vertices, faces_vert, normals)
}

pub fn hyperrect(lens: (f32, f32, f32, f32)) -> Mesh {
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

    Mesh::new(vertices, faces_vert, normals)
}

pub fn make_hypercube(side_len: f32) -> Mesh {
    // Convenience function.
    hyperrect((side_len, side_len, side_len, side_len))
}

fn avg_normals(normals: Vec<Normal>) -> Normal {
    let x = normals.iter().fold(0., |acc, norm| acc + norm.normal.0);
    let y = normals.iter().fold(0., |acc, norm| acc + norm.normal.1);
    let z = normals.iter().fold(0., |acc, norm| acc + norm.normal.2);
    let w = normals.iter().fold(0., |acc, norm| acc + norm.normal.3);

    let len = normals.len() as f32;
    Normal::new(x/len , y/len, z/len, w/len)
}

pub fn terrain(dims: (f32, f32), res: u32,
               height_map: Array2<f32>, spissitude_map: Array2<f32>,
               ) -> Mesh {
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

    for i in 0..res {  // x
        let x = util::value_from_grid(i, res, (0., dims.0));
        for j in 0..res {  // z
            let z = util::value_from_grid(j, res, (0., dims.1));
            let height = height_map[[i as usize, j as usize]];
            let spissitude = spissitude_map[[i as usize, j as usize]];
            // You could change which planes this is over by rearranging
            // these node points.
            vertices.insert(id, Vertex::new(
                x,
                height,
                z,
                spissitude,
            ));
            id += 1;
        }
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
                    active_ind + j + res + 1,  // front left
                    active_ind + j + res
                ]
            );

            let current_ind = active_ind + j;
            let current_vert = &vertices[&(current_ind)];

            // Compute normal as the avg of the norm of all 4 neighboring faces.
            // We are ignoring w, for now.
            let mut edge_pairs = Vec::new();
            // If logic is to prevent index mistakes on edge cases.
            // Start at North; go around clockwise.
            if i != res - 2 && j != res - 2 {  // not at ne corner
                edge_pairs.push((
                    vertices[&(current_ind + 1)].subtract(current_vert),  // n
                    vertices[&(current_ind + res + 1)].subtract(current_vert)  // ne
                ));
                edge_pairs.push((
                    vertices[&(current_ind + res + 1)].subtract(current_vert),  // ne
                    vertices[&(current_ind + res)].subtract(current_vert)  // e
                ));
            }
            if i != res - 2 && j != 0 {  // not at se corner
                edge_pairs.push((
                    vertices[&(current_ind + res)].subtract(current_vert),  // e
                    vertices[&(current_ind + res - 1)].subtract(current_vert)  // se
                ));
                edge_pairs.push((
                    vertices[&(current_ind + res - 1)].subtract(current_vert),  // se
                    vertices[&(current_ind - 1)].subtract(current_vert)  // s
                ));
            }
            if i != 0 && j != 0 {  // not at sw corner
                edge_pairs.push((
                    vertices[&(current_ind - 1)].subtract(current_vert),  // s
                    vertices[&(current_ind - res - 1)].subtract(current_vert)  // sw
                ));
                edge_pairs.push((
                    vertices[&(current_ind - res - 1)].subtract(current_vert),  // sw
                    vertices[&(current_ind - res)].subtract(current_vert)  // w
                ));
            }
             if i != 0 && j != res - 2 {  // not at nw corner
                edge_pairs.push((
                    vertices[&(current_ind - res)].subtract(current_vert),  // w
                    vertices[&(current_ind - res + 1)].subtract(current_vert)  // nw
                ));
                edge_pairs.push((  // nw
                    vertices[&(current_ind - res + 1)].subtract(current_vert),  // nw
                    vertices[&(current_ind + 1)].subtract(current_vert)  // n
                ));
            }

            // Note: This isn't normalized; we handle that in the shader, for now.
            let mut surrounding_norms = Vec::new();
            for (edge0, edge1) in &edge_pairs {
                surrounding_norms.push(edge0.cross(edge1));
            }

            normals.push(avg_normals(surrounding_norms));

        }
        active_ind += res;
    }

    Mesh::new(vertices, faces_vert, normals)
}


//pub fn grid(n_dims: u32, dims: Vec<f32>, res: u32) -> HashMap<u32, Shape> {
//    // An evenly-spaced grid of n-dimensions.
//    let mut result = HashMap::new();
//    if n_dims == 0 {
//        return result
//    }
//
//    for i in 0..res {
//
//    }
//
//    grid(active_dim, dims, res)
//}


pub fn hypergrid(dims: (f32, f32, f32), res: u32,
                 spissitude_map: Array3<f32>) -> HashMap<u32, Shape> {
    // todo you could make this recursive, for a grid of arbitrary dimension.
    let mut result = HashMap::new();

    let mut x = -dims.0 / 2.;
    for i in 0..res {  // x
        let mut y = -dims.1 / 2.;
        for j in 0..res {  // y
            let mut z = -dims.2 / 2.;
            for k in 0..res {  // z
                result.insert(
                    res.pow(2) * i + res * j + k,
                    Shape::new(cube(0.5), array![x, y, z, spissitude_map[[i as usize, j as usize, k as usize]]],
                               array![0., 0., 0., 0., 0., 0.], array![0., 0., 0., 0., 0., 0.], 1.)
                );
                z += dims.2 / res as f32
            }
            y += dims.1 / res as f32
        }
        x += dims.0 / res as f32
    }
    result
}

pub fn grid_4d(dims: (f32, f32, f32, f32), res: u32) -> HashMap<u32, Shape> {
    // Creates and evenly spaced grid in 4 dimensions. We don't take a
    // map like with the 3d grid, since by definition it's evenly spaced; there's
    // no 5th dimension in the program to map to!
    let mut result = HashMap::new();

    let mut u = -dims.3 / 2.;
    let mut id = 0;
    for _ in 0..res {
        let spiss = Array3::ones((res as usize, res as usize, res as usize)) * u;
        let subgrid = hypergrid((dims.0, dims.1, dims.2), res, spiss);

        for (_, shape) in subgrid.into_iter() {
            // We discard the id in the subgrid's HashMap.
            result.insert(id, shape);
            id += 1;
        }

        u += dims.3 / res as f32
    }

    result
}

pub fn arrow(lens: (f32, f32), res: u32) -> Mesh {
    let body = spherinder(lens, res);
    let point = fivecell(lens.1 * 4.);

    combine_meshes(body, vec![(point, [0., 0., 0., lens.0])])
}

//pub fn make_sphere(radius: f32, res: u32) -> Mesh {
//    assert_eq!(res % 2, 0);
//}

pub fn twentyfourcell(radius: f32) -> Mesh {
    // Using this diagram as reference:
    // https://en.wikipedia.org/wiki/24-cell#/media/File:Schlegel_wireframe_24-cell.png
    let coords = [
//        // Permutations of (±1, ±1, 0, 0)
//        // Could do this cleverly with an iteration
        // Outside
        [1., 0., 0., 1.], // 0
        [0., 1., 0., 1.],  // 1
        [0., 0., 1., 1.],  // 2
        [-1., 0., 0., 1.],  // 3
        [0., -1., 0., 1.],  // 4
        [0., 0., -1., 1.],  // 5

        // Middle
        [1., 1., 0., 0.],  // 6
        [1., 0., 1., 0.],  // 7
        [0., 1., 1., 0.],  // 8

        [-1., -1., 0., 0.],  // 9
        [-1., 0., -1., 0.],  // 10
        [0., -1., -1., 0.],  // 11

        [1., -1., 0., 0.],  // 12
        [1., 0., -1., 0.],  // 13
        [0., 1., -1., 0.],  // 14

        [-1., 1., 0., 0.],  // 15
        [-1., 0., 1., 0.],  // 16
        [0., -1., 1., 0.],  // 17

        // Inside
        [1., 0., 0., -1.],  // 18
        [0., 1., 0., -1.],  // 19
        [0., 0., 1., -1.],  // 20
        [-1., 0., 0., -1.],  // 21
        [0., -1., 0., -1.],  // 22
        [0., 0., -1., -1.],  //23

    ];

    let mut vertices = HashMap::new();
    for (id, coord) in coords.iter().enumerate() {
        vertices.insert(id as u32, Vertex::new(
            coord[0] * radius/2., coord[1] * radius/2., coord[2] * radius/2., coord[3] * radius/2.
        ));
    }

    // Each outer and inner face has 3 vertices and 3 edges
    // Inner and outer resemble octahedrons. Middle resembles a cuboctahedron.
    // Middle has 8 3-vert faces.
    // Each vertex is connected to 8 others.
    // Ref drawing in tablet
    // 96 faces total:
    // 8 in inner; 8 in outer; 8 in middle
    // 24 connecting middle squares to outside
    // 24 connecting middle squares to inside
    // 18 connecting middle triangles to outside.

    let faces_vert = vec![
        // Outside to itself right
        array![0, 1, 2],
        array![0, 2, 4],
        array![0, 4, 5],
        array![0, 5, 1],

        // Outside to itself left
        array![3, 1, 2],
        array![3, 2, 4],
        array![3, 4, 5],
        array![3, 5, 1],

        // Inside to itself right
        array![18, 19, 20],
        array![18, 20, 22],
        array![18, 22, 23],
        array![18, 23, 19],

        // Inside to itself left
        array![21, 19, 20],
        array![21, 20, 22],
        array![21, 22, 23],
        array![21, 23, 19],

        // Leaving this commented out chunk to prove a point: There are no square
        // faces! the squares in the 'middle' are just a tool to organize this.
        // The 24-cell has 96 triangular faces. Same principle as with the octahedron.

//        // Middle square faces
//        array![9, 10, 15, 16],  // Left
//        array![6, 7, 12, 13],  // Right
//
//        array![9, 17, 12, 11],  // Bottom
//        array![15, 8, 6, 14],  // Top
//
//        array![10, 14, 13, 11],  // Aft
//        array![8, 7, 17, 16],  // Forward

        // These triangular middle faces each have an edge belonging to a different
        // one of the (non-face) squares above.
        // Bottom tris
        array![11, 13, 12],  // aft right
        array![12, 7, 17],  // fwd right
        array![17, 16, 9],  // fwd lrft
        array![9, 10, 11],  // aft left

        // Top tris
        array![14, 13, 6],  // aft right
        array![6, 7, 8],  // fwd right
        array![8, 16, 15],  // fwd lrft
        array![15, 10, 14],  // aft left

        // Connect each of these tris to a point on the inside and outside; 24 total.
        // Each vertex in the middle is at the intersection of two of the squares.
        // Connect each middle vertex to the two inner and outer vertices corresponding
        // to these squares.

        // Notice the cyclic pattern below.
        // Inner
        array![6, 1, 0],  // top right
        array![7, 0, 2],  // right forward
        array![8, 2, 1 ],  // forward top

        array![9, 4, 3],  // bottom left
        array![10, 3, 5],  // left aft
        array![11, 5, 4],  // aft bottom
        array![12, 4, 0],  // bottom right
        array![13, 0, 5],  // right aft
        array![14, 5, 1],  // aft top
        array![15, 1, 3],  // top left
        array![16, 3, 2],  // left forward
        array![17, 2, 4 ],  // forward  bottom

        // Outer
        array![6, 19, 18],  // top right
        array![7, 18, 20],  // right forward
        array![8, 20, 19 ],  // forward top

        array![9, 22, 21],  // bottom left
        array![10, 21, 23],  // left aft
        array![11, 23, 22],  // aft bottom
        array![12, 22, 18],  // bottom right
        array![13, 18, 23],  // right aft
        array![14, 23, 19],  // aft top
        array![15, 19, 21],  // top left
        array![16, 21, 20],  // left forward
        array![17, 20, 22 ],  // forward  bottom

        // Connect each middle square to its point on the inside and outside
        // octahedrons, with triangular faces.  This forms new octahedrons.
        // We make use of cyclic patterns here to simplify the logic.
        // This uses up 48 faces; 24 connecting to inner, 24 connecting to outer.

        // Left square to inner
        array![9, 10, 3],
        array![10, 15, 3],
        array![15, 16, 3],
        array![16, 9, 3],

        // Left square to outer
        array![9, 10, 21],
        array![10, 15, 21],
        array![15, 16, 21],
        array![16, 9, 21],

        // Right square to inner
        array![6, 7, 0],
        array![7, 12, 0],
        array![12, 13, 0],
        array![13, 6, 0],

        // Right square to outer
        array![6, 7, 18],
        array![7, 12, 18],
        array![12, 13, 18],
        array![13, 6, 18],

        // Bottom square to inner
        array![9, 17, 4],
        array![17, 12, 4],
        array![12, 11, 4],
        array![11, 9, 1],

        // Bottom square to outer
        array![9, 17, 22],
        array![17, 12, 22],
        array![12, 11, 22],
        array![11, 9, 22],

        // Top square to inner
        array![15, 8, 1],
        array![8, 6, 1],
        array![6, 14, 1],
        array![14, 15, 1],

        // Top square to outer
        array![15, 8, 19],
        array![8, 6, 19],
        array![6, 14, 19],
        array![14, 15, 19],

        // Aft square to inner
        array![10, 14, 5],
        array![14, 13, 5],
        array![13, 11, 5],
        array![11, 10, 5],

        // Aft square to outer
        array![10, 14, 23],
        array![14, 13, 23],
        array![13, 11, 23],
        array![11, 10, 23],

        // Forward square to inner
        array![8, 7, 2],
        array![7, 17, 2],
        array![17, 16, 2],
        array![16, 8, 2],

        // Forward square to outer
        array![8, 7, 20],
        array![7, 17, 20],
        array![17, 16, 20],
        array![16, 8, 20],
        // only 78 so far...

    ];

//    let normals = vec![
//        Normal::new(0., 0., 1., 0.),
//    ];
    let normals: Vec<Normal> = faces_vert.iter().map(|face| Normal::new(1., 0., 1., 0.)).collect();

    Mesh::new(vertices, faces_vert, normals)
}

pub fn spherinder(lens: (f32, f32), res: u32) -> Mesh {
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

    // We build vertices and faces for both spheres in one pass.
    for i in 0..res {
        // ISO standard definitions of θ and φ. The reverse is common too.
        let φ = util::value_from_grid(i, res, (0., τ));  // longitude, 0 to τ
        for j in 0..res / 2 {
            let θ = util::value_from_grid(j, res, (0., τ));  // latitude, 0 to τ/2
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
            // Num lat vertices * num lon verts between this and origin,
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

    Mesh::new(vertices, faces_vert, normals)
}


pub fn letter_cube(len: f32, letter: &str) {

}


pub fn origin(lens: (f32, f32), res: u32) -> Mesh {

    let mut w = arrow(lens, res);
    let x = arrow(lens, res);
    let y = arrow(lens, res);
    let z = arrow(lens, res);

    let param_set = vec![
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