import {mat4} from 'gl-matrix'
import * as shapeMaker from './shapeMaker'
import * as transforms from './transforms'
import {Camera, ProgramInfo, Shape, Vec5} from './interfaces'

// import {Button, Grid, Row, Col,
//     Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

// todo global shapes and cam for now
const τ = 2 * Math.PI

let colorMax = 15  // At this z distance, our blue/red shift fully saturated.
let currentlyPressedKeys = {}
const moveSensitivity = .1
const rotateSensitivity = .03

function moveCam(unitVec: number[]) {
    // Modifies the global camera
    const direc = transforms.make_rotator(cam.θ).dotV(new Vec5(unitVec))
    const amount = direc.mul(moveSensitivity)
    cam.position = cam.position.add(amount)
}

function handleKeyDown(event: any) {
    currentlyPressedKeys[event.keyCode] = true
    switch(event.keyCode) {
        case 87:  // w
            if (scene === 0) {
                console.log()
            } else {
                moveCam([0, 0, 1, 0])
            }
            event.preventDefault()
            break
        case 83:  // s
            if (scene === 0) {
                console.log()
            } else {
                moveCam([0, 0, -1, 0])
            }
            break
        case 68:  // d
            if (scene === 0) {
                console.log()
            } else {
                moveCam([1, 0, 0, 0])
            }
            break
        case 65:  // a
            if (scene === 0) {
                console.log()
            } else {
                moveCam([-1, 0, 0, 0])
            }
            break
        case 32:  // Space
            if (scene === 0) {
                console.log()
            } else {
                moveCam([0, 1, 0, 0])
            }
            event.preventDefault()
            break
        case 67:  // c
            if (scene === 0) {
                console.log()
            } else {
                moveCam([0, -1, 0, 0])
            }
            break
        case 17:  // Control
            if (scene === 0) {
                console.log()
            } else {
                moveCam([0, -1, 0, 0])
            }
            break
        case 82:  // r
            if (scene === 0) {
                console.log()
            } else {
                moveCam([0, 0, 0, 1])
            }
            break
        case 70:  // f
            if (scene === 0) {
                console.log()
            } else {
                moveCam([0, 0, 0, -1])
            }
            break
        // todo add deltaTime!
        case 38:  // Up
            if (scene === 0) {
                shapes.get(0).orientation[1] -=rotateSensitivity
            } else {
                cam.θ[1] +=rotateSensitivity
            }
            event.preventDefault();
            break
        case 40:  // Down
            if (scene === 0) {
                shapes.get(0).orientation[1] +=rotateSensitivity
            } else {
                cam.θ[1] -=rotateSensitivity
            }
            event.preventDefault();
            break
        case 39:  // Right

            if (scene === 0) {
                shapes.get(0).orientation[2] +=rotateSensitivity
            } else {
                cam.θ[2] -=rotateSensitivity
            }
            event.preventDefault();
            break
        case 37:  // Left
            if (scene === 0) {
                shapes.get(0).orientation[2] -=rotateSensitivity
            } else {
                cam.θ[2] +=rotateSensitivity
                event.preventDefault();
            }
            break
        case 69:  // E
            if (scene === 0) {
                shapes.get(0).orientation[0] +=rotateSensitivity
            } else {
                cam.θ[0] +=rotateSensitivity
            }
            break
        case 81:  // Q
            if (scene === 0) {
                shapes.get(0).orientation[0] -=rotateSensitivity

            } else {
                cam.θ[0] -=rotateSensitivity
            }
            break
        case 45:  // Ins
            if (scene === 0) {
                shapes.get(0).orientation[3] +=rotateSensitivity
            } else {
                cam.θ[3] +=rotateSensitivity
            }
            break
        case 46:  // Del
            if (scene === 0) {
                shapes.get(0).orientation[3] -=rotateSensitivity
            } else {
                cam.θ[3] -=rotateSensitivity
            }
            event.preventDefault();
            break
        case 36:  // Home
            if (scene === 0) {
                shapes.get(0).orientation[4] +=rotateSensitivity
            } else {
                cam.θ[4] +=rotateSensitivity
            }
            event.preventDefault();
            break
        case 35:  // End
            if (scene === 0) {
                shapes.get(0).orientation[4] -=rotateSensitivity
            } else {
                cam.θ[4] -=rotateSensitivity
            }
            event.preventDefault();
            break
        case 33:  // Pgup
            if (scene === 0) {
                shapes.get(0).orientation[5] +=rotateSensitivity
            } else {
                cam.θ[5] +=rotateSensitivity
            }
            event.preventDefault();
            break
        case 34:  // Pgdn
            if (scene === 0) {
                shapes.get(0).orientation[5] -=rotateSensitivity
            } else {
                cam.θ[5] -=rotateSensitivity
            }
            event.preventDefault();
            break
        default:
            break
    }
}

function handleKeyUp(event: any) {
    currentlyPressedKeys[event.keyCode] = false;
}

let heightMap = [
    [1.3, 1.3, 0, 0, 0, 0, 0, 0, 1.2, 1.2],
    [1.3, 1.2, 0, 0, 0, 0, 0, 1.1, 1.2, 1.2],
    [0, 1.2, 1.2, 0, 0, 0, 0, 1.1, 1.2, 0],
    [0, 1.1, 0, 0, 0, 0, 0, 0, 1.1, 1.2],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1.2],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 1.2],
    [0, 0, 0, 1.1, 0, 0, 0, 0, 1.1, 1.2],
    [0, 1.1, 0, 0, 0, 0, 0, 0, 1.2, 1.2],
    [0, 1.1, 1.1, 1.1, 1.1, 0, 1.3, 1.3, 2.4, 2.2],
    [0, 1.1, 1.1, 1.1, 1.2, 1.3, 1.3, 1.4, 2.4, 2.8]
]

let spissMap = [
    [5, 4, 2, 1.2, 0, 0, 1, 1, 2, 2.5],
    [5, 3, 2.5, 1.2, 0, 0, 0, 0, 2, 2.5],
    [5, 4, 2, 1, 2, 0, 0, 0, 2, 2.5],
    [4, 3, 2, 1, 0, 0, 0, 0, 2, 2.5],
    [4, 4, 3, 1, 0, 1, 0, 0, 2, 2.5],
    [6, 4, 3, 3.5, 1, 0, 0, 0, 2, 2.5],
    [6, 5.5, 5, 3.5, 2, 0, 0, 0, 2, 2.5],
    [6, 5.5, 5.5, 4, 2, 0, 0, 1, 2, 2.5],
    [6, 6, 6, 3.5, 2, 0, 0, 0, 2, 1.5],
    [7, 7, 7, 3.5, 2, 0, 0, 0, 2, 2.5],
]

let shape_list = [
    shapeMaker.make_terrain([20, 20], [10, 10], heightMap, spissMap, new Vec5([0, 0, 0, 0])),

    shapeMaker.make_box([1, 2, 1], new Vec5([-1, 3, 4, 0]),
        [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

    shapeMaker.make_house([1, 1, 1], new Vec5([4, -1, 0, 5]),
        [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

    shapeMaker.make_rectangular_pyramid([3, 2, 3], new Vec5([-2, -4, 3, 5]),
        [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

    shapeMaker.make_cube(1, new Vec5([2, 0, 5, 0]),
        [0, 0, 0, 0, 0, 0], [.002, 0, 0, 0, 0, 0]),

    // On ana of other cube.
    shapeMaker.make_cube(1, new Vec5([2, 0, 5, 10]),
        [0, 0, 0, 0, 0, 0], [.002, 0, 0, 0, 0, 0]),

    shapeMaker.make_hypercube(1, new Vec5([3, 3, 3, 0]),
        [0, 0, 0, 0, 0, 0], [0, 0, 0, .005, .005, .004]),

    shapeMaker.make_hypercube(1, new Vec5([-3, 0, 3, 0]),
        [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

    // rustClone.make_origin(1, new Vec5([0, 0, 0, 0]), 1,
    //     [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
]

// let skybox = shapeMaker.make_skybox(100, cam.position)

let scene = 0

let shapes = new Map()

const defaultCam = new Camera (
    new Vec5([0., 0., 0., 0.]),
    [0., 0., 0., 0., 0., 0.],
    τ / 4.,
    4 / 3.,
    1.,
    100.,
    0.1,
    1.0,
)
let cam = defaultCam

function setScene(scene_: number) {
    if (scene_ === 0) {  // Single hypercube
        cam = new Camera (
            new Vec5([0., 0., -2., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 5.5,
            4 / 3.,
            1.,
            100.,
            0.1,
            1.0,
        )

        shapes = new Map()
        shapes.set(
            0,
            shapeMaker.make_hypercube(1, new Vec5([0, 0, 0, 0]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
        )

        colorMax = 1.5;
    } else if (scene_ === 1) {  // Terain with shapes
        cam = new Camera (
            new Vec5([0., 0., -3., 0.]),
            [0., -.5, 0., 0., 0., 0.],
            τ / 4.,
            4 / 3.,
            1.,
            100.,
            0.1,
            1.0,
        );

        shapes = new Map()
        for (let id=0; id < shape_list.length; id++) {
            shapes.set(id, shape_list[id])
        }

        colorMax = 15
    } else {
        cam = defaultCam
        shapes = new Map()
        colorMax = 15
    }
}

function findColor(dist: number): number[] {
    // produce a color ranging from red to blue, based on how close a point is
    // to the edge.
    let portion_through = Math.abs(dist)  / colorMax

    if (portion_through > 1.) {
        portion_through = 1.
    }
    const baseGray = .0
    const colorVal = (baseGray + portion_through * 1. - baseGray)

    if (dist > 0) {
        return [baseGray, baseGray, colorVal, .3]  // Blue
    } else {
        return [colorVal, baseGray, baseGray, .3]  // Red
    }
}

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
                   processedShapes: Map<string, Vec5>, vertexCount: number) {
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
}

function initBuffers(gl: any, processedShapes: Map<string, Vec5>) {
    // Create a buffer for the square's positions and color.

    const positionBuffer = gl.createBuffer()

    // Select the positionBuffer as the one to apply buffer
    // operations to from here out.

    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)

    // Now create an array of positions for the square.

    let vertex
    let positions: number[] = []
    // Set up vertices.
    shapes.forEach(
        (shape, s_id, map) => {
            for (let face of shape.faces_vert) {
                for (let vertex_i of face) {
                    // Map doesn't like tuples as keys :/
                    vertex = processedShapes.get([s_id, vertex_i].join(',')) as any
                    positions.push(vertex.vals[0])
                    positions.push(vertex.vals[1])
                    positions.push(vertex.vals[2])
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

    let faceColors: number[][] = []

    shapes.forEach(
        (shape, s_id, map) => {
            for (let face of shape.faces_vert) {
                for (let vertI of face) {
                    let zDist = (processedShapes.get([s_id, vertI].join(',')) as any).vals[3] - cam.position.vals[3]
                    faceColors.push(findColor(zDist))
                }
            }

            // if (shape.faces_vert.length === 6) {
            //     faceColors.push(...colorSet6)
            // } else if (shape.faces_vert.length === 5) {
            //     faceColors.push(...colorSet5)
            // } else if (shape.faces_vert.length === 24) {
            //     faceColors.push(...colorSet24)
            // }
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
            tri_indices = shape.get_tris().map(ind => ind + indexModifier)
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

export function gl_main(scene_: number) {
    // Initialize WebGL rendering.
    const canvas = document.getElementById("glCanvas")
    const gl = (canvas as any).getContext("webgl")

    setScene(scene_)

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

    let vertexCount = 0
    shapes.forEach(
        (shape, id, map) => {
            vertexCount += shape.get_tris().length
        }
    )
    let then = 0
    // Draw the scene repeatedly
    function render(now: number) {
        now *= 0.001;  // convert to seconds
        const deltaTime = now - then;
        then = now;

        // Here's where we call the routine that builds all the
        // objects we'll be drawing.
        const processedShapes = transforms.processShapes(cam, shapes)
        const buffers = initBuffers(gl, processedShapes)

        drawScene(gl, programInfo, buffers, deltaTime, processedShapes, vertexCount);

        requestAnimationFrame(render)
    }
    requestAnimationFrame(render)
}
