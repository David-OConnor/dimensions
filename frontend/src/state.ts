import {Camera, Lighting, Mesh, Normal, Scene, Shape, Vertex} from "./types";

// todo global shapes and cam for now
const τ = 2 * Math.PI

export let currentlyPressedKeys: number[] = []
export const moveSensitivity = 1.  // units per millisecond
export const rotateSensitivity = 0.7  // radians per millisecond.

export let staticBuffers = {}

export let sceneLib: Map<number, Scene> = new Map()

export let scene = sceneLib.get(0)

export function setSceneLib(sceneLib_: Map<number, Scene>) {
    sceneLib = sceneLib_
}

export function setScene(scene_: Scene) {
    scene = scene_
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
