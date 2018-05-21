import * as React from 'react'

// import * as Rust from './unitalgebra'

// import {Button, Grid, Row, Col,
//     Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

import {Shape} from './interfaces'
import * as render from './render'


export const Main = ({state, dispatch}: {state: any, dispatch: Function}) => {

    const shapes = [
        Shape {

        }
    ]


    render.gl_main(shapes)

    return (
        <div>
            <h1>Hello</h1>
            {/*{Rust.greet("Data")}*/}
        </div>
    )
}
