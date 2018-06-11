import {Camera, Shape} from "./interfaces";
import * as shapeMaker from "./shapeMaker";

// todo global shapes and cam for now
const τ = 2 * Math.PI

export let shapes = new Map()

export let colorMax = 15  // At this z distance, our blue/red shift fully saturated.
export let currentlyPressedKeys: number[] = []
export const moveSensitivity = .1
export const rotateSensitivity = .017

// todo initBuffers needs to go back in render. Here for now to manage updating
// todo index buffers on scene change.

export function initBuffers(gl: any, shapes_: Map<number, Shape>, skybox_: Shape) {
    // Create a buffer for our shapes' positions and color.
    const sbPositions: any = []
    let sbVertex
    skybox_.faces_vert.map(face => face.map(vertex_i => {
        sbVertex = (skybox_.nodes.get(vertex_i) as any).a
        for (let i=0; i < 4; i++) {
            sbPositions.push(sbVertex[i])

        }
    }))

    // Now pass the list of positions into WebGL to build the
    // shape. We do this by creating a Float32Array from the
    // JavaScript array, then use it to fill the current buffer.
    const skyboxPositBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, skyboxPositBuffer)
    gl.bufferData(gl.ARRAY_BUFFER,
        new Float32Array(sbPositions),
        gl.STATIC_DRAW)

    // todo skybox texture wip
    // look up where the vertex data needs to go.
    // const positionLocation = gl.getAttribLocation(shaderProgram, "a_position");
    // consttexcoordLocation = gl.getAttribLocation(program, "a_texcoords");
    // Create a buffer for texcoords.

    const skyboxTexBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, skyboxTexBuffer)
    // Set Texcoords.
    gl.bufferData(
        gl.ARRAY_BUFFER,
        new Float32Array([
            // left column front
            0, 0,
            0, 1,
            1, 0,
            0, 1,
            1, 1,
            1, 0,

            // top rung front
            0, 0,
            0, 1,
            1, 0,
            0, 1,
            1, 1,
            1, 0,
        ]),
        gl.STATIC_DRAW)

    // Build the element array buffer; this specifies the indices
    // into the vertex arrays for each face's vertices.

    // Build the element array buffer; this specifies the indices
    // into the vertex arrays for each face's vertices.

    // This array defines each face as two triangles, using the
    // indices into the vertex array to specify each triangle's
    // position.
    let indices: number[] = []
    let indexModifier = 0
    let tri_indices, indexBuffer
    let indexBuffers = new Map()

    shapes_.forEach(
        (shape, s_id, map_) => {
            indexBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer)

            // This array defines each face as two triangles, using the
            // indices into the vertex array to specify each triangle's
            // position.
            indices = []
            indexModifier = 0
            tri_indices = shape.get_tris().map(ind => ind + indexModifier)
            indices.push(...tri_indices)
            indexModifier += shape.numFaceVerts()

            // Now send the element array to GL
            gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,
                new Uint16Array(indices), gl.STATIC_DRAW)

            indexBuffers.set(s_id, indexBuffer)
        })

    // Now send the element array to GL
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,
        new Uint16Array(indices), gl.STATIC_DRAW)

    return {
        indexBuffers: indexBuffers,
        skybox: skyboxTexBuffer,
        skyboxPosits: skyboxPositBuffer
    }
}

export let staticBuffers = {}

const defaultCam = new Camera (
    new Float32Array([0., 0., 0., 0.]),
    [0., 0., 0., 0., 0., 0.],
    τ / 4.,
    4 / 3.,
    1.,
    100.,
    0.1,
    1.0,
)
export let cam = defaultCam

export const skybox = shapeMaker.make_skybox(100, cam.position)
skybox.make_tris()
// export const processedSkybox = transforms.processShapes(cam, new Map([[0, skybox]]))

// Imported values seem to be read-only, hence the setters.
export function setColorMax(val: number) {
    colorMax = val
}

export function setShapes(shapes_: Map<number, Shape>) {
    shapes = shapes_
}

export function setCam(cam_: Camera) {
    cam = cam_
}

export function updateStaticBuffers(gl: any) {
    staticBuffers = initBuffers(gl, shapes, skybox)
}
