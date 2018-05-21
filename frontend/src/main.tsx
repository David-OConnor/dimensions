import * as React from 'react'

// import * as Rust from './unitalgebra'

// import {Button, Grid, Row, Col,
//     Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

import {Shape} from './interfaces'
import * as render from './render'

// Not sure how to get TS to accept WebAssembly.
declare const WebAssembly: any

export const Main = ({state, dispatch}: {state: any, dispatch: Function}) => {

    // const shapes = [
    //     Shape {
    //
    //     }
    // ]

    render.gl_main(new Map)

      // fetch('../../target/wasm32-unknown-unknown/release/dimensions.wasm')
      fetch('dimensions.wasm')
        .then(r => r.arrayBuffer())
        .then(r => WebAssembly.instantiate(r))
        .then(wasmModule => {
            alert(`2 + 1 = ${wasmModule.instance.exports.add_one(2)}`);
        });

    return (
        <div>
            <h1>Hello</h1>
            {/*{Rust.greet("Data")}*/}
        </div>
    )
}
