import {Camera, Shape} from "./types";
import * as shapeMaker from "./shapeMaker";

// todo global shapes and cam for now
const τ = 2 * Math.PI

export let shapes = new Map()

export let colorMax = 15  // At this z distance, our blue/red shift fully saturated.
export let currentlyPressedKeys: number[] = []
export const moveSensitivity = .1  // units per millisecond
export const rotateSensitivity = .3  // radians per millisecond.

export let staticBuffers = {}

export let scene = 0

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

export let skybox = shapeMaker.make_skybox(100, cam.position)

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

export function setScene(scene_: number) {
    scene = scene_
}