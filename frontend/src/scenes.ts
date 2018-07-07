// This file contains information to set up the camera, shapes, and some constants,
// based on the selected scene.  Similar to scenes.rs

import * as shapeMaker from './shapeMaker';
import * as state from './state';
import {Camera, Lighting, Scene} from './types'

const τ = 2 * Math.PI
const EMPTY = new Float32Array([0, 0, 0, 0])

const ASPECT = 4 / 3;  // this must match gl canvas width and height.

const baseLighting: Lighting = {
            ambientIntensity: 0.4,
            diffuseIntensity: 0.3,
            specularIntensity: 0.3,
            ambientColor: [1.0, 1.0, 1.0, 0.4],
            diffuseColor: [0., 1., 0., 0.2],
            diffuseDirection: [1., 0., 0., 0.],
}

function make2dGridEmpty(size: number): number[][] {
    // todo we have make3d grids in scenes.
    let outer = [], inner
    for (let i=0; i < size; i++) {
        inner = []
        for (let j=0; j < size; j++) {
            inner.push(0)
        }
        outer.push(inner)
    }
    return outer
}

function make3dGridEmpty(size: number): number[][][] {
    let outer = [], middle, inner
    for (let i=0; i < size; i++) {
        middle = []
        for (let j=0; j < size; j++) {
            inner = []
            for (let k=0; k < size; k++) {
                inner.push(0)
            }
            middle.push(inner)
        }
        outer.push(middle)
    }
    return outer
}

function make4dGridEmpty(size: number): number[][][][] {
    let outer = [], middle, inner, superInner
    for (let i=0; i < size; i++) {
        middle = []
        for (let j=0; j < size; j++) {
            inner = []
            for (let k=0; k < size; k++) {
                superInner = []
                for (let l=0; l < size; l++) {
                    superInner.push(0)
                }
                inner.push(superInner)
            }
            middle.push(inner)
        }
        outer.push(middle)
    }
    return outer
}

function genFractalHeightmap(size: number, map: number[][],
                              roughness: number, depth: number): number[][] {
    // Experimenting with fractals to create pseudo-natural terrain.

    // Alter the middle third of a random square segment.
    // todo: Doesn't need to be square.
    let lowerBoundX, upperBoundX, lowerBoundY, upperBoundY, workingSize
    for (let d=0; d < depth; d++) {
        workingSize = size * Math.random()

        lowerBoundX = workingSize * Math.random()
        upperBoundX = lowerBoundX + (workingSize - lowerBoundX)

        lowerBoundY = workingSize * Math.random()
        upperBoundY = lowerBoundY + (workingSize - lowerBoundX)

            for (let i=0; i < size; i++) {
                for (let j=0; j < size; j++) {
                    if (i >= lowerBoundX && i <= upperBoundX &&
                        j >= lowerBoundY && j <= upperBoundY ) {
                        // Offset height in range -1 to +1 * roughness
                        map[i][j] += (Math.random() * 2. - 1.) * roughness
                    }
                }
            }
    }
    return map

}

let testMap = genFractalHeightmap(10, make2dGridEmpty(10), .1, 300)
console.log(testMap, "TEST")

let hypercubeScene: Scene
{
    let hypercubeShapes = new Map;
    hypercubeShapes.set(0, shapeMaker.make_hypercube(1, EMPTY,
        [0., 0., 0., 0., 0., 0.], [0., 0., 0., 0., 0., 0.]))
    hypercubeScene = {
        id: 0,
        shapes: hypercubeShapes,
        camStart: new Camera(
            new Float32Array([0., 0., -2., 0.]),
            [0., 0., τ / 2, 0., 0., 0.],
            τ / 5.5,
            ASPECT,
            1.,
            200.,
            0.1,
            1.0,
        ),
        camType: 'single',
        colorMax: 0.4,
        lighting: baseLighting
    }
}

// Such a janky way of cloning...
let fivecellScene = JSON.parse(JSON.stringify(hypercubeScene))

// JSON converts F32Arrays to objects.
fivecellScene.ambientLightColor = new Float32Array([0.2, 0.2, 0.2, 0.5])
fivecellScene.diffuseLightColor = new Float32Array([0., 1., 0., 0.5])
fivecellScene.diffuseLightDirection = new Float32Array([1./Math.sqrt(2.), -1./Math.sqrt(2.), 0., 0.])

fivecellScene.shapes = new Map()
fivecellScene.shapes.set(0, shapeMaker.make5Cell(2, EMPTY,
    [0., 0., 0., 0., 0., 0.], [0., 0., 0., 0., 0., 0.]))

let cubeScene = JSON.parse(JSON.stringify(hypercubeScene))

// JSON converts F32Arrays to objects.
cubeScene.ambientLightColor = new Float32Array([0.2, 0.2, 0.2, 0.5])
cubeScene.diffuseLightColor = new Float32Array([0., 1., 0., 0.5])
cubeScene.diffuseLightDirection = new Float32Array([1., 0., 0., 0.])

cubeScene.shapes = new Map()
cubeScene.shapes.set(0, shapeMaker.makeCube(1, EMPTY,
    [0., 0., 0., 0., 0., 0.], [0., 0., 0., 0., 0., 0.]))

let worldScene: Scene
{
    let shapeList = [
        shapeMaker.makeTerrain([200., 200.], 10,
            genFractalHeightmap(10, make2dGridEmpty(300), 4, 30),
            genFractalHeightmap(10, make2dGridEmpty(300), 4, 30),
            EMPTY),

        shapeMaker.makeBox([1., 2., 1.], new Float32Array([-1., 3., 4., 1.]),
            [0., 0., 0., 0., 0., 0.], [0., 0., 0., 0., 0., 0.]),

        shapeMaker.makeRectangularPyramid([2., 1., 2.], new Float32Array([-2., 3., 3., -1.]),
            [τ/6, τ/3, 0, 0, 0, 0], [0., 0., 0., 0., 0., 0.]),

        shapeMaker.makeCube(1., new Float32Array([2., 0., 5., 2.]),
            [0., 0., 0., 0., 0., 0.], [.002, 0, 0, 0, 0, 0]),

        // On ana of other cube.
        shapeMaker.makeCube(1., new Float32Array([2., 0., 5., 10.]),
            [0., 0., 0., 0., 0., 0.], [.002, 0, 0, 0, 0, 0]),

        shapeMaker.make_hypercube(1, new Float32Array([3., 3., 3., 0.]),
            [0., 0., 0., 0., 0., 0.], [0, 0, 0, .002, .0005, .001]),

        shapeMaker.make_hypercube(1, new Float32Array([-3., 1, 0., 1.5]),
            [0., 0., 0., 0., 0., 0.], [0., 0., 0., 0., 0., 0.]),

        // rustClone.make_origin(1, new Vec5([0, 0, 0, 0]), 1,
        //     [0., 0., 0., 0., 0., 0.], [0., 0., 0., 0., 0., 0.])
    ]

    let shapes = new Map()
    for (let id = 0; id < shapeList.length; id++) {
        shapes.set(id, shapeList[id])
    }

    worldScene = {
        id: 1,
        shapes: shapes,
        camStart: new Camera(
            new Float32Array([0., 2., -3., 0.]),
            [0., 0, τ/2., 0., 0., 0.],
            τ / 4.,
            ASPECT,
            1.,
            200.,
            0.1,
            1.0,
        ),
        camType: 'free',
        colorMax: 10.,
        lighting: baseLighting
    }

}

let townScene: Scene
{
    // todo you could generate these with a loop
    const housePositions = [
        [-8., 2, 0., -2.],
        [-8., 2, 12., -2.],
        [-8., 2, 24., -2.],
        [-8., 2, 36., -2.],

        [8., 2, 0., -2.],
        [8., 2, 12., -2.],
        [8., 2, 24., -2.],
        [8., 2, 36., -2.],

        [-8., 2, 0., 2.],
        [-8., 2, 12., 2.],
        [-8., 2, 24., 2.],
        [-8., 2, 36., 2.],

        [8., 2, 0., 2.],
        [8., 2, 12., 2.],
        [8., 2, 24., 2.],
        [8., 2, 36., 2.],
    ]

    const houses = housePositions.map(posit => shapeMaker.make_house([4., 4., 4.],
        new Float32Array(posit), [0., 0., 0., 0., 0., 0.], [0., 0., 0., 0., 0., 0.]))

    let shapeTownList = [
        shapeMaker.makeTerrain([1000., 1000.], 10, make2dGridEmpty(10), make2dGridEmpty(10),
            EMPTY),
        ...houses
    ]

    let townShapes = new Map()
    for (let id = 0; id < shapeTownList.length; id++) {
        townShapes.set(id, shapeTownList[id])
    }

    townScene = {
        id: 2,
        shapes: townShapes,
        camStart: new Camera(
            new Float32Array([0., 0., -2., 0.]),
            [0., 0., τ / 2, 0., 0., 0.],
            τ / 5.5,
            ASPECT,
            1.,
            200.,
            0.1,
            1.0,
        ),
        camType: 'fps',
        colorMax: 10.,
        lighting: baseLighting
    }
}

let gridScene: Scene
{
    const gridSize = 14
    const shapes = shapeMaker.makeHypergrid([200, 200, 200], gridSize, make3dGridEmpty(gridSize), EMPTY)

    gridScene = {
        id: 3,
        shapes: shapes,
        camStart: new Camera(
            new Float32Array([0., 0., 0., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 4.,
            ASPECT,
            1.,
            1000.,
            0.1,
            1.0,
        ),
        camType: 'free',
        colorMax: 30,
        lighting: baseLighting
    }
}

let gridSceneWarped: Scene
{
     const mapTest = [
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ]

    const mapTestWarped = [
        [1, 1.5, 1.5, 1.5, 1.5, 0, 0, 0],
        [1.5, 2, 2.5, 2, 1, 1, 0, 0],
        [1, 2.5, 3, 2.5, 2, 1, 0, 0],
        [1.5, 2, 2.5, 2, 1.5, 0, 0, 0],
        [1, 1.5, 1.5, 1.5, 1, 0, 0, 0],
        [1, 1, 1, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ]

    const mapTest3dWarped = [
        [...mapTest], [...mapTestWarped], [...mapTestWarped], [...mapTest],
        [...mapTest], [...mapTestWarped], [...mapTestWarped], [...mapTest]
    ]

    const gridSize = 8
    const shapes = shapeMaker.makeHypergrid([20, 20, 20], gridSize, mapTest3dWarped, EMPTY)

    gridSceneWarped = {
        id: 3,
        shapes: shapes,
        camStart: new Camera(
            new Float32Array([0., 0., 0., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 4.,
            ASPECT,
            1.,
            1000.,
            0.1,
            1.0,
        ),
        camType: 'free',
        colorMax: 30,
        lighting: baseLighting
    }
}

let gridScene4d: Scene
{
    const gridSize = 10
    const shapes = shapeMaker.makeCubeHypergrid4d([200, 200, 200, 200], gridSize, make4dGridEmpty(gridSize), EMPTY)

    gridScene4d = {
        id: 3,
        shapes: shapes,
        camStart: new Camera(
            new Float32Array([0., 0., 0., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 4.,
            ASPECT,
            1.,
            1000.,
            0.1,
            1.0,
        ),
        camType: 'free',
        colorMax: 30,
        lighting: baseLighting
    }
}

const sceneMap = new Map()
sceneMap.set([0, 0].join(','), hypercubeScene)
sceneMap.set([0, 1].join(','), fivecellScene)
sceneMap.set([0, 2].join(','), cubeScene)
sceneMap.set([0, 0].join(','), hypercubeScene)
sceneMap.set([1, 0].join(','), worldScene)
sceneMap.set([2, 0].join(','), townScene)
sceneMap.set([3, 0].join(','), gridScene)
sceneMap.set([3, 1].join(','), gridSceneWarped)
sceneMap.set([3, 2].join(','), gridScene4d)

export function setScene(sceneId: [number, number]) {
    // Map doesn't support tuples; use string instead.
    const scene = sceneMap.get(sceneId.join(','))

    state.setCam(scene.camStart)
    state.setCamType(scene.camType)
    state.setShapes(scene.shapes)
    state.setColorMax(scene.colorMax)
    state.setLighting(scene.lighting)
}
