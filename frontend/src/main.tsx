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
    constructor(props: any) {
        super(props)
    }
    render() {
        return (
            <Form>
                <ButtonGroup style={{marginTop: 20}}>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(0)}>Hypercube</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(1)}>Small world</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(2)}>Small town</Button>

                </ButtonGroup>
                <br />

                {/*<Button bsStyle="primary">There's no place like home</Button>*/}
            </Form>
        )
    }
}

const InstructionsOneShape = () => (
    <div>
        <h3>Controls</h3>
        <h4>Rotate</h4>
        <ul>

            <li>Left / right / up / down: Arrow keys</li>
            <li>Roll: Q / E</li>
            <li>4D A: Ins / Del</li>
            <li>4D B: Home / End</li>
            <li>4D C: PgUp / PgDn</li>
        </ul>
    </div>
)

const InstructionsFreeMove = () => (
    <div>
        <h3>Controls</h3>
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
            <li>Roll: Q / E</li>
            <li>4D A: Ins / Del</li>
            <li>4D B: Home / End</li>
            <li>4D C: PgUp / PgDn</li>
        </ul>
    </div>
)

const InstructionsFps = () => (
    <div>
        <h3>Controls</h3>
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

        this.setScene = this.setScene.bind(this)
    }

    setScene(scene: number) {
        this.setState({scene: scene})
    }

    render() {
        render.gl_main(this.state.scene)
        let instructions
        if (this.state.scene === 0) {
            instructions = <InstructionsOneShape />
        } else if (this.state.scene === 1) {
            instructions = <InstructionsFreeMove />
        } else if (this.state.scene === 2) {
            instructions = <InstructionsFps />
        } else {
            throw "Oops!"
        }

        return (
            <div>
                <Controls setScene={this.setScene} />
                {instructions}
                <br />
                <a href="http://www.youtube.com/watch?v=UnURElCzGc0&t=0m3s"><h4>Huh‽</h4></a>
            </div>
        )
    }
}

export default Main
