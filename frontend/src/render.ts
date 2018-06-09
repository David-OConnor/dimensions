import {mat4} from 'gl-matrix'

import * as setup from './setup'
import * as state from './state'
import * as transforms from './transforms'
import {ProgramInfo, Shape} from './interfaces'
import {dotMM5} from "./util";

// WebGl reference:
// https://developer.mozilla.org/en-US/docs/Web/API/WebGL_API/Tutorial/Adding_2D_content_to_a_WebGL_context

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

function drawScene(
    gl: any,
    programInfo: ProgramInfo,
    uniforms: Map<number, any>,
    buffers: any,
    viewMatrix: Float32Array,
    projectionMatrix: Float32Array,
    shapes: Map<number, Shape>,
    shapeBuffers: any
) {

    // Clear the canvas before we start drawing on it.
    gl.clear(gl.COLOR_BUFFER_BIT | gl.DEPTH_BUFFER_BIT)

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

    shapes.forEach(
        (shape, s_id, map_) => {
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
                const numComponents = 4  // pull out 3 values per iteration
                const type = gl.FLOAT    // the data in the buffer is 32bit floats
                const normalize = false  // don't normalize
                const stride = 0         // how many bytes to get from one set of values to the next
                                         // 0 = use type and numComponents above
                const offset = 0         // how many bytes inside the buffer to start from

                gl.bindBuffer(gl.ARRAY_BUFFER, shapeBuffers.position)
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
                gl.bindBuffer(gl.ARRAY_BUFFER, shapeBuffers.color)
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

            // Tell WebGL which indices to use to index the vertices
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, shapeBuffers.indices)

            // Tell WebGL to use our program when drawing
            gl.useProgram(programInfo.program)

            // Set the shader uniforms
            gl.uniformMatrix4fv(
                programInfo.uniformLocations.projectionMatrix,
                false,
                projectionMatrix)

            let modelViewMat = new Float32Array(25)
            dotMM5(modelViewMat, viewMatrix, uniforms.get(s_id).u_matrix)

            console.log("ModMat", uniforms.get(s_id).u_matrix)
            // console.log("ViewMat", viewMatrix)
            // console.log(modelViewMat, "MVM")

            gl.uniformMatrix4fv(
                programInfo.uniformLocations.modelViewMatrix,
                false,
                modelViewMat)
            {
                const type = gl.UNSIGNED_SHORT
                const offset = 0
                const vertexCount = shape.get_tris().length

                gl.drawElements(gl.TRIANGLES, vertexCount, type, offset)
            }
        }
    )
}
//
// function perFrameBuffers(gl: any, processedShapes: Map<string, Float32Array>) {
//     const positionBuffer = gl.createBuffer()
//     // Select the positionBuffer as the one to apply buffer
//     // operations to from here out.
//     gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)
//
//     // Now create an array of positions for the shapes.
//     let vertex
//     let positions: number[] = []
//     // Set up vertices.
//     state.shapes.forEach(
//         (shape, s_id, map) => {
//             for (let face of shape.faces_vert) {
//                 for (let vertex_i of face) {
//                     // Map doesn't like tuples as keys :/
//                     vertex = processedShapes.get([s_id, vertex_i].join(',')) as any
//                     positions.push(vertex[0])
//                     positions.push(vertex[1])
//                     positions.push(vertex[2])
//                 }
//             }
//         }
//     )
//
//     gl.bufferData(gl.ARRAY_BUFFER,
//         new Float32Array(positions),
//         gl.STATIC_DRAW)
//
//     let faceColors: number[][] = []
//
//     state.shapes.forEach(
//         (shape, s_id, map) => {
//             for (let face of shape.faces_vert) {
//                 for (let vertI of face) {
//                     let zDist = (processedShapes.get([s_id, vertI].join(',')) as any)[3] -
//                         state.cam.position[3]
//                     faceColors.push(findColor(zDist))
//                 }
//             }
//         }
//     )
//
//     // Convert the array of colors into a table for all the vertices.
//     let colors: any = []
//     for (let c of faceColors) {
//         // Repeat each color four times for the four vertices of the face
//         colors = colors.concat(c, c, c, c);
//     }
//
//     const colorBuffer = gl.createBuffer()
//     gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer)
//     gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW)
//
//     return {
//         position: positionBuffer,
//         color: colorBuffer,
//     }
// }

function makeShapeBuffers(gl: any, shapes: Map<number, Shape>): Map<number, any> {
    let positionBuffer, colorBuffer, indexBuffer, positions, faceColors, colors: any,
        indices, tri_indices, indexModifier: number

    let result = new Map()
    shapes.forEach(
        (shape, s_id, map_) => {
            positionBuffer = gl.createBuffer()
            // Select the positionBuffer as the one to apply buffer
            // operations to from here out.
            gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)

            // Now create an array of positions for the shapes.
            positions = []
            // Set up vertices.
            for (let face of shape.faces_vert) {
                for (let vertex_i of face) {
                    positions.push(vertex_i[0])
                    positions.push(vertex_i[1])
                    positions.push(vertex_i[2])
                }
            }

            gl.bufferData(gl.ARRAY_BUFFER,
                new Float32Array(positions),
                gl.STATIC_DRAW)

            faceColors = []

            for (let face of shape.faces_vert) {
                for (let vertI of face) {
                    let zDist = 0  // todo temp
                    faceColors.push(findColor(zDist))
                }
            }

            // Convert the array of colors into a table for all the vertices.
            colors = []
            for (let c of faceColors) {
                // Repeat each color four times for the four vertices of the face
                colors = colors.concat(c, c, c, c);
            }

            colorBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW)

            // todo you don't need to do the index buffer here, since it's only calcualted
            // todo once per shape. Make a map in init_buffers.
            indexBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, indexBuffer)

            // This array defines each face as two triangles, using the
            // indices into the vertex array to specify each triangle's
            // position.
            indices = []
            indexModifier = 0
            tri_indices = shape.get_tris().map(ind => ind + indexModifier)
            indices.push(...tri_indices)
            indexModifier += shape.numFaceVerts()

            // Now send the element array to GL
            gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,
                new Uint16Array(indices), gl.STATIC_DRAW)

            result.set(s_id, {
                position: positionBuffer,
                color: colorBuffer,
                indices: indexBuffer
            })
        }
    )

    return result
}

function initBuffers(gl: any) {
    // Create a buffer for our shapes' positions and color.
    let vertex
    const sbPositions: any = []
    state.skybox.faces_vert.map(face => face.map(vertex_i => {
        vertex = state.processedSkybox.get([0, vertex_i].join(',')) as any
        sbPositions.push(vertex[0])
        sbPositions.push(vertex[1])
        sbPositions.push(vertex[2])
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

export function gl_main(scene: number, subScene: number) {
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
    setup.setScene(scene, subScene)

    if (!gl) {
        alert("Unable to initialize WebGL. Your browser or machine may not support it.")
    }

    document.onkeyup = transforms.handleKeyUp

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

    let then = 0
    let buffers = initBuffers(gl)

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

    let shapeBuffers
    // Draw the scene repeatedly
    function render(now: number) {
        now *= 0.001;  // convert to seconds
        const deltaTime = now - then;
        then = now;

        // Here's where we call the routine that builds all the
        // objects we'll be drawing.
        // const processedShapes = transforms.processShapes(state.cam, state.shapes)
        // pfBuffers = perFrameBuffers(gl, processedShapes)

        shapeBuffers = makeShapeBuffers(gl, state.shapes)

        // const modelMat = transforms.makeModelMat(state.shapes[0])
        // const viewMat = transforms.makeViewMat(state.cam)
        // const modelViewMat = new Float32Array(25)  // todo reuse one of the old ones?
        // Model transform is performed before view transform
        // dotMM5(modelViewMat, viewMat, modelMat)

        // Uniform keys correspond to shape ids.
        let uniforms = new Map()
        state.shapes.forEach(
            (shape, id, map_) => uniforms.set(id, {
                // u_colorMult: [1, 0.5, 0.5, .3],
                // todo model view mat here??
                u_matrix: transforms.makeModelMat(shape)
            })
        )

        const viewMat = transforms.makeViewMat(state.cam)

        drawScene(gl, programInfo, uniforms, buffers,
            viewMat, projectionMatrix,
            state.shapes, shapeBuffers)

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
