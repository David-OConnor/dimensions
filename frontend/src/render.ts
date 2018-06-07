import {mat4} from 'gl-matrix'
import * as shapeMaker from './shapeMaker'
import * as transforms from './transforms'
import * as state from './state'
import * as util from './util'
import {Camera, ProgramInfo, Shape, Vec5} from './interfaces'

// import {Button, Grid, Row, Col,
//     Form, FormGroup, FormControl, ButtonGroup} from 'react-bootstrap'

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

const τ = 2 * Math.PI

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

let mapFlat = [
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
]

let mapFlat3d = [
    [...mapFlat], [...mapFlat], [...mapFlat], [...mapFlat], [...mapFlat],
    [...mapFlat], [...mapFlat], [...mapFlat], [...mapFlat], [...mapFlat],
]

// todo you could generate these with a loop
const housePositions = [
    [-8., 2, 0., 0.],
    [-8., 2, 12., 0.],
    [-8., 2, 24., 0.],
    [-8., 2, 36., 0.],

    [8., 2, 0., 0.],
    [8., 2, 12., 0.],
    [8., 2, 24., 0.],
    [8., 2, 36., 0.],

    [-8., 2, 0., 4.],
    [-8., 2, 12., 4.],
    [-8., 2, 24., 4.],
    [-8., 2, 36., 4.],

    [8., 2, 0., 4.],
    [8., 2, 12., 4.],
    [8., 2, 24., 4.],
    [8., 2, 36., 4.],
]

const houses = housePositions.map(posit => shapeMaker.make_house([4., 4., 4.],
    new Vec5(posit), [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]) )

function setScene(scene: number, shape: number) {
    document.onkeydown = e => util.handleKeyDown(e, scene)
    if (scene === 0) {  // Single hypercube
        state.setCam(new Camera(
            new Vec5([0., 0., -2., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 5.5,
            4 / 3.,
            1.,
            200.,
            0.1,
            1.0,
        ))

        let selectedShape
        if (shape === 0) {
            selectedShape = shapeMaker.make_hypercube(1, new Vec5([0, 0, 0, 0]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
        } else if (shape === 1) {
            selectedShape = shapeMaker.make_5cell(2, new Vec5([0, 0, 0, 0]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
        } else {
            throw "Oops; a non-existant key was selected. :("
        }
        state.shapes.set(
            0,
            selectedShape
        )

        state.setColorMax(1.5)
    } else if (scene === 1) {  // Terain with shapes
        state.setCam(new Camera(
            new Vec5([0., 2., -3., 0.]),
            [0., -.5, 0., 0., 0., 0.],
            τ / 4.,
            4 / 3.,
            1.,
            200.,
            0.1,
            1.0,
        ))

        let shapeList0 = [
            shapeMaker.make_terrain([20, 20], 10, heightMap, spissMap, new Vec5([0, 0, 0, 0])),

            shapeMaker.make_box([1, 2, 1], new Vec5([-1, 3, 4, 1]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

            shapeMaker.make_rectangular_pyramid([2, 1, 2], new Vec5([-2, 3, 3, -1]),
                [τ/6, τ/3, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

            shapeMaker.make_cube(1, new Vec5([2, 0, 5, 2]),
                [0, 0, 0, 0, 0, 0], [.002, 0, 0, 0, 0, 0]),

            // On ana of other cube.
            shapeMaker.make_cube(1, new Vec5([2, 0, 5, 10]),
                [0, 0, 0, 0, 0, 0], [.002, 0, 0, 0, 0, 0]),

            shapeMaker.make_hypercube(1, new Vec5([3, 3, 3, 0]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, .002, .0005, .001]),

            shapeMaker.make_hypercube(1, new Vec5([-3, 1, 0, 1.5]),
                [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0]),

            // rustClone.make_origin(1, new Vec5([0, 0, 0, 0]), 1,
            //     [0, 0, 0, 0, 0, 0], [0, 0, 0, 0, 0, 0])
        ]

        let shapes = new Map()
        for (let id = 0; id < shapeList0.length; id++) {
            shapes.set(id, shapeList0[id])
        }
        state.setShapes(shapes)

        state.setColorMax(10)
    } else if (scene === 2) {  // Terain with shapes
        state.setCam(new Camera(
            new Vec5([0., 3., -3., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 4.,
            4 / 3.,
            1.,
            1000.,
            0.1,
            1.0,
        ))

        let shapeTownList = [
            shapeMaker.make_terrain([1000, 1000], 10, mapFlat, mapFlat, new Vec5([0, 0, 0, 0])),
            ...houses
        ]

        let shapes = new Map()
        for (let id = 0; id < shapeTownList.length; id++) {
            shapes.set(id, shapeTownList[id])
        }
        state.setShapes(shapes)

        state.setColorMax(30)
    } else if (scene === 3) {  // Hypergrid
        state.setCam(new Camera(
            new Vec5([0., 0., 0., 0.]),
            [0., 0., 0., 0., 0., 0.],
            τ / 4.,
            4 / 3.,
            1.,
            1000.,
            0.1,
            1.0,
        ))

        let mapTest = [
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
            [0, 0, 0, 0],
        ]
        let mapTest3d = [
            [...mapTest], [...mapTest], [...mapTest], [...mapTest]
        ]

        state.setShapes(
            shapeMaker.make_cube_hypergrid([10, 10, 10], 4, mapTest3d, new Vec5([0, 0, 0, 0]))
        )
        state.setColorMax(30)
    } else {
        throw "Nonexistant scene selected."
    }
}

function findColor(dist: number): number[] {
    // produce a color ranging from red to blue, based on how close a point is
    // to the edge.
    let portion_through = Math.abs(dist)  / state.colorMax

    if (portion_through > 1.) {
        portion_through = 1.
    }
    const baseGray = .0
    const colorVal = (baseGray + portion_through * 1. - baseGray)

    if (dist > 0) {
        return [baseGray, baseGray, colorVal, 0.2]  // Blue
    } else {
        return [colorVal, baseGray, baseGray, 0.2]  // Red
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
                   pfBuffers: any,
                   I_4: Float32Array,
                   projectionMatrix: Float32Array,
                   deltaTime: number,
                   processedShapes: Map<string, Vec5>, vertexCount: number) {

    // Clear the canvas before we start drawing on it.
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

    // Create a perspective matrix, a special matrix that is
    // used to simulate the distortion of perspective in a camera.
    // Our field of view is 45 degrees, with a width/height
    // ratio that matches the display size of the canvas
    // and we only want to see objects between 0.1 units
    // and 100 units away from the camera.

    // Set the drawing position to the "identity" point, which is
    // the center of the scene.

    // We've positioned our points rel to their model and the cam already,
    // using 4d transforms; doesn't modify further.

    // Tell WebGL how to pull out the positions from the position
    // buffer into the vertexPosition attribute.
    {
        const numComponents = 3  // pull out 3 values per iteration
        const type = gl.FLOAT    // the data in the buffer is 32bit floats
        const normalize = false  // don't normalize
        const stride = 0         // how many bytes to get from one set of values to the next
                                 // 0 = use type and numComponents above
        const offset = 0         // how many bytes inside the buffer to start from

        gl.bindBuffer(gl.ARRAY_BUFFER, pfBuffers.position)
        gl.vertexAttribPointer(
            programInfo.attribLocations.vertexPosition,
            numComponents,
            type,
            normalize,
            stride,
            offset)
        gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition)
    }

    // Tell WebGL how to pull out the colors from the color buffer
    // into the vertexColor attribute.
    {
        const numComponents = 4
        const type = gl.FLOAT
        const normalize = false
        const stride = 0
        const offset = 0
        gl.bindBuffer(gl.ARRAY_BUFFER, pfBuffers.color)
        gl.vertexAttribPointer(
            programInfo.attribLocations.vertexColor,
            numComponents,
            type,
            normalize,
            stride,
            offset)
        gl.enableVertexAttribArray(
            programInfo.attribLocations.vertexColor)
    }

    // Skybox
    // {
    //     // We'll supply texcoords as floats.
    //     gl.vertexAttribPointer(
    //         programInfo.attribLocations.skyboxTexCoords,
    //         2,
    //         gl.FLOAT,
    //         false,
    //         0,
    //         0
    //     )
    //     gl.enableVertexAttribArray(programInfo.attribLocations.skyboxTexCoords)
    // }

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
}

function perFrameBuffers(gl: any, processedShapes: Map<string, Vec5>) {
    const positionBuffer = gl.createBuffer()
    // Select the positionBuffer as the one to apply buffer
    // operations to from here out.
    gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)

    // Now create an array of positions for the shapes.
    let vertex
    let positions: number[] = []
    // Set up vertices.
    state.shapes.forEach(
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

    gl.bufferData(gl.ARRAY_BUFFER,
        new Float32Array(positions),
        gl.STATIC_DRAW)

    let faceColors: number[][] = []

    state.shapes.forEach(
        (shape, s_id, map) => {
            for (let face of shape.faces_vert) {
                for (let vertI of face) {
                    let zDist = (processedShapes.get([s_id, vertI].join(',')) as any).vals[3] -
                        state.cam.position.vals[3]
                    faceColors.push(findColor(zDist))
                }
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

    return {
        position: positionBuffer,
        color: colorBuffer,
    }
}

function initBuffers(gl: any) {
    // Create a buffer for our shapes' positions and color.
    let vertex
    const sbPositions: any = []
    state.skybox.faces_vert.map(face => face.map(vertex_i => {
        vertex = state.processedSkybox.get([0, vertex_i].join(',')) as any
        sbPositions.push(vertex.vals[0])
        sbPositions.push(vertex.vals[1])
        sbPositions.push(vertex.vals[2])
    }))

    // Now pass the list of positions into WebGL to build the
    // shape. We do this by creating a Float32Array from the
    // JavaScript array, then use it to fill the current buffer.
    const skyboxPositBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, skyboxPositBuffer)
    gl.bufferData(gl.ARRAY_BUFFER,
        new Float32Array(sbPositions),
        gl.STATIC_DRAW)

    // todo skybox texture wip
    // look up where the vertex data needs to go.
    // const positionLocation = gl.getAttribLocation(shaderProgram, "a_position");
    // consttexcoordLocation = gl.getAttribLocation(program, "a_texcoords");
    // Create a buffer for texcoords.

    const skyboxBuffer = gl.createBuffer()
    gl.bindBuffer(gl.ARRAY_BUFFER, skyboxBuffer)
    // Set Texcoords.
    gl.bufferData(
        gl.ARRAY_BUFFER,
        new Float32Array([
            // left column front
            0, 0,
            0, 1,
            1, 0,
            0, 1,
            1, 1,
            1, 0,

            // top rung front
            0, 0,
            0, 1,
            1, 0,
            0, 1,
            1, 1,
            1, 0,
        ]),
        gl.STATIC_DRAW)

    // Build the element array buffer; this specifies the indices
    // into the vertex arrays for each face's vertices.

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
    state.shapes.forEach(
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
        indices: indexBuffer,
        skybox: skyboxBuffer,
        skyboxPosits: skyboxPositBuffer
    }
}

export function gl_main(scene_: number, shape_: number) {
    // Initialize WebGL rendering.
    const canvas = document.getElementById("glCanvas")
    const gl = (canvas as any).getContext("webgl")

    // Overall settings here.

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
    setScene(scene_, shape_)

    if (!gl) {
        alert("Unable to initialize WebGL. Your browser or machine may not support it.")
    }

    document.onkeyup = util.handleKeyUp

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

    // Vertex shader program
    const vsSkybox = `
        attribute vec4 a_position;
        attribute vec2 a_texcoord;
         
        uniform mat4 u_matrix;
         
        varying vec2 v_texcoord;
         
        void main() {
          // Multiply the position by the matrix.
          gl_Position = u_matrix * a_position;
         
          // Pass the texcoord to the fragment shader.
          v_texcoord = a_texcoord;
        }
    `

    // Fragment shader program
    const fsSkybox = `
        precision mediump float;
         
        // Passed in from the vertex shader.
        varying vec2 v_texcoord;
         
        // The texture.
        uniform sampler2D u_texture;
         
        void main() {
           gl_FragColor = texture2D(u_texture, v_texcoord);
        }
    `

    // Initialize a shader program; this is where all the lighting
    // for the vertices and so forth is established.
    const shaderProgram = initShaderProgram(gl, vsSource, fsSource)
    const shaderSkybox = initShaderProgram(gl, vsSkybox, fsSkybox)

    // Collect all the info needed to use the shader program.
    // Look up which attribute our shader program is using
    // for aVertexPosition and look up uniform locations.
    const programInfo = {
        program: shaderProgram,
        attribLocations: {
            vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
            vertexColor: gl.getAttribLocation(shaderProgram, 'aVertexColor'),
            skyboxTexCoords: gl.getAttribLocation(shaderSkybox, 'a_texcoord'),
        },
        uniformLocations: {
            projectionMatrix: gl.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
            modelViewMatrix: gl.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
        },
    }

    let vertexCount = 0
    state.shapes.forEach(
        (shape, id, map) => {
            vertexCount += shape.get_tris().length
        }
    )
    let then = 0
    let pfBuffers
    let buffers = initBuffers(gl)

    const I_4 = new Float32Array([
        1, 0, 0, 0,
        0, 1, 0, 0,
        0, 0, -1, 0,
        0, 0, 0, 1
    ])

    // Note: If we want dynamically-adjustable FOV, we need to move this,
    // or part of it to drawScene.
    let projectionMatrix = mat4.create()
    // note: glmatrix.js always has the first argument
    // as the destination to receive the result.
    mat4.perspective(
        projectionMatrix,
        state.cam.fov,
        state.cam.aspect,
        state.cam.near,
        state.cam.far
    )

    // Draw the scene repeatedly
    function render(now: number) {
        now *= 0.001;  // convert to seconds
        const deltaTime = now - then;
        then = now;

        // Here's where we call the routine that builds all the
        // objects we'll be drawing.
        const processedShapes = transforms.processShapes(state.cam, state.shapes)
        pfBuffers = perFrameBuffers(gl, processedShapes)

        drawScene(gl, programInfo, buffers, pfBuffers, I_4, projectionMatrix,
            deltaTime, processedShapes, vertexCount);

        requestAnimationFrame(render)

        // Update the rotation for the next draw
        state.shapes.forEach(
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
    requestAnimationFrame(render)
}
