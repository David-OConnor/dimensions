import {Camera, Shape} from "./interfaces";
import * as transforms from "./transforms";
import * as shapeMaker from "./shapeMaker";
import {makeV5} from './util'

// todo global shapes and cam for now
const τ = 2 * Math.PI

export let shapes = new Map()

export let colorMax = 15  // At this z distance, our blue/red shift fully saturated.
export let currentlyPressedKeys: number[] = []
export const moveSensitivity = .1
export const rotateSensitivity = .017

const defaultCam = new Camera (
    makeV5([0., 0., 0., 0.]),
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
export const processedSkybox = transforms.processShapes(cam, new Map([[0, skybox]]))

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
