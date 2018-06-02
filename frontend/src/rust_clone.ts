// Code in this file was intended to be in Rust/WASM. Here since I've given up
// on getting ASM working for now.
import {Shape, Edge, Face, Node2, Camera, Vec5, Array5} from './interfaces'

// toddo: Not specifying njnew Vec5 return types since TS doesn't like it.

export function make_box(lens: [number, number, number],
                         position: Vec5, scale: number, orientation: number[],
                         rotation_speed: number[]): Shape {
    // Make a rectangular prism.  Use negative lengths to draw in the opposite
    // direction.

    const coords = [
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
        nodes.set(id, new Node2(new Vec5([coord[0] * lens[0], coord[1] * lens[1],
            coord[2] * lens[2], coord[3]])))
    }

    const edges = [
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

    let faces_vert = [  // Vertex indices for each face.
        [0, 1, 2, 3],  // Front
        [4, 5, 6, 7],  // Back
        [3, 2, 6, 7],  // Top
        [0, 1, 5, 4],  // Bottom
        [0, 4, 7, 3],  // Left
        [1, 5, 6, 2],  // Right

    ]

    return new Shape(nodes, edges, faces, faces_vert, position,
        scale, orientation, rotation_speed)
}

export function make_origin(len: number, position: Vec5, scale: number,
                            orientation: number[], rotation_speed: number[]): Shape {
    // A 4-dimensional cross, for marking the origin.
    const coords = [
        [-1., 0., 0., 0.],
        [1., 0., 0., 0.],
        [0., -1., 0., 0.],
        [0., 1., 0., 0.],

        [0., 0., -1., 0.],
        [0., 0., 1., 0.],
        [0., 0., 0., -1.],
        [0., 0., 0., 1.],
    ];

    let nodes = new Map()
    for (let id=0; id < coords.length; id++) {
        let coord = coords[id]
        // todo should have better vector arithmetic
        nodes.set(id, new Node2(new Vec5([coord[0] * len, coord[1] * len,
            coord[2] * len, coord[3] * len])))
    }
    const edges = [
        new Edge(0, 1),
        new Edge(2, 3),
        new Edge(4, 5),
        new Edge(6, 7),
    ];

    return new Shape(nodes, edges, [], [], position as any, scale,
        orientation, rotation_speed)
}

export function make_rotator_4d(θ: number[]): Array5 {
    // Rotation matrix information: https://en.wikipedia.org/wiki/Rotation_matrix
    // 4d rotation example: http://kennycason.com/posts/2009-01-08-graph4d-rotation4d-project-to-2d.html
    // http://eusebeia.dyndns.org/4d/vis/10-rot-1

    // We rotation around each of six planes; the combinations of the 4
    // dimensions.

    // cache trig computations
    const cos_xy = Math.cos(θ[0])
    const sin_xy = Math.sin(θ[0])
    const cos_yz = Math.cos(θ[1])
    const sin_yz = Math.sin(θ[1])
    const cos_xz = Math.cos(θ[2])
    const sin_xz = Math.sin(θ[2])
    const cos_xu = Math.cos(θ[3])
    const sin_xu = Math.sin(θ[3])
    const cos_yu = Math.cos(θ[4])
    const sin_yu = Math.sin(θ[4])
    const cos_zu = Math.cos(θ[5])
    const sin_zu = Math.sin(θ[5])

    // Potentially there exist 4 hyperrotations as well? ie combinations of
    // 3 axes ?  xyz  yzu  zux  uxy

    // Rotations around the xy, yz, and xz planes should appear normal.
    let R_xy = new Array5([
        [cos_xy, sin_xy, 0., 0., 0.],
        [-sin_xy, cos_xy, 0., 0., 0.],
        [0., 0., 1., 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ])

    const R_yz = new Array5([
        [1., 0., 0., 0., 0.],
        [0., cos_yz, sin_yz, 0., 0.],
        [0., -sin_yz, cos_yz, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ])

    const R_xz = new Array5([
        [cos_xz, 0., -sin_xz, 0., 0.],
        [0., 1., 0., 0., 0.],
        [sin_xz, 0., cos_xz, 0., 0.],
        [0., 0., 0., 1., 0.],
        [0., 0., 0., 0., 1.]
    ])

    // Rotations involving u, the fourth dimension, should distort 3d objects.
    const R_xu = new Array5([
        [cos_xu, 0., 0., sin_xu, 0.],
        [0., 1., 0., 0., 0.],
        [0., 0., 1., 0., 0.],
        [-sin_xu, 0., 0., cos_xu, 0.],
        [0., 0., 0., 0., 1.]
    ])

    const R_yu = new Array5([
        [1., 0., 0., 0., 0.],
        [0., cos_yu, 0., -sin_yu, 0.],
        [0., 0., 1., 0., 0.],
        [0., sin_yu, 0., cos_yu, 0.],
        [0., 0., 0., 0., 1.]
    ])

    const R_zu = new Array5([
        [1., 0., 0., 0., 0.],
        [0., 1., 0., 0., 0.],
        [0., 0., cos_zu, -sin_zu, 0.],
        [0., 0., sin_zu, cos_zu, 0.],
        [0., 0., 0., 0., 1.]
    ])

    // Combine the rotations.
    const R_1 = R_xy.dotM(R_yz.dotM(R_xz))
    const R_2 = R_xu.dotM(R_yu.dotM(R_zu))
    return R_1.dotM(R_2)
}

export function make_translator(position: Vec5): Array5 {
    // Return a translation matrix; the pt must have 1 appended to its end.
    // We do this augmentation so we can add a constant term.  Scale and
    // rotation matrices may have this as well for matrix compatibility.
    return new Array5([
        [1., 0., 0., 0., position.vals[0]],
        [0., 1., 0., 0., position.vals[1]],
        [0., 0., 1., 0., position.vals[2]],
        [0., 0., 0., 1., position.vals[3]],
        [0., 0., 0., 0., 1.]
    ])
}

export function make_scaler(scale: Vec5): Array5 {
    // Return a scale matrix; the pt must have 1 appended to its end.
    return new Array5([
        [scale.vals[0], 0., 0., 0., 0.],
        [0., scale.vals[1], 0., 0., 0.],
        [0., 0., scale.vals[2], 0., 0.],
        [0., 0., 0., scale.vals[3], 0.],
        [0., 0., 0., 0., 1.]
    ])
}

export function make_projector(cam: Camera): Array5 {
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

    return new Array5([
        [x_scale, 0., 0., 0., 0.],
        [0., y_scale, 0., 0., 0.],
        [0., 0., (cam.far + cam.near) / (cam.far - cam.near),
            (-2. * cam.far * cam.near) / (cam.far - cam.near),  0.],
        // u_scale is, ultimately, not really used.
        [0., 0., 0., u_scale, 0.],
        // This row allows us to divide by z after taking the dot product,
        // as part of our scaling operation.
        [0., 0., 1., 0., 1.],
    ])
}

export function position_shape(shape: Shape): Map<number, Vec5> {
    // Position a shape's nodes in 3 or 4d space, based on its position
    // and rotation parameters.

    // T must be done last, since we scale and rotate with respect to the orgin,
    // defined in the shape's initial nodes. S may be applied at any point.
    const R = make_rotator_4d(shape.orientation)
    const S = make_scaler(new Vec5([shape.scale, shape.scale, shape.scale, shape.scale]))
    const T = make_translator(shape.position)

    let positioned_nodes = new Map()
    for (let id=0; id < shape.nodes.size; id++) {

        let node: any = shape.nodes.get(id)
        // We dot what OpenGL calls the 'Model matrix' with our point. Scale,
        // then rotate, then translate.
        const homogenous = new Vec5([node.a.vals[0], node.a.vals[1],
            node.a.vals[2], node.a.vals[3], 1.])

        const transform = T.dotM(R.dotM(S))
        const new_pt = transform.dotV(homogenous)
        positioned_nodes.set(id, new_pt)
    }

    return positioned_nodes
}
