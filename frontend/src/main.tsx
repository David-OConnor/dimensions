import * as React from 'react'
import {array} from 'numjs'

// import * as Rust from './unitalgebra'

// import {Button, Grid, Row, Col,
//     Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

import {Shape, Camera, ShapeArgs} from './interfaces'
import * as rustClone from './rust_clone'
import * as render from './render'

//
// import * as test from './dimensions'
// const js = import ("./dimensions")

// Not sure how to get TS to accept WebAssembly.
declare const WebAssembly: any
const τ = 2 * Math.PI

interface MainProps {
    state: any
    dispatch: Function
}

interface MainState {
    shapes: Map<number, Shape>
    camera: Camera
}

export class Main extends React.Component<any, any> {
    constructor(props: MainProps) {
        super(props)
        this.state = {
            shapes: new Map( // todo temp
                [[0, rustClone.make_box([1, 2, 1], array([2, 0, 2, 0]), 1,
                array([0, 0, 0, 0, 0, 0]), array([0, 0, 0, 0, 0, 0]))]]
            ),

            camera: {
                position: [0., 3., -6., -2.],
                θ_3d: [0., 0., 0.],
                θ_4d: [0., 0., 0., 0., 0., 0.],
                fov: τ / 5.,
                aspect: 1.,
                aspect_4: 1.,
                far: 30.,
                near: 0.9,
                strange: 1.0,
            }
        }
    }

    // const shapeArgss: ShapeArgs[] = [
    //     {
    //         name: 'box',
    //         lens: [0.5, 1, 1],
    //         position: [3, 0, 3, 0],
    //         scale: 1,
    //         orientation: [0, 0, 0, 0, 0, 0],
    //         rotation_speed: [.01, 0, 0, 0, 0, 0],
    //     },
    //     {
    //         name: 'origin',
    //         lens: [1],
    //         position: [0, 0, 0, 0],
    //         scale: 1,
    //         orientation: [0, 0, 0, 0, 0, 0],
    //         rotation_speed: [0, 0, 0, 0, 0, 0],
    //     },
    // ]

    // fetch('../../target/wasm32-unknown-unknown/release/dimensions.wasm')
    // WebAssembly.instantiateStreaming(fetch('dimensions_bg.wasm'))
    //     .then((wasm_module: any) => {
    //         let result = wasm_module.instance.exports.render_from_js()
    //         console.log(result)
    //     });

    componentDidMount() {
        let shape_list = [
            rustClone.make_box([1, 2, 1], array([2, 0, 2, 0]), 1,
                array([0, 0, 0, 0, 0, 0]), array([0, 0, 0, 0, 0, 0])),

                rustClone.make_origin(1, array([0, 0, 0, 0]), 1,
                    array([0, 0, 0, 0, 0, 0]), array([0, 0, 0, 0, 0, 0]))

        ]

        let shapes = new Map()
        for (let id=0; id < shape_list.length; id++) {
             shapes.set(id, shape_list[id])
        }

        this.setState({shapes: shapes})
    }

    render() {
        render.gl_main(this.state.camera, this.state.shapes)

        return (
            <div>
                <h1>Hello</h1>
            </div>
        )
    }
}
