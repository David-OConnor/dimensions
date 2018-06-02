import {mat4, glMatrix} from 'gl-matrix'

// import * as Rust from './unitalgebra'

// import {Button, Grid, Row, Col,
//     Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

import * as rustClone from './rust_clone'
import {ProgramInfo, Shape, Camera, Vec5, Array5} from './interfaces'
import {position_shape} from "./rust_clone";

let cubeRotation = 0.0;

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
                   deltaTime: number, cam: Camera, shapes: Map<number, Shape>,
                   vertexCount: number) {
    gl.clearColor(0.0, 0.0, 0.0, 1.0)  // Clear to black, fully opaque
    gl.clearDepth(1.0)                 // Clear everything
    gl.enable(gl.DEPTH_TEST)           // Enable depth testing
    gl.depthFunc(gl.LEQUAL)            // Near things obscure far things

    // Clear the canvas before we start drawing on it.
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

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
    // const modelViewMatrix = new Float32Array([
    //     1, 0, 0, 0,
    //     0, 1, 0, 0,
    //     0, 0, 1, 0,
    //     0, 1, cam.position.vals[2], 1
    // ])

    const modelViewMatrix = new Float32Array([
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, 1, 0,
        0, 1, -7, 1
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
        modelViewMatrix)

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

function preprocessShapes(cam: Camera, shapes: Map<number, Shape>): Map<string, Float32Array> {
    // Set up shapes rel to their model, and the camera.  The result is
    // T must be done last.
    let result = new Map()
    let positionedModel, positionM

    const R = rustClone.make_rotator_4d(cam.θ_4d)
    const T = rustClone.make_translator(cam.position)
    shapes.forEach(
        (shape, id, map) => {
            positionedModel = rustClone.position_shape(shape)
            // console.log("PM", positionedModel)
            positionedModel.forEach(
                (node, nid, _map) => {
                    // For cam transform, position first; then rotate.
                    positionM = T.dotM(R)

                    // Map doesn't like tuples/arrays as keys :/
                    result.set([id, nid].join(','), positionM.dotV(node).toGl())
                }
            )
        }
    )
    return result
}

function initBuffers(gl: any, shapes: Map<number, Shape>,
                     processedShapes: Map<string, Float32Array>) {
    // Create a buffer for the square's positions and color.

    const positionBuffer = gl.createBuffer()

    // Select the positionBuffer as the one to apply buffer
    // operations to from here out.

    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)

    // Now create an array of positions for the square.

    let node0
    let positions: number[] = []
    console.log(processedShapes, "PS")
    // Set up vertices.
    shapes.forEach(
        (shape, s_id, map) => {
            for (let face of shape.faces) {
                for (let edge of face.edges) {
                    // Map doesn't like tuples as keys :/
                    node0 = processedShapes.get([s_id, edge.node0].join(',')) as any
                    // node0 = (shapes.get(s_id) as any).nodes.get(edge.node0)
                    // node0 = node0.a.vals
                    positions.push(node0[0])
                    positions.push(node0[1])
                    positions.push(node0[2])

                    // GL likes nodes per face, while we iterate over
                    // edges in our model's faces... There's duplication either way.
                    // By only including the first node of each edge, do we
                    // get what we want?

                    // positions.push(node1[0])
                    // positions.push(node1[1])
                    // positions.push(node1[2])
                }
            }
        }
    )

    console.log(positions, "P")

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

    let faceColors: number[] = []
    shapes.forEach(
        (shape, s_id, map) => {
            if (shape.faces.length === 6) {
                faceColors.push(...colorSet6 as any)
            } else if (shape.faces.length === 5) {
                faceColors.push(...colorSet5 as any)
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
        (shape, s_i, map) => {
            tri_indices = shape.tri_indices.map(ind => ind + indexModifier)
            indices.push(...tri_indices)
            indexModifier += shape.nodes.size
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

export function gl_main(cam: Camera, shapes: Map<number, Shape>) {
    // Initialize WebGL rendering.

    const canvas = document.getElementById("glCanvas")
    const gl = (canvas as any).getContext("webgl")

    if (!gl) {
        alert("Unable to initialize WebGL. Your browser or machine may not support it.")
    }

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
    const processedShapes = preprocessShapes(cam, shapes)

    const buffers = initBuffers(gl, shapes, processedShapes)

    let vertexCount = 0
    shapes.forEach(
        (shape, id, map) => {
            vertexCount += shape.tri_indices.length
        }
    )
    let then = 0
    // Draw the scene repeatedly
    function render(now: number) {
        now *= 0.001;  // convert to seconds
        const deltaTime = now - then;
        then = now;

        drawScene(gl, programInfo, buffers, deltaTime, cam, shapes, vertexCount);

        requestAnimationFrame(render)
    }
    requestAnimationFrame(render)
}
