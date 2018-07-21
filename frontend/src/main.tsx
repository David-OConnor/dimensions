import * as React from 'react'
import * as ReactDOM from "react-dom"

const rust = import("./from_rust");

import {Button, Grid, Row, Col,
    Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

import * as render from './render'
import * as state from './state'
import * as util from './util'

// Not sure how to get TS to accept WebAssembly.
// declare const WebAssembly: any

interface MainProps {
    state: any
    dispatch: Function
}

class Controls extends React.Component<any, any> {
    constructor(props: any) {
        super(props)
    }
    render() {
        // Reference the scene lib in render_vulkano.rs
        return (
            <Form>
                <ButtonGroup style={{marginTop: 20}}>

                    <Button bsStyle="primary" onClick={() => this.props.setScene(0)}>Hypercube</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(1)}>5-cell</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(2)}>Spherinder</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(3)}>Cube</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(4)}>Pyramid</Button>
                    {/*<Button bsStyle="primary" onClick={() => this.props.setScene(5)}>Small world</Button>*/}
                    <Button bsStyle="primary" onClick={() => this.props.setScene(6)}>Grid</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(7)}>Plot</Button>
                    <Button bsStyle="primary" onClick={() => this.props.setScene(8)}>Origin</Button>
                </ButtonGroup>

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
            sceneId: 0,
        }

        this.setScene = this.setScene.bind(this)
    }

    setScene(sceneId: number) {
        state.emptyStaticBuffers()
        this.setState({sceneId: sceneId})
        state.setScene(sceneId)
    }

    setSubscene(subScene: number) {
        // state.emptyStaticBuffers()
        // this.setState({subScene: subScene})
        // state.setScene([this.state.scene, subScene])
    }

    render() {
        let instructions
        let camType = state.scene.cam_type  // code shortener
        if (camType === 'single') {
            instructions = <InstructionsOneShape />
        } else if (camType === 'free') {
            instructions = <InstructionsFreeMove />
        } else if (camType === 'fps') {
            instructions = <InstructionsFps />
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

rust.then(
    r => {
        state.setSceneLib(util.deserSceneLib(r.scene_lib()))
        state.setScene(1)
        // Don't render until we've imported and initialized the scenes.
        render.main()
        ReactDOM.render(<Main />, document.getElementById('root') as HTMLElement)
    }
)


