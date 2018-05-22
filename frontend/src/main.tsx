import * as React from 'react'

// import * as Rust from './unitalgebra'

// import {Button, Grid, Row, Col,
//     Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

import {Shape, Camera, ShapeArgs} from './interfaces'
import * as render from './render'

// Not sure how to get TS to accept WebAssembly.
declare const WebAssembly: any
const τ = 2 * Math.PI


export const Main = ({state, dispatch}: {state: any, dispatch: Function}) => {

    const shapeArgss: ShapeArgs[] = [
        {
            name: 'box',
            lens: [0.5, 1, 1],
            position: [3, 0, 3, 0],
            scale: 1,
            orientation: [0, 0, 0, 0, 0, 0],
            rotation_speed: [.01, 0, 0, 0, 0, 0],
        },
        {
            name: 'origin',
            lens: [1],
            position: [0, 0, 0, 0],
            orientation: [0, 0, 0, 0, 0, 0],
            rotation_speed: [0, 0, 0, 0, 0, 0],
        },
    ]

    let camera: Camera = {
        position: [0., 3., -6., -2.],
        θ_3d: [0., 0., 0.],
        θ_4d: [0., 0., 0., 0., 0., 0.],
        fov_hor: τ / 5.,
        fov_vert: τ / 5.,
        fov_strange: τ / 5.,
        clip_far: 30.,
        clip_near: 0.9,
        clip_strange: 1.0,
    }


    // fetch('../../target/wasm32-unknown-unknown/release/dimensions.wasm')
    fetch('dimensions.wasm')
        .then(r => r.arrayBuffer())
        .then(r => WebAssembly.instantiate(r))
        .then(wasmModule => {
            let projected = wasmModule.instance.exports.render_from_js(camera,  shapeArgss)
            console.log("Proj: ", projected)
            render.gl_main(projected)
        })

    return (
        <div>
            <h1>Hello</h1>
            {/*{Rust.greet("Data")}*/}
        </div>
    )
}
