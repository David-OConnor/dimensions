import {mat4, glMatrix} from 'gl-matrix'

// import * as Rust from './unitalgebra'

// import {Button, Grid, Row, Col,
//     Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

import * as rustClone from './rust_clone'
import {ProgramInfo, Shape, Camera, Vec5} from './interfaces'

let cubeRotation = 0.0;

// todo global shapes and cam for now
const τ = 2 * Math.PI

let currentlyPressedKeys = {}
const moveSensitivity = .15
const rotateSensitivity = .04

function handleKeyDown(event: any) {
    currentlyPressedKeys[event.keyCode] = true
    switch(event.keyCode) {
        case 87:  // w
            cam.position.vals[2] += moveSensitivity
            break
        case 83:  // s
            cam.position.vals[2] -= moveSensitivity
            break
        case 68:  // d
            cam.position.vals[0] += moveSensitivity
            break
        case 65:  // a
            cam.position.vals[0] -= moveSensitivity
            break
        case 32:  // Space
            cam.position.vals[1] += moveSensitivity
            break
        case 67:  // c
            cam.position.vals[1] -= moveSensitivity
            break
        case 17:  // Control
            cam.position.vals[1] -= moveSensitivity
            break
        case 82:  // r
            cam.position.vals[3] += moveSensitivity
            break
        case 70:  // f
            cam.position.vals[3] -= moveSensitivity
            break

        case 38:  // Up
            cam.θ_4d[1] +=rotateSensitivity
            break
        case 40:  // Down
            cam.θ_4d[1] -=rotateSensitivity
            break
        case 39:  // Right
            cam.θ_4d[2] -=rotateSensitivity
            break
        case 37:  // Left
            cam.θ_4d[2] +=rotateSensitivity
            break
        case 69:  // E
            cam.θ_4d[0] +=rotateSensitivity
            break
        case 81:  // Q
            cam.θ_4d[0] -=rotateSensitivity
            break
        case 84:  // t
            cam.θ_4d[3] +=rotateSensitivity
            break
        case 71:  // g
            cam.θ_4d[3] -=rotateSensitivity
            break
        case 89:  // y
            cam.θ_4d[4] +=rotateSensitivity
            break
        case 72:  // h
            cam.θ_4d[4] -=rotateSensitivity
            break
        case 85:  // u
            cam.θ_4d[5] +=rotateSensitivity
            break
        case 74:  // j
            cam.θ_4d[5] -=rotateSensitivity
            break

        default:
            break
    }
}

function handleKeyUp(event: any) {
    currentlyPressedKeys[event.keyCode] = false;
}

let cam = {
    position: new Vec5([0., 0., -3., 0.]),
    θ_3d: [0., 0., 0.],
    θ_4d: [0., -.5, 0., 0., 0., 0.],
    fov: τ / 4.,
    aspect: 640 / 480.,
    aspect_4: 1.,
    far: 30.,
    near: 0.1,
    strange: 1.0,
}

let shape_list = [
    rustClone.make_box([1, 2, 1], new Vec5([-1, 3, 4, 0]), 1,
        [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

    rustClone.make_cube(1, new Vec5([2, 0, 5, 0]), 1,
        [0, 0, 0, 0, 0, 0], [.002, 0, 0, 0, 0, 0]),

    // On sky of other cube.
    rustClone.make_cube(1, new Vec5([2, 0, 5, 4]), 1,
        [0, 0, 0, 0, 0, 0], [.002, 0, 0, 0, 0, 0]),

    rustClone.make_hypercube(1, new Vec5([3, 3, 3, 0]), 1,
        [0, 0, 0, 0, 0, 0], [0, 0, 0, .005, .005, .004]),

    rustClone.make_hypercube(1, new Vec5([-3, 0, 3, 0]), 1,
        [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

    // rustClone.make_origin(1, new Vec5([0, 0, 0, 0]), 1,
    //     [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])

]

let shapes = new Map()
for (let id=0; id < shape_list.length; id++) {
    shapes.set(id, shape_list[id])
}

// function glVerticesFromShape(shapes: Map<number, Shape>): number[]{
//     // WebGL needs us to define a vertex for each face edge; this involves
//     // duplicating vertices from shapes.
// }

function initShaderProgram(gl: any, vsSource: any, fsSource: any) {
    // Initialize a shader program, so WebGL knows how to draw our data

    const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource)
    const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource)

    // Create the shader program
    const shaderProgram = gl.createProgram()
    gl.attachShader(shaderProgram, vertexShader)
    gl.attachShader(shaderProgram, fragmentShader)
    gl.linkProgram(shaderProgram)

    // If creating the shader program failed, alert
    if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
        alert('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram))
        return null
    }

    return shaderProgram
}

function loadShader(gl: any, type: any, source: any) {
    // creates a shader of the given type, uploads the source and
    // compiles it.
    const shader = gl.createShader(type)

    // Send the source to the shader object
    gl.shaderSource(shader, source)

    // Compile the shader program
    gl.compileShader(shader)

    // See if it compiled successfully
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        alert('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader))
        gl.deleteShader(shader)
        return null
    }

    return shader
}

function drawScene(gl: any, programInfo: ProgramInfo, buffers: any,
                   deltaTime: number,
                   processedShapes: Map<string, Float32Array>, vertexCount: number) {
    gl.clearColor(0.0, 0.0, 0.0, 1.0)  // Clear to black, fully opaque

    // These settings affect transparency.
    gl.clearDepth(1.0)                 // Clear everything
    // gl.enable(gl.DEPTH_TEST)           // Enable depth testing
    // gl.depthFunc(gl.LEQUAL)            // Near things obscure far things
    gl.disable(gl.DEPTH_TEST);
    // gl.disable(gl.CULL_FACE)  // transparency TS

    gl.enable(gl.BLEND);
    gl.blendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
    // gl.blendFunc(gl.ONE, gl.ONE_MINUS_SRC_ALPHA)
    // gl.blendFuncSeparate(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA, gl.ONE, gl.ONE_MINUS_SRC_ALPHA);

    // gl.blendFunc(gl.SRC_ALPHA, gl.ONE);

    // Clear the canvas before we start drawing on it.
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

    const processedShapes_ = preProcessShapes(cam, shapes)
    buffers = initBuffers(gl, processedShapes_)

    // console.log(shapes, "S")

    // Create a perspective matrix, a special matrix that is
    // used to simulate the distortion of perspective in a camera.
    // Our field of view is 45 degrees, with a width/height
    // ratio that matches the display size of the canvas
    // and we only want to see objects between 0.1 units
    // and 100 units away from the camera.

    // const aspect = gl.canvas.clientWidth / gl.canvas.clientHeight
    let projectionMatrix = mat4.create()

    // note: glmatrix.js always has the first argument
    // as the destination to receive the result.
    mat4.perspective(
        projectionMatrix,
        cam.fov,
        cam.aspect,
        cam.near,
        cam.far
    )
        // todo add in Rview and Tview later.

    // Set the drawing position to the "identity" point, which is
    // the center of the scene.

    // We've positioned our points rel to their model and the cam already,
    // using 4d transforms; doesn't modify further.
    const I_4 = new Float32Array([
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, -1, 0,
        0, 0, 0, 1
    ])

    // Tell WebGL how to pull out the positions from the position
    // buffer into the vertexPosition attribute.
    {
        const numComponents = 3  // pull out 3 values per iteration
        const type = gl.FLOAT    // the data in the buffer is 32bit floats
        const normalize = false  // don't normalize
        const stride = 0         // how many bytes to get from one set of values to the next
                                 // 0 = use type and numComponents above
        const offset = 0         // how many bytes inside the buffer to start from

        gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position)
        gl.vertexAttribPointer(
            programInfo.attribLocations.vertexPosition,
            numComponents,
            type,
            normalize,
            stride,
            offset)
        gl.enableVertexAttribArray(
            programInfo.attribLocations.vertexPosition)
    }

    // Tell WebGL how to pull out the colors from the color buffer
    // into the vertexColor attribute.
    {
        const numComponents = 4
        const type = gl.FLOAT
        const normalize = false
        const stride = 0
        const offset = 0
        gl.bindBuffer(gl.ARRAY_BUFFER, buffers.color)
        gl.vertexAttribPointer(
            programInfo.attribLocations.vertexColor,
            numComponents,
            type,
            normalize,
            stride,
            offset)
        gl.enableVertexAttribArray(
            programInfo.attribLocations.vertexColor);
    }

    // Tell WebGL which indices to use to index the vertices
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, buffers.indices)

    // Tell WebGL to use our program when drawing
    gl.useProgram(programInfo.program)

    // Set the shader uniforms
    gl.uniformMatrix4fv(
        programInfo.uniformLocations.projectionMatrix,
        false,
        projectionMatrix)
    gl.uniformMatrix4fv(
        programInfo.uniformLocations.modelViewMatrix,
        false,
        I_4)

    {
        const type = gl.UNSIGNED_SHORT
        const offset = 0
        gl.drawElements(gl.TRIANGLES, vertexCount, type, offset)
        // gl.drawArrays(gl.TRIANGLES, 0, 3)
    }

    // Update the rotation for the next draw
    cubeRotation += deltaTime/4
    shapes.forEach(
        (shape, id, map) => {
            // todo need vector addition to simplify...
            shape.orientation[0] += shape.rotation_speed[0]
            shape.orientation[1] += shape.rotation_speed[1]
            shape.orientation[2] += shape.rotation_speed[2]
            shape.orientation[3] += shape.rotation_speed[3]
            shape.orientation[4] += shape.rotation_speed[4]
            shape.orientation[5] += shape.rotation_speed[5]
        }
    )
    // throw "DEBUG"
}

function preProcessShapes(cam_: Camera, shapes_: Map<number, Shape>): Map<string, Float32Array> {
    // Set up shapes rel to their model, and the camera.  The result is
    // T must be done last.
    let result = new Map()
    let positionedModel, positionM

    let negRot = [-cam_.θ_4d[0], -cam_.θ_4d[1], -cam_.θ_4d[2], -cam_.θ_4d[3], -cam_.θ_4d[4], -cam_.θ_4d[5]]
    const R = rustClone.make_rotator_4d(negRot)

    const negPos = new Vec5([-cam_.position.vals[0], -cam_.position.vals[1], -cam_.position.vals[2],
        -cam_.position.vals[3], 1])
    const T = rustClone.make_translator(negPos)

    shapes_.forEach(
        (shape, id, map) => {
            positionedModel = rustClone.position_shape(shape)
            positionedModel.forEach(
                (node, nid, _map) => {
                    // For cam transform, position first; then rotate.
                    positionM = R.dotM(T)

                    // Map doesn't like tuples/arrays as keys :/
                    result.set([id, nid].join(','), positionM.dotV(node).toGl())
                }
            )
        }
    )
    return result
}

function initBuffers(gl: any, processedShapes: Map<string, Float32Array>) {
    // Create a buffer for the square's positions and color.

    const positionBuffer = gl.createBuffer()

    // Select the positionBuffer as the one to apply buffer
    // operations to from here out.

    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)

    // Now create an array of positions for the square.

    let node
    let positions: number[] = []
    // Set up vertices.
    shapes.forEach(
        (shape, s_id, map) => {
            for (let face of shape.faces_vert) {
                for (let vertex_i of face) {
                    // Map doesn't like tuples as keys :/
                    node = processedShapes.get([s_id, vertex_i].join(',')) as any
                    positions.push(node[0])
                    positions.push(node[1])
                    positions.push(node[2])
                }
            }
        }
    )

    // Now pass the list of positions into WebGL to build the
    // shape. We do this by creating a Float32Array from the
    // JavaScript array, then use it to fill the current buffer.
    gl.bufferData(gl.ARRAY_BUFFER,
        new Float32Array(positions),
        gl.STATIC_DRAW)

    const colorSet6 = [
        [1.0,  1.0,  1.0,  0.5],
        [1.0,  0.0,  0.0,  0.5],
        [0.0,  1.0,  0.0,  0.5],
        [0.0,  0.0,  1.0,  0.5],
        [1.0,  1.0,  0.0,  0.5],
        [1.0,  0.0,  1.0,  0.5],
    ]
    const colorSet5 = [
        [1.0,  1.0,  1.0,  0.5],
        [1.0,  0.0,  0.0,  0.5],
        [0.0,  1.0,  0.0,  0.5],
        [0.0,  0.0,  1.0,  0.5],
        [1.0,  1.0,  0.0,  0.5],
    ]

    const colorSet24 = [
        [1.0,  1.0,  1.0,  0.5],
        [1.0,  0.0,  0.0,  0.5],
        [0.0,  1.0,  0.0,  0.5],
        [0.0,  0.0,  1.0,  0.5],
        [1.0,  1.0,  0.0,  0.5],
        [1.0,  0.0,  1.0,  0.5],

        [1.0,  0.3,  0.4,  0.5],
        [0.3,  0.0,  0.0,  0.5],
        [0.5,  0.5,  1.0,  0.5],
        [0.7,  0.4,  1.0,  0.5],
        [0.8,  1.0,  0.4,  0.5],
        [1.0,  0.2,  1.0,  0.5],

        [0.3,  0.3,  1.0,  0.5],
        [1.0,  0.1,  0.7,  0.5],
        [0.0,  0.2,  0.0,  0.5],
        [0.7,  1.0,  0.5,  0.5],
        [1.0,  .7,  0.2,  0.5],
        [0.8,  0.0,  0.3,  0.5],

        [0.3,  0.2,  1.0,  0.5],
        [1.0,  0.0,  0.0,  0.5],
        [0.4,  0.4,  0.4,  0.5],
        [0.3,  0.0,  0.4,  0.5],
        [0.2,  0.3,  0.0,  0.5],
        [0.3,  0.2,  1.0,  0.5],
    ]

    let faceColors: number[] = []
    shapes.forEach(
        (shape, s_id, map) => {
            if (shape.faces_vert.length === 6) {
                faceColors.push(...colorSet6 as any)
            } else if (shape.faces_vert.length === 5) {
                faceColors.push(...colorSet5 as any)
            } else if (shape.faces_vert.length === 24) {
                faceColors.push(...colorSet24 as any)

            }
        }
    )

    // Convert the array of colors into a table for all the vertices.

    let colors: any = []
    for (let c of faceColors) {
        // Repeat each color four times for the four vertices of the face
        colors = colors.concat(c, c, c, c);
    }

    const colorBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer)
    gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW)

    // Build the element array buffer; this specifies the indices
    // into the vertex arrays for each face's vertices.
    const indexBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer)

    // This array defines each face as two triangles, using the
    // indices into the vertex array to specify each triangle's
    // position.
    let indices: number[] = []
    let indexModifier = 0
    let tri_indices
    shapes.forEach(
        (shape: Shape, s_i, map) => {
            tri_indices = shape.make_tris().map(ind => ind + indexModifier)
            indices.push(...tri_indices)
            indexModifier += shape.numFaceVerts()
        }
    )
    // Now send the element array to GL
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,
        new Uint16Array(indices), gl.STATIC_DRAW)

    return {
        position: positionBuffer,
        color: colorBuffer,
        indices: indexBuffer,
    }
}

export function gl_main(cam_: Camera) {
    // Initialize WebGL rendering.

    const canvas = document.getElementById("glCanvas")
    const gl = (canvas as any).getContext("webgl")

    if (!gl) {
        alert("Unable to initialize WebGL. Your browser or machine may not support it.")
    }

    document.onkeydown = handleKeyDown
    document.onkeyup = handleKeyUp

    // Vertex shader program
    const vsSource = `
        attribute vec4 aVertexPosition;
        attribute vec4 aVertexColor;
    
        uniform mat4 uModelViewMatrix;
        uniform mat4 uProjectionMatrix;
        
        varying lowp vec4 vColor;
    
        void main() {
          gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
          vColor = aVertexColor;
        }
    `

    // Fragment shader program
    const fsSource = `
        varying lowp vec4 vColor;
    
        void main() {
          gl_FragColor = vColor;
        }
    `

    // Initialize a shader program; this is where all the lighting
    // for the vertices and so forth is established.
    const shaderProgram = initShaderProgram(gl, vsSource, fsSource)

    // Collect all the info needed to use the shader program.
    // Look up which attribute our shader program is using
    // for aVertexPosition and look up uniform locations.
    const programInfo = {
        program: shaderProgram,
        attribLocations: {
            vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
            vertexColor: gl.getAttribLocation(shaderProgram, 'aVertexColor'),
        },
        uniformLocations: {
            projectionMatrix: gl.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
            modelViewMatrix: gl.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
        },
    }

    // Here's where we call the routine that builds all the
    // objects we'll be drawing.
    const processedShapes = preProcessShapes(cam, shapes)

    const buffers = initBuffers(gl, processedShapes)

    let vertexCount = 0
    shapes.forEach(
        (shape, id, map) => {
            vertexCount += shape.make_tris().length
        }
    )
    let then = 0
    // Draw the scene repeatedly
    function render(now: number) {
        now *= 0.001;  // convert to seconds
        const deltaTime = now - then;
        then = now;

        drawScene(gl, programInfo, buffers, deltaTime, processedShapes, vertexCount);

        requestAnimationFrame(render)
    }
    requestAnimationFrame(render)
}
