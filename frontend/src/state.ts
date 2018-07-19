import {Camera, Lighting, Mesh, Normal, Scene, Shape, Vertex} from "./types";

// todo global shapes and cam for now
const τ = 2 * Math.PI

export let currentlyPressedKeys: number[] = []
export const moveSensitivity = 1.  // units per millisecond
export const rotateSensitivity = 0.7  // radians per millisecond.

export let staticBuffers = {}


// todo temp
const vertices = new Map()
vertices.set(0, {position: [0., 0., 0., 0.]} as Vertex)
vertices.set(1, {position: [0., 1., 0., 0.]} as Vertex)
vertices.set(2, {position: [1., 0., 0., 0.]} as Vertex)
// const normals
const mesh = new Mesh(
    vertices,
    [new Uint16Array([0, 1, 2])],
    [
        {normal: [0., 0., 1., 0.]} as Normal,
        {normal: [0., 0., 1., 0.]} as Normal,
        {normal: [0., 0., 1., 0.]} as Normal
    ],
)
mesh.makeTris()

const shapes = new Map()
shapes.set(
    0,
    new Shape(
        mesh,
        new Float32Array([0., 0., 0., 0.]),
        [0., 0., 0., 0., 0., 0.],
        [0., 0., 0., 0., 0., 0.],
        1.
    )
)

const cam = new Camera(
    new Float32Array([0., 0., 0., 0.]),
    [0., 0., 0., 0., 0., 0.],
    Math.PI / 2.,
    1.,
    1.,
    1.,
    .1,
    100,
    100
)

const lighting: Lighting = {
    ambient_intensity: 1.,
    diffuse_intensity: 1.,
    specular_intensity: 1.,
    ambient_color: [1., 1., 1., 1.],
    diffuse_color: [1., 1., 1., 1.],
    diffuse_direction: [1., 1., 1., 1.],
    sources: []
}

const defaultScene =  {
        shapes: shapes,
        cam: cam,
        cam_type: "free",
        lighting: lighting,
        color_max: 1.,
        sensitivities: [0.1, 0.1, 0.1]
    }
// todo temp to avoid an uninitialized scene var.

export let sceneLib: Map<number, Scene> = new Map()
export let scene = defaultScene
// export let scene = 0  // todo testing wheather to store this as num or scene.

export function setSceneLib(sceneLib_: Map<number, Scene>) {
    sceneLib = sceneLib_
}

export function setScene(sceneId: number) {
    scene = sceneLib.get(sceneId)
    // scene = sceneId
}

export function updateStaticBuffers(gl: any, buffers: any) {
    staticBuffers = buffers
}

export function emptyStaticBuffers() {
    // Render checks if static buffers are empty each pass; if so, it'll
    // call updateStaticBuffers. We call this function when resetting the shapes.
    // We have these two steps so we can initiate the reset from the UI, which
    // doesn't have access to the GL object directly; render owns it.
    staticBuffers = {}
}
