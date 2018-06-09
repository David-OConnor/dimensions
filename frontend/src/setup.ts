// This file contains information to set up the camera, shapes, and some constants,
// based on the selected scene.

import * as transforms from './transforms';
import {makeV5} from './util';
import * as util from './util';
import * as shapeMaker from './shapeMaker';
import * as state from './state';
import {Camera} from './interfaces'

const τ = 2 * Math.PI
const empty = makeV5([0, 0, 0, 0])

let heightMap = [
    [1.3, 1.3, 0, 0, 0, 0, 0, 0, 1.2, 1.2],
    [1.3, 1.2, 0, 0, 0, 0, 0, 1.1, 1.2, 1.2],
    [0, 1.2, 1.2, 0, 0, 0, 0, 1.1, 1.2, 0],
    [0, 1.1, 0, 0, 0, 0, 0, 0, 1.1, 1.2],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1.2],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1.2],
    [0, 0, 0, 1.1, 0, 0, 0, 0, 1.1, 1.2],
    [0, 1.1, 0, 0, 0, 0, 0, 0, 1.2, 1.2],
    [0, 1.1, 1.1, 1.1, 1.1, 0, 1.3, 1.3, 2.4, 2.2],
    [0, 1.1, 1.1, 1.1, 1.2, 1.3, 1.3, 1.4, 2.4, 2.8]
]

let spissMap = [
    [5, 4, 2, 1.2, 0, 0, 1, 1, 2, 2.5],
    [5, 3, 2.5, 1.2, 0, 0, 0, 0, 2, 2.5],
    [5, 4, 2, 1, 2, 0, 0, 0, 2, 2.5],
    [4, 3, 2, 1, 0, 0, 0, 0, 2, 2.5],
    [4, 4, 3, 1, 0, 1, 0, 0, 2, 2.5],
    [6, 4, 3, 3.5, 1, 0, 0, 0, 2, 2.5],
    [6, 5.5, 5, 3.5, 2, 0, 0, 0, 2, 2.5],
    [6, 5.5, 5.5, 4, 2, 0, 0, 1, 2, 2.5],
    [6, 6, 6, 3.5, 2, 0, 0, 0, 2, 1.5],
    [7, 7, 7, 3.5, 2, 0, 0, 0, 2, 2.5],
]

let mapFlat = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
]

let mapFlat3d = [
    [...mapFlat], [...mapFlat], [...mapFlat], [...mapFlat], [...mapFlat],
    [...mapFlat], [...mapFlat], [...mapFlat], [...mapFlat], [...mapFlat],
]

// todo you could generate these with a loop
const housePositions = [
    [-8., 2, 0., 0.],
    [-8., 2, 12., 0.],
    [-8., 2, 24., 0.],
    [-8., 2, 36., 0.],

    [8., 2, 0., 0.],
    [8., 2, 12., 0.],
    [8., 2, 24., 0.],
    [8., 2, 36., 0.],

    [-8., 2, 0., 4.],
    [-8., 2, 12., 4.],
    [-8., 2, 24., 4.],
    [-8., 2, 36., 4.],

    [8., 2, 0., 4.],
    [8., 2, 12., 4.],
    [8., 2, 24., 4.],
    [8., 2, 36., 4.],
]

const houses = housePositions.map(posit => shapeMaker.make_house([4., 4., 4.],
    makeV5(posit), [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]))

export function setScene(scene: number, subScene: number) {
    document.onkeydown = e => transforms.handleKeyDown(e, scene)
    if (scene === 0) {  // Single hypercube
        state.setCam(new Camera(
            makeV5([0., 0., -2., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 5.5,
            4 / 3.,
            1.,
            200.,
            0.1,
            1.0,
        ))

        let selectedShape
        if (subScene === 0) {
            selectedShape = shapeMaker.make_hypercube(1, empty,
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
        } else if (subScene === 1) {
            selectedShape = shapeMaker.make_5cell(2, empty,
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
        } else if (subScene === 2) {
            selectedShape = shapeMaker.make_cube(1, util.makeV5([0, 0, 0, 0]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
        } else {
            throw "Oops; a non-existant key was selected. :("
        }
        let shapes = new Map()
        shapes.set(
            0,
            selectedShape
        )
        state.setShapes(shapes)

        state.setColorMax(1.5)
    } else if (scene === 1) {  // Terain with shapes
        state.setCam(new Camera(
            makeV5([0., 2., -3., 0.]),
            [0., -.5, 0., 0., 0., 0.],
            τ / 4.,
            4 / 3.,
            1.,
            200.,
            0.1,
            1.0,
        ))

        let shapeList0 = [
            shapeMaker.make_terrain([20, 20], 10, heightMap, spissMap, empty),

            shapeMaker.make_box([1, 2, 1], makeV5([-1, 3, 4, 1]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

            shapeMaker.make_rectangular_pyramid([2, 1, 2], makeV5([-2, 3, 3, -1]),
                [τ/6, τ/3, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

            shapeMaker.make_cube(1, makeV5([2, 0, 5, 2]),
                [0, 0, 0, 0, 0, 0], [.002, 0, 0, 0, 0, 0]),

            // On ana of other cube.
            shapeMaker.make_cube(1, makeV5([2, 0, 5, 10]),
                [0, 0, 0, 0, 0, 0], [.002, 0, 0, 0, 0, 0]),

            shapeMaker.make_hypercube(1, makeV5([3, 3, 3, 0]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, .002, .0005, .001]),

            shapeMaker.make_hypercube(1, makeV5([-3, 1, 0, 1.5]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

            // rustClone.make_origin(1, new Vec5([0, 0, 0, 0]), 1,
            //     [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
        ]

        let shapes = new Map()
        for (let id = 0; id < shapeList0.length; id++) {
            shapes.set(id, shapeList0[id])
        }
        state.setShapes(shapes)

        state.setColorMax(10)
    } else if (scene === 2) {  // Terain with shapes
        state.setCam(new Camera(
            makeV5([0., 3., -3., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 4.,
            4 / 3.,
            1.,
            1000.,
            0.1,
            1.0,
        ))

        let shapeTownList = [
            shapeMaker.make_terrain([1000, 1000], 10, mapFlat, mapFlat,
                empty),
            ...houses
        ]

        let shapes = new Map()
        for (let id = 0; id < shapeTownList.length; id++) {
            shapes.set(id, shapeTownList[id])
        }
        state.setShapes(shapes)

        state.setColorMax(30)
    } else if (scene === 3) {  // Hypergrid
        state.setCam(new Camera(
            makeV5([0., 0., 0., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 4.,
            4 / 3.,
            1.,
            1000.,
            0.1,
            1.0,
        ))

        let mapTest = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]

        let mapTestWarped = [
            [0, -3, -4, -5],
            [2, 0, -3, -4],
            [-1, 0, -2, -3],
            [-1, 0, -1, -2],
        ]

        let mapTest3d = [
            [...mapTest], [...mapTest], [...mapTest], [...mapTest]
        ]

        let mapTest3dWarped = [
            [...mapTest], [...mapTestWarped], [...mapTestWarped], [...mapTest]
        ]

        const spissMap_ = subScene === 1 ? mapTest3dWarped : mapTest3d

        state.setShapes(
            shapeMaker.make_cube_hypergrid([10, 10, 10], 4, spissMap_, empty)
        )
        state.setColorMax(30)
    } else {
        throw "Nonexistant scene selected."
    }
}