import {mat4} from 'gl-matrix'

import * as setup from './setup'
import * as state from './state'
import * as transforms from './transforms'
import {ProgramInfo, Shape} from './interfaces'
import {dotMM5, dotMV5} from "./util";

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
    staticBuffers: any,
    shapeBuffers: any,
    // modelViewMat: Float32Array,
    viewMatrix: Float32Array,
    projectionMatrix: Float32Array,
    shapes: Map<number, Shape>,
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

    let modelMatrix

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
                const numComponents = 3  // pull out 3 values per iteration
                const type = gl.FLOAT    // the data in the buffer is 32bit floats
                const normalize = false  // don't normalize
                const stride = 0         // how many bytes to get from one set of values to the next
                                         // 0 = use type and numComponents above
                const offset = 0         // how many bytes inside the buffer to start from

                // console.log(shapeBuffers.get(s_id).position, "P")
                // console.log(shapeBuffers.get(s_id).indices, "P")

                gl.bindBuffer(gl.ARRAY_BUFFER, shapeBuffers.get(s_id).position)
                gl.vertexAttribPointer(
                    programInfo.attribLocations.vertexPosition,
                    numComponents,
                    type,
                    normalize,
                    stride,
                    offset)
                gl.enableVertexAttribArray(programInfo.attribLocations.vertexPosition)
            }

            {
                const numComponents = 3  // pull out 3 values per iteration
                const type = gl.FLOAT    // the data in the buffer is 32bit floats
                const normalize = false  // don't normalize
                const stride = 0         // how many bytes to get from one set of values to the next
                                         // 0 = use type and numComponents above
                const offset = 0         // how many bytes inside the buffer to start from

                // console.log(shapeBuffers.get(s_id).position, "P")
                // console.log(shapeBuffers.get(s_id).indices, "P")

                gl.bindBuffer(gl.ARRAY_BUFFER, shapeBuffers.get(s_id).quickPosition)
                gl.vertexAttribPointer(
                    programInfo.attribLocations.shapePosition,
                    numComponents,
                    type,
                    normalize,
                    stride,
                    offset)
                gl.enableVertexAttribArray(programInfo.attribLocations.shapePosition)
            }

            {
                const numComponents = 3  // pull out 3 values per iteration
                const type = gl.FLOAT    // the data in the buffer is 32bit floats
                const normalize = false  // don't normalize
                const stride = 0         // how many bytes to get from one set of values to the next
                                         // 0 = use type and numComponents above
                const offset = 0         // how many bytes inside the buffer to start from

                // console.log(shapeBuffers.get(s_id).position, "P")
                // console.log(shapeBuffers.get(s_id).indices, "P")

                gl.bindBuffer(gl.ARRAY_BUFFER, shapeBuffers.get(s_id).camPosition)
                gl.vertexAttribPointer(
                    programInfo.attribLocations.camPosition,
                    numComponents,
                    type,
                    normalize,
                    stride,
                    offset)
                gl.enableVertexAttribArray(programInfo.attribLocations.camPosition)
            }

            // Tell WebGL how to pull out the colors from the color buffer
            // into the vertexColor attribute.
            {
                const numComponents = 4
                const type = gl.FLOAT
                const normalize = false
                const stride = 0
                const offset = 0
                gl.bindBuffer(gl.ARRAY_BUFFER, shapeBuffers.get(s_id).color)
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
            gl.bindBuffer(gl.ELEMENT_ARRAY_BUFFER, staticBuffers.indexBuffers.get(s_id))

            // Tell WebGL to use our program when drawing
            gl.useProgram(programInfo.program)

            // Set the shader uniforms
            gl.uniformMatrix4fv(
                programInfo.uniformLocations.projectionMatrix,
                false,
                projectionMatrix
            )

            modelMatrix = transforms.makeModelMat4(shape)

            // gl.uniformMatrix4fv(
            //     programInfo.uniformLocations.modelViewMatrix,
            //     false,
            //     modelViewMat)

            gl.uniformMatrix4fv(
                programInfo.uniformLocations.viewMatrix,
                false,
                modelMatrix
            )
            gl.uniformMatrix4fv(
                programInfo.uniformLocations.viewMatrix,
                false,
                viewMatrix
            )

            {
                const type = gl.UNSIGNED_SHORT
                const offset = 0
                const vertexCount = shape.get_tris().length

                gl.drawElements(gl.TRIANGLES, vertexCount, type, offset)
            }
        }
    )
}

function makeShapeBuffers(gl: any, shapes: Map<number, Shape>, viewMat: Float32Array): Map<number, any> {
    let positionBuffer, colorBuffer, indexBuffer, positions, faceColors,
        colors: any, quickPositionBuffer, camPositionBuffer
    let result = new Map()
    let modelMat, vertex, transformed, uDist
    let modelViewMat = new Float32Array(25)

    shapes.forEach(
        (shape, s_id, map_) => {
            // modelMat = transforms.makeModelMat(shape)
            // dotMM5(modelViewMat, viewMat, modelMat)

            positionBuffer = gl.createBuffer()
            // Select the positionBuffer as the one to apply buffer
            // operations to from here out.
            gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer)

            // Now create an array of positions for the shapes.
            positions = []
            faceColors = []
            // Set up vertices.
            for (let face of shape.faces_vert) {
                for (let vertex_i of face) {
                    vertex = (shape.nodes.get(vertex_i) as any).a

                    // transformed = new Float32Array(5)
                    // dotMV5(transformed, modelViewMat, vertex)
                    // positions.push(transformed[0])
                    // positions.push(transformed[1])
                    // positions.push(transformed[2])

                    positions.push(vertex[0])
                    positions.push(vertex[1])
                    positions.push(vertex[2])

                    // Now handle colors, based on u-coord dist
                    // uDist = transformed[3] - state.cam.position[3]
                    uDist = 2
                    faceColors.push(findColor(uDist))
                }
            }

            gl.bufferData(gl.ARRAY_BUFFER,
                new Float32Array(positions),
                gl.STATIC_DRAW)

            // Convert the array of colors into a table for all the vertices.
            colors = []
            for (let c of faceColors) {
                // Repeat each color four times for the four vertices of the face
                colors = colors.concat(c, c, c, c);
            }

            colorBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, colorBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(colors), gl.STATIC_DRAW)

            // quick position is the shape's whole position; eg just 4 elements.
            // used for our separate model, view logic.
            quickPositionBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, quickPositionBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, shape.position, gl.STATIC_DRAW)

            camPositionBuffer = gl.createBuffer()
            gl.bindBuffer(gl.ARRAY_BUFFER, camPositionBuffer)
            gl.bufferData(gl.ARRAY_BUFFER, state.cam.position, gl.STATIC_DRAW)

            result.set(s_id, {
                position: positionBuffer,
                color: colorBuffer,
                quickPosition: quickPositionBuffer,
                // todo we don't need to add the cam to the buffer each shape;
                // todo need to rethink how we organize our buffer-creation funcs.
                camPosition: camPositionBuffer
            })
        }
    )

    return result
}

function initBuffers(gl: any, shapes: Map<number, Shape>) {
    // Create a buffer for our shapes' positions and color.
    let vertex
    const sbPositions: any = []
    // state.skybox.faces_vert.map(face => face.map(vertex_i => {
    //     vertex = state.processedSkybox.get([0, vertex_i].join(',')) as any
    //     sbPositions.push(vertex[0])
    //     sbPositions.push(vertex[1])
    //     sbPositions.push(vertex[2])
    // }))

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

    // This array defines each face as two triangles, using the
    // indices into the vertex array to specify each triangle's
    // position.
    let indices: number[] = []
    let indexModifier = 0
    let tri_indices, indexBuffer
    let indexBuffers = new Map()

    shapes.forEach(
        (shape, s_id, map_) => {
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

            indexBuffers.set(s_id, indexBuffer)
            })

    // Now send the element array to GL
    gl.bufferData(gl.ELEMENT_ARRAY_BUFFER,
        new Uint16Array(indices), gl.STATIC_DRAW)

    return {
        indexBuffers: indexBuffers,
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
        
        attribute vec4 aShapePosition;
        attribute vec4 aCamPosition;
        
        uniform mat4 uModelViewMatrix;
        uniform mat4 uProjectionMatrix;
                
        // Trying to sep model and view matrices, so we can pass 4x4,
        // non-homogenous matrices to the shader.
        uniform mat4 uModelMatrix;
        uniform mat4 uViewMatrix;
               
        varying lowp vec4 vColor;
        
        // Intermediate variable we use for applying our sequence of custom transforms.
        // We split this up so we can operate on matrices and vectors of size 4,
        // since WebGL doesn't support 5x5 (homogenous) matrices.
        vec4 positionedPt;
    
        void main() {
          // gl_Position = uProjectionMatrix * uModelViewMatrix * aVertexPosition;
          
          // For model transform, position after the transform
          positionedPt = (uModelMatrix * aVertexPosition) + aShapePosition;
          // for view transform, position first.
          positionedPt = uViewMatrix * (aVertexPosition - aCamPosition);
          gl_Position = uProjectionMatrix * positionedPt;
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
            shapePosition: gl.getAttribLocation(shaderProgram, 'aShapePosition'),
            camPosition: gl.getAttribLocation(shaderProgram, 'aCamPosition'),
            vertexColor: gl.getAttribLocation(shaderProgram, 'aVertexColor'),
            skyboxTexCoords: gl.getAttribLocation(shaderSkybox, 'a_texcoord'),
        },
        uniformLocations: {
            projectionMatrix: gl.getUniformLocation(shaderProgram, 'uProjectionMatrix'),
            modelViewMatrix: gl.getUniformLocation(shaderProgram, 'uModelViewMatrix'),
            modelMatrix: gl.getUniformLocation(shaderProgram, 'uModelMatrix'),
            ViewMatrix: gl.getUniformLocation(shaderProgram, 'uViewMatrix'),
        },
    }

    let then = 0
    let staticBuffers = initBuffers(gl, state.shapes)

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

    // const I_4 = mat4.create()

    // Draw the scene repeatedly
    function render(now: number) {
        now *= 0.001;  // convert to seconds
        const deltaTime = now - then;
        then = now;

        // Here's where we call the routine that builds all the
        // objects we'll be drawing.
        // const processedShapes = transforms.processShapes(state.cam, state.shapes)
        // pfBuffers = perFrameBuffers(gl, processedShapes)

        // const viewMatrix = transforms.makeViewMat(state.cam)
        const viewMatrix = transforms.makeViewMat4(state.cam)
        const shapeBuffers = makeShapeBuffers(gl, state.shapes, viewMatrix)

        // drawScene(gl, programInfo, staticBuffers, shapeBuffers,
        //     I_4, projectionMatrix,
        //     state.shapes)
        //
        drawScene(gl, programInfo, staticBuffers, shapeBuffers,
            viewMatrix, projectionMatrix,
            state.shapes)

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
