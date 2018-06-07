import * as React from 'react'

import {Button, Grid, Row, Col,
    Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

import {Shape, Camera} from './interfaces'
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

class Controls extends React.Component<any, any> {

    render() {
        return (
            <Form>

                <Button>There's no place like home</Button>
            </Form>
        )
    }
}

const InstructionsScene0 = () => (
    <div>
        <h3>Controls:</h3>
        <h4>Rotate hypercube</h4>
        <ul>

            <li>Left / right / up / down: Arrow keys</li>
            <li>Roll left / right: Q / E</li>
            <li>4D A: Ins / Del</li>
            <li>4D B: Home / End</li>
            <li>4D C: PgUp / PgDn</li>
        </ul>
    </div>
)

const InstructionsScene1 = () => (
    <div>
        <h3>Controls:</h3>
        <h4>Move</h4>
        <ul>

            <li>Forward: W</li>
            <li>Back: S</li>
            <li>Left: A</li>
            <li>Right: D</li>
            <li>Ana: R</li>
            <li>Kata: F</li>
        </ul>

        <h4>Look</h4>
        <ul>
            <li>Left / right / up / down: Arrow keys</li>
            <li>Roll left / right: Q / E</li>
            <li>4D A: Ins / Del</li>
            <li>4D B: Home / End</li>
            <li>4D C: PgUp / PgDn</li>
        </ul>
    </div>
)

class Main extends React.Component<any, any> {
    constructor(props: MainProps) {
        super(props)
        this.state = {
            scene: 0
        }
    }

    render() {
        render.gl_main(this.state.scene)
        let instructions
        if (this.state.scene === 0) {
            instructions = <InstructionsScene0 />
        } else if (this.state.scene === 1) {
            instructions = <InstructionsScene1 />
        } else {
            throw "Oops!"
        }

        return (
            <div>
                {instructions}
                <Controls />
            </div>
        )
    }
}

export default Main
