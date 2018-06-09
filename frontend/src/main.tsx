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

class Controls extends React.Component<any, any> {
    constructor(props: any) {
        super(props)
    }
    render() {
        const shapeButtons = (
            <ButtonGroup style={{marginTop: 20}}>
                <Button bsStyle="primary" onClick={() => this.props.setSubscene(0)}>Hypercube</Button>
                <Button bsStyle="primary" onClick={() => this.props.setSubscene(1)}>5-Cell</Button>
                <Button bsStyle="primary" onClick={() => this.props.setSubscene(2)}>Cube</Button>
            </ButtonGroup>
        )

        const hyperButtons = (
            <ButtonGroup style={{marginTop: 20}}>
                <Button bsStyle="primary" onClick={() => this.props.setSubscene(0)}>Uniform</Button>
                <Button bsStyle="primary" onClick={() => this.props.setSubscene(1)}>Warped</Button>
            </ButtonGroup>
        )

        return (
            <Form>
                <ButtonGroup style={{marginTop: 20}}>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(0)}>Singles</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(1)}>Small world</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(2)}>Small town</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(3)}>Hyper grid</Button>
                </ButtonGroup>

                <br />
                {this.props.showShapeBtns ? shapeButtons : null}
                {this.props.showHyperBtns ? hyperButtons : null}
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
            <li>Up: Space</li>
            <li>Down: ctrl or c</li>
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
            scene: 0,
            subScene: 0  // The shape to display for scene 0.
        }

        this.setScene = this.setScene.bind(this)
        this.setSubscene = this.setSubscene.bind(this)
    }

    setScene(scene: number) {
        this.setState({scene: scene})
    }

    setSubscene(subScene: number) {
        this.setState({subScene: subScene})
    }

    // Scene descriptions:
    // 0: Single 4d shape; controls rotate it
    // 1: A world with scattered 3d and 4d shapes at varying positions; a terrain
    // mesh mapped to various points in dimensions 3 and 4.
    // 2: A small town; doubled up in the 4th dimension
    // 3: A hyper lattice; currently of cubes.

    render() {
        render.gl_main(this.state.scene, this.state.subScene)

        let instructions
        if (this.state.scene === 0) {
            instructions = <InstructionsOneShape />
        } else if (this.state.scene === 1) {
            instructions = <InstructionsFreeMove />
        } else if (this.state.scene === 2) {
            instructions = <InstructionsFps />
        } else if (this.state.scene === 3) {
            instructions = <InstructionsFreeMove />
        } else {
            throw "Oops!"
        }

        return (
            <div>
                <Controls
                    setScene={this.setScene}
                    setSubscene={this.setSubscene}
                    showShapeBtns={this.state.scene === 0}
                    showHyperBtns={this.state.scene === 3}
                />
                {instructions}
                <br />
                <a href="http://www.youtube.com/watch?v=UnURElCzGc0&t=0m3s"><h4>Huh‽</h4></a>
            </div>
        )
    }
}

export default Main
