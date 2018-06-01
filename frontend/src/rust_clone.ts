// Code in this file was intended to be in Rust/WASM. Here since I've given up
// on getting ASM working for now.

import {Shape, Edge, Face, Node2, Camera} from './interfaces'

export function make_box(x_len: number, y_len: number, z_len: number,
                position: number[], scale: number, orientation: number[],
                rotation_speed: number[]): Shape {
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

    let nodes = new Map()

    for (let id=0; id < coords.length; id++) {
        let coord = coords[id]
        nodes.set(id, new Node2([coord[0] * x_len, coord[1] * y_len, coord[2] * z_len, coord[3]]))
    }

    let edges = [
        // Front
        new Edge(0, 1),
        new Edge(1, 2),
        new Edge(2, 3),
        new Edge(3, 0),

        // Back
        new Edge(4, 5),
        new Edge(5, 6),
        new Edge(6, 7),
        new Edge(7, 4),

        // Bridger
        new Edge(0, 4),
        new Edge(1, 5),
        new Edge(2, 6),
        new Edge(3, 7)
    ]

    let faces = [
        // Front
        new Face([edges[0], edges[1], edges[2], edges[3]]),
        // Back
        new Face([edges[4], edges[5], edges[6], edges[7]]),
        // Top
        new Face([edges[2], edges[10], edges[6], edges[11]]),
        // Bottom
        new Face([edges[0], edges[9], edges[4], edges[8]]),
        // Left
        new Face([edges[3], edges[8], edges[7], edges[11]]),
        // Right
        new Face([edges[1], edges[9], edges[5], edges[10]]),
    ]

    return new Shape(nodes, edges, faces, position, scale, orientation, rotation_speed)
}

export function make_rotator_4d(θ: number[]) : number[] {
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // 4d rotation example: http://kennycason.com/posts/2009-01-08-graph4d-rotation4d-project-to-2d.html
    // http://eusebeia.dyndns.org/4d/vis/10-rot-1

    // We rotation around each of six planes; the combinations of the 4
    // dimensions.

    // cache trig computations
    let cos_xy = Math.cos(θ[0])
    let sin_xy = Math.sin(θ[0])
    let cos_yz = Math.cos(θ[1])
    let sin_yz = Math.sin(θ[1])
    let cos_xz = Math.cos(θ[2])
    let sin_xz = Math.sin(θ[2])
    let cos_xu = Math.cos(θ[3])
    let sin_xu = Math.sin(θ[3])
    let cos_yu = Math.cos(θ[4])
    let sin_yu = Math.sin(θ[4])
    let cos_zu = Math.cos(θ[5])
    let sin_zu = Math.sin(θ[5])

    // Potentially there exist 4 hyperrotations as well? ie combinations of
    // 3 axes ?  xyz  yzu  zux  uxy

    // Rotations around the xy, yz, and xz planes should appear normal.
    let R_xy = [
        [cos_xy, sin_xy, 0., 0., 0.],
        [-sin_xy, cos_xy, 0., 0., 0.],
        [0., 0., 1., 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ]

    let R_yz = [
        [1., 0., 0., 0., 0.],
        [0., cos_yz, sin_yz, 0., 0.],
        [0., -sin_yz, cos_yz, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ]

    let R_xz = [
        [cos_xz, 0., -sin_xz, 0., 0.],
        [0., 1., 0., 0., 0.],
        [sin_xz, 0., cos_xz, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ]

    // Rotations involving u, the fourth dimension, should distort 3d objects.
    let R_xu = [
        [cos_xu, 0., 0., sin_xu, 0.],
        [0., 1., 0., 0., 0.],
        [0., 0., 1., 0., 0.],
        [-sin_xu, 0., 0., cos_xu, 0.],
        [0., 0., 0., 0., 1.]
    ]

    let R_yu = [
        [1., 0., 0., 0., 0.],
        [0., cos_yu, 0., -sin_yu, 0.],
        [0., 0., 1., 0., 0.],
        [0., sin_yu, 0., cos_yu, 0.],
        [0., 0., 0., 0., 1.]
    ]

    let R_zu = [
        [1., 0., 0., 0., 0.],
        [0., 1., 0., 0., 0.],
        [0., 0., cos_zu, -sin_zu, 0.],
        [0., 0., sin_zu, cos_zu, 0.],
        [0., 0., 0., 0., 1.]
    ]

    // Combine the rotations.
    let R_1 = dot(R_xy, dot(R_yz, R_xz))
    let R_2 = dot(R_xu, dot(R_yu, R_zu))
    return dot(R_1, R_2)
}

export function make_rotator_3d(θ: number[]): number[] {
    // Compute a 3-dimensional rotation matrix.
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // We return 5x5 matrices for compatibility with other transforms, and to
    // reduce repetition between 4d and 3d vectors.

    // Note that we might accept a rotation vector of len 6, but only the
    // first 3 values will be used.

    // cache trig computations
    let cos_x = Math.cos(θ[0])
    let sin_x = Math.sin(θ[0])
    let cos_y = Math.cos(θ[1])
    let sin_y = Math.sin(θ[1])
    let cos_z = Math.cos(θ[2])
    let sin_z = Math.sin(θ[2])

    // R matrices rotate a vector around a single axis.
    let R_x = [
        [1., 0., 0., 0., 0.],
        [0., cos_x, -sin_x, 0., 0.],
        [0., sin_x, cos_x, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ];

    let R_y = [
        [cos_y, 0., sin_y, 0., 0.],
        [0., 1., 0., 0., 0.],
        [-sin_y, 0., cos_y, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ]

    let R_z = [
        [cos_z, -sin_z, 0., 0., 0.],
        [sin_z, cos_z, 0., 0., 0.],
        [0., 0., 1., 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ]

    // Combine the three rotations.
    return dot(R_x, dot(R_y, R_z))
}

export function make_translator(cam_position: number[]): number[][] {
    // Return a translation matrix; the pt must have 1 appended to its end.
    // We do this augmentation so we can add a constant term.  Scale and
    // rotation matrices may have this as well for matrix compatibility.
    return [
        [1., 0., 0., 0., cam_position[0]],
        [0., 1., 0., 0., cam_position[1]],
        [0., 0., 1., 0., cam_position[2]],
        [0., 0., 0., 1., cam_position[3]],
        [0., 0., 0., 0., 1.]
    ]
}

export function make_scaler(scale: number[]): number[][] {
    // Return a scale matrix; the pt must have 1 appended to its end.
    return [
        [scale[0], 0., 0., 0., 0.],
        [0., scale[1], 0., 0., 0.],
        [0., 0., scale[2], 0., 0.],
        [0., 0., 0., scale[3], 0.],
        [0., 0., 0., 0., 1.]
    ]
}

export function make_projector(cam: Camera): number[][] {
    // Create the projection matrix, used to transform translated and
    // rotated points.

    // Let's compile the different versions you've seen:
    // 1: http://learnwebgl.brown37.net/08_projections/projections_perspective.html
    // 2: https://en.wikipedia.org/wiki/3D_projection
    // 3: https://solarianprogrammer.com/2013/05/22/opengl-101-matrices-projection-view-model/
    // 4: https://www.scratchapixel.com/lessons/3d-basic-rendering/perspective-and-orthographic-
    // projection-matrix/building-basic-perspective-projection-matrix
    // 5: https://github.com/brendanzab/cgmath/blob/master/src/projection.rs
    // 6: https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/WebGL_model_view_projection
    // 7: http://www.songho.ca/opengl/gl_projectionmatrix.html

    // 7's on point with my calcs, although stated in terms of right/top.

    let y_scale = 1. / Math.tan(cam.fov / 2.)
    let x_scale = y_scale / cam.aspect
    let u_scale = y_scale / cam.aspect_4  // depth for 4d

    // We are defining z as the axis that determines how x and y points are
    // scaled, for both 4d and 3d projections. U points don't play a factor
    // in our final result; their data is only included during rotations;
    // This function transforms them, but that ultimately is not projected to
    // 2d screens.

    // Insight: z (or u, depending on which convention we settle on) is used
    // for two things: Determining how we should scale x and y (The vars that


    // I've derived these matrices myself; none of the ones described in the
    // above links seem to produce a unit cube for easy clipping.
    // They map the frustum to a "unit" [hyper]cube; actually ranging from -1 to +1,
    // along each axis.
    // Note: Unlike x, y, (and u?) z (doesn't map in a linear way; it goes
    // as a decaying exponential from -1 to +1.

    return [
            [x_scale, 0., 0., 0., 0.],
            [0., y_scale, 0., 0., 0.],
            [0., 0., (cam.far + cam.near) / (cam.far - cam.near),
                (-2. * cam.far * cam.near) / (cam.far - cam.near),  0.],
            // u_scale is, ultimately, not really used.
            [0., 0., 0., u_scale, 0.],
            // This row allows us to divide by z after taking the dot product,
            // as part of our scaling operation.
            [0., 0., 1., 0., 1.],
        ]
}

function dot(a: any, b: any) {
	return a.map(function(x: any, i: any) {
		return a[i] * b[i];
	}).reduce(function(m: any, n: any) { return m + n; });
}


export function position_shape(shape: Shape): Map<number, number[]> {
    // Position a shape's nodes in 3 or 4d space, based on its position
    // and rotation parameters.

    let is_4d = Math.abs(shape.rotation_speed[3]) > 0. || Math.abs(shape.rotation_speed[4]) > 0. ||
        Math.abs(shape.rotation_speed[5]) > 0. || Math.abs(shape.orientation[3]) > 0. ||
        Math.abs(shape.orientation[4]) > 0. || Math.abs(shape.orientation[5]) > 0.

    // T must be done last, since we scale and rotate with respect to the orgin,
    // defined in the shape's initial nodes. S may be applied at any point.
    let R = is_4d ? make_rotator_4d(shape.orientation) : make_rotator_3d(shape.orientation)

    let S = make_scaler([shape.scale, shape.scale, shape.scale, shape.scale])
    let T = make_translator(shape.position)

    let positioned_nodes = new Map()
    for (let id=0; id < shape.nodes.size; id++) {
        let node = shape.nodes[id]
    // We dot what OpenGL calls the 'Model matrix' with our point. Scale,
    // then rotate, then translate.
        let homogenous = [node.a[0], node.a[1], node.a[2], node.a[3], 1.]
        let new_pt = dot(T, dot(R, S))
        positioned_nodes.set(id, new_pt)
    }

    return positioned_nodes
}